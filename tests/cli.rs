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
