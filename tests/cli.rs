use std::{fs, process::Command};

fn binary() -> Command {
    Command::new(env!("CARGO_BIN_EXE_api-example-linter"))
}

#[test]
fn documented_markdown_command_passes() {
    let output = binary()
        .args([
            "check",
            "fixtures/valid.md",
            "--spec",
            "fixtures/openapi.yaml",
            "--operation",
            "createPet",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("2 example(s) checked"));
}

#[test]
fn invalid_example_returns_one_and_json_findings() {
    let output = binary()
        .args([
            "check",
            "fixtures/invalid.md",
            "--spec",
            "fixtures/openapi.yaml",
            "--schema",
            "Pet",
            "--format",
            "json",
        ])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["summary"]["failed"], 1);
    assert_eq!(report["diagnostics"][0]["line"], 4);
}

#[test]
fn conventional_curl_data_equals_is_discovered_and_validated() {
    let fixture = tempfile::Builder::new().suffix(".md").tempfile().unwrap();
    fs::write(
        fixture.path(),
        "```curl operation=createPet direction=request\ncurl --data='{\"name\":\"Ada\",\"tag\":\"rescue\"}' https://example.invalid/pets\n```\n",
    )
    .unwrap();
    let output = binary()
        .args([
            "check",
            fixture.path().to_str().unwrap(),
            "--spec",
            "fixtures/openapi.yaml",
            "--operation",
            "createPet",
            "--direction",
            "request",
            "--format",
            "json",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["summary"]["discovered"], 1);
    assert_eq!(report["summary"]["passed"], 1);
    assert!(report["diagnostics"].as_array().unwrap().is_empty());
}

#[test]
fn malformed_example_does_not_add_a_redundant_no_examples_finding() {
    let fixture = tempfile::Builder::new().suffix(".md").tempfile().unwrap();
    fs::write(fixture.path(), "```json\n{\"name\":\n```\n").unwrap();
    let output = binary()
        .args([
            "check",
            fixture.path().to_str().unwrap(),
            "--format",
            "json",
        ])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let codes = report["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .map(|diagnostic| diagnostic["code"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(codes, ["INVALID_EXAMPLE"]);
}

#[test]
fn embedded_openapi_example_uses_implicit_spec() {
    let output = binary()
        .args([
            "check",
            "fixtures/openapi.yaml",
            "--operation",
            "createPet",
            "--direction",
            "request",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("1 example(s) checked"));
}

#[test]
fn configuration_errors_return_two() {
    let output = binary()
        .args(["check", "missing.md", "--format", "json"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("CONFIGURATION_ERROR"));
}

#[test]
fn demo_uses_real_linter_and_removes_its_temporary_workspace() {
    let current = tempfile::tempdir().unwrap();
    let sentinel = current.path().join("real-user-file.txt");
    fs::write(&sentinel, "keep me").unwrap();
    fs::write(
        current.path().join(".api-example-linter.json"),
        r#"{"inputs":["missing-real-path"]}"#,
    )
    .unwrap();

    let output = binary()
        .current_dir(current.path())
        .arg("demo")
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("$ api-example-linter demo"));
    assert!(stdout.contains("SCHEMA_MISMATCH"));
    assert!(stdout.contains("2 example(s) checked · 1 passed · 1 failed"));
    assert_eq!(fs::read_to_string(&sentinel).unwrap(), "keep me");
    let workspace = stdout
        .lines()
        .find_map(|line| line.strip_prefix("Temporary folder: "))
        .expect("demo prints its temporary folder");
    assert!(!std::path::Path::new(workspace).exists());
}
