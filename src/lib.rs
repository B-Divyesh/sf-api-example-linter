//! Core extraction, OpenAPI mapping, validation, and report rendering.
//!
//! The CLI is the supported interface, but output rendering is deterministic:
//! ```
//! use api_example_linter::{OutputFormat, Report, render_report};
//! let json = render_report(&Report::default(), OutputFormat::Json);
//! assert!(json.contains("\"diagnostics\""));
//! ```
#![allow(clippy::collapsible_if)]

use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};
use std::collections::HashSet;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::path::{Path, PathBuf};
use std::time::Duration;
use url::Url;
use walkdir::WalkDir;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    Request,
    Response,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    #[default]
    Auto,
    Text,
    Json,
    Github,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct FileConfig {
    pub spec: Option<PathBuf>,
    #[serde(default)]
    pub inputs: Vec<PathBuf>,
    pub operation: Option<String>,
    pub schema: Option<String>,
    pub direction: Option<Direction>,
    pub format: Option<OutputFormat>,
    pub mock_base_url: Option<String>,
    #[serde(default)]
    pub allow_hosts: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct CheckOptions {
    pub inputs: Vec<PathBuf>,
    pub spec: Option<PathBuf>,
    pub operation: Option<String>,
    pub schema: Option<String>,
    pub direction: Option<Direction>,
    pub format: OutputFormat,
    pub mock_base_url: Option<String>,
    pub allow_hosts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Debug, Clone, Serialize)]
pub struct Diagnostic {
    pub severity: Severity,
    pub code: String,
    pub message: String,
    pub file: String,
    pub line: usize,
    pub column: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct Summary {
    pub discovered: usize,
    pub passed: usize,
    pub failed: usize,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct Report {
    pub summary: Summary,
    pub diagnostics: Vec<Diagnostic>,
}

#[derive(Debug)]
pub struct AppError {
    pub message: String,
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}
impl std::error::Error for AppError {}
impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Mapping {
    operation: Option<String>,
    schema: Option<String>,
    direction: Option<Direction>,
}

#[derive(Debug, Clone)]
struct Example {
    value: Value,
    file: String,
    line: usize,
    mapping: Mapping,
}

#[derive(Debug, Clone)]
struct Operation<'a> {
    method: &'a str,
    path: &'a str,
    value: &'a Value,
}

pub fn load_config(path: &Path) -> Result<FileConfig, AppError> {
    let text = fs::read_to_string(path).map_err(|e| AppError {
        message: format!("cannot read {}: {e}", path.display()),
    })?;
    serde_json::from_str(&text).map_err(|e| AppError {
        message: format!("invalid {}: {e}", path.display()),
    })
}

pub fn write_starter_config(path: &Path) -> Result<(), AppError> {
    if path.exists() {
        return Err(AppError {
            message: format!("{} already exists; nothing was overwritten", path.display()),
        });
    }
    fs::write(
        path,
        "{\n  \"spec\": \"openapi.yaml\",\n  \"inputs\": [\"docs\"],\n  \"operation\": \"createPet\",\n  \"direction\": \"request\",\n  \"format\": \"auto\"\n}\n",
    )?;
    Ok(())
}

pub fn check(options: &CheckOptions) -> Result<Report, AppError> {
    if options.operation.is_some() && options.schema.is_some() {
        return Err(AppError {
            message: "choose either --operation or --schema, not both".into(),
        });
    }
    let implicit_spec =
        if options.spec.is_none() && options.inputs.len() == 1 && options.inputs[0].is_file() {
            let ext = options.inputs[0]
                .extension()
                .and_then(|v| v.to_str())
                .unwrap_or("");
            if ["json", "yaml", "yml"].contains(&ext.to_ascii_lowercase().as_str()) {
                Some(options.inputs[0].clone())
            } else {
                None
            }
        } else {
            None
        };
    let spec_path = options.spec.as_ref().or(implicit_spec.as_ref());
    let spec = match spec_path {
        Some(path) => Some(load_document(path)?),
        None => None,
    };
    if (options.operation.is_some() || options.schema.is_some()) && spec.is_none() {
        return Err(AppError {
            message: "--operation and --schema require --spec (or a config spec)".into(),
        });
    }
    if let (Some(root), Some(name)) = (&spec, &options.schema) {
        resolve_named_schema(root, name)?;
    }
    if let (Some(root), Some(id)) = (&spec, &options.operation) {
        find_operation(root, id)?;
    }
    let paths = collect_files(&options.inputs)?;
    let mut examples = Vec::new();
    let mut diagnostics = Vec::new();
    for path in paths {
        let ext = path
            .extension()
            .and_then(|v| v.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        if ext == "md" || ext == "mdx" || ext == "markdown" {
            extract_markdown(&path, &mut examples, &mut diagnostics)?;
        } else if ["yaml", "yml", "json"].contains(&ext.as_str()) {
            let is_selected_spec = spec_path.is_some_and(|p| same_path(p, &path));
            if is_selected_spec || options.spec.is_none() {
                let document = load_document(&path)?;
                extract_openapi_examples(&path, &document, options, &mut examples)?;
            }
        }
    }
    let mut report = Report::default();
    report.diagnostics.append(&mut diagnostics);
    for example in examples {
        report.summary.discovered += 1;
        let mapping = Mapping {
            operation: example
                .mapping
                .operation
                .clone()
                .or_else(|| options.operation.clone()),
            schema: example
                .mapping
                .schema
                .clone()
                .or_else(|| options.schema.clone()),
            direction: example.mapping.direction.or(options.direction),
        };
        let schema_result = schema_for_mapping(spec.as_ref(), &mapping);
        let before = report.diagnostics.len();
        match schema_result {
            Ok((schema, operation)) => {
                let mut issues = Vec::new();
                validate_value(
                    &example.value,
                    schema,
                    spec.as_ref().unwrap_or(schema),
                    "$",
                    &mut issues,
                    0,
                );
                for (pointer, message) in issues {
                    report.diagnostics.push(Diagnostic {
                        severity: Severity::Error,
                        code: "SCHEMA_MISMATCH".into(),
                        message,
                        file: example.file.clone(),
                        line: example.line,
                        column: 1,
                        pointer: Some(pointer),
                    });
                }
                if report.diagnostics.len() == before {
                    if let (Some(base), Some(op)) = (&options.mock_base_url, operation) {
                        if mapping.direction.unwrap_or(Direction::Request) == Direction::Request {
                            if let Err(message) = mock_check(
                                base,
                                &options.allow_hosts,
                                op.method,
                                op.path,
                                &example.value,
                            ) {
                                report.diagnostics.push(Diagnostic {
                                    severity: Severity::Error,
                                    code: "MOCK_CHECK_FAILED".into(),
                                    message,
                                    file: example.file.clone(),
                                    line: example.line,
                                    column: 1,
                                    pointer: None,
                                });
                            }
                        }
                    }
                }
            }
            Err(message) => report.diagnostics.push(Diagnostic {
                severity: Severity::Error,
                code: "EXAMPLE_MAPPING".into(),
                message,
                file: example.file.clone(),
                line: example.line,
                column: 1,
                pointer: None,
            }),
        }
        if report.diagnostics.len() == before {
            report.summary.passed += 1;
        } else {
            report.summary.failed += 1;
        }
    }
    if report.summary.discovered == 0 {
        report.diagnostics.push(Diagnostic {
            severity: Severity::Error,
            code: "NO_EXAMPLES".into(),
            message: "no JSON or curl examples were found; add a fenced block or OpenAPI example"
                .into(),
            file: options
                .inputs
                .first()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| ".".into()),
            line: 1,
            column: 1,
            pointer: None,
        });
    }
    if report.summary.failed == 0 && !report.diagnostics.is_empty() {
        report.summary.failed = 1;
    }
    Ok(report)
}

fn same_path(a: &Path, b: &Path) -> bool {
    match (fs::canonicalize(a), fs::canonicalize(b)) {
        (Ok(a), Ok(b)) => a == b,
        _ => a == b,
    }
}

fn collect_files(inputs: &[PathBuf]) -> Result<Vec<PathBuf>, AppError> {
    if inputs.is_empty() {
        return Err(AppError { message: "no inputs configured; pass a Markdown/OpenAPI path or add inputs to .api-example-linter.json".into() });
    }
    let mut files = Vec::new();
    for input in inputs {
        if !input.exists() {
            return Err(AppError {
                message: format!("input does not exist: {}", input.display()),
            });
        }
        if input.is_file() {
            files.push(input.clone());
            continue;
        }
        for entry in WalkDir::new(input)
            .follow_links(false)
            .into_iter()
            .filter_map(Result::ok)
        {
            if entry.file_type().is_file() {
                let ext = entry
                    .path()
                    .extension()
                    .and_then(|v| v.to_str())
                    .unwrap_or("")
                    .to_ascii_lowercase();
                if ["md", "mdx", "markdown", "yaml", "yml", "json"].contains(&ext.as_str()) {
                    files.push(entry.path().to_path_buf());
                }
            }
        }
    }
    files.sort();
    files.dedup();
    Ok(files)
}

fn load_document(path: &Path) -> Result<Value, AppError> {
    let text = fs::read_to_string(path).map_err(|e| AppError {
        message: format!("cannot read {}: {e}", path.display()),
    })?;
    if path
        .extension()
        .and_then(|v| v.to_str())
        .is_some_and(|v| v.eq_ignore_ascii_case("json"))
    {
        serde_json::from_str(&text).map_err(|e| AppError {
            message: format!("invalid JSON in {}: {e}", path.display()),
        })
    } else {
        serde_yaml::from_str(&text).map_err(|e| AppError {
            message: format!("invalid YAML in {}: {e}", path.display()),
        })
    }
}

fn extract_markdown(
    path: &Path,
    examples: &mut Vec<Example>,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<(), AppError> {
    let text = fs::read_to_string(path).map_err(|e| AppError {
        message: format!("cannot read {}: {e}", path.display()),
    })?;
    let mut fence: Option<(String, Mapping, usize, Vec<&str>)> = None;
    for (index, line) in text.lines().enumerate() {
        let number = index + 1;
        if let Some((lang, mapping, start, body)) = &mut fence {
            if line.trim_start().starts_with("```") {
                let raw = body.join("\n");
                if lang == "curl-shell" && !raw.trim_start().starts_with("curl ") {
                    fence = None;
                    continue;
                }
                let parsed = if lang == "json" || lang == "jsonc" {
                    parse_json_example(&raw)
                } else {
                    parse_curl_body(&raw)
                };
                match parsed {
                    Ok(value) => examples.push(Example {
                        value,
                        file: path.display().to_string(),
                        line: *start,
                        mapping: mapping.clone(),
                    }),
                    Err(message) => diagnostics.push(Diagnostic {
                        severity: Severity::Error,
                        code: "INVALID_EXAMPLE".into(),
                        message,
                        file: path.display().to_string(),
                        line: *start,
                        column: 1,
                        pointer: None,
                    }),
                }
                fence = None;
            } else {
                body.push(line);
            }
            continue;
        }
        let trimmed = line.trim_start();
        if let Some(info) = trimmed.strip_prefix("```") {
            let mut parts = info.split_whitespace();
            let lang = parts.next().unwrap_or("").to_ascii_lowercase();
            if matches!(lang.as_str(), "json" | "jsonc" | "curl" | "sh" | "bash") {
                let metadata = parts.collect::<Vec<_>>().join(" ");
                if lang == "sh" || lang == "bash" {
                    // Shell fences are considered only when their body starts with curl.
                    fence = Some((
                        "curl-shell".into(),
                        parse_mapping(&metadata),
                        number + 1,
                        Vec::new(),
                    ));
                } else {
                    fence = Some((lang, parse_mapping(&metadata), number + 1, Vec::new()));
                }
            }
        }
    }
    if let Some((_, _, start, _)) = fence {
        diagnostics.push(Diagnostic {
            severity: Severity::Error,
            code: "UNCLOSED_FENCE".into(),
            message: "example fence is not closed".into(),
            file: path.display().to_string(),
            line: start.saturating_sub(1),
            column: 1,
            pointer: None,
        });
    }
    Ok(())
}

fn parse_mapping(info: &str) -> Mapping {
    let mut mapping = Mapping::default();
    for part in info.split_whitespace() {
        let Some((key, value)) = part.split_once('=') else {
            continue;
        };
        let value = value.trim_matches(['"', '\'']);
        match key {
            "operation" => mapping.operation = Some(value.into()),
            "schema" => mapping.schema = Some(value.into()),
            "direction" if value.eq_ignore_ascii_case("request") => {
                mapping.direction = Some(Direction::Request)
            }
            "direction" if value.eq_ignore_ascii_case("response") => {
                mapping.direction = Some(Direction::Response)
            }
            _ => {}
        }
    }
    mapping
}

fn parse_json_example(raw: &str) -> Result<Value, String> {
    serde_json::from_str(raw).map_err(|e| format!("example is not valid JSON: {e}"))
}

fn parse_curl_body(raw: &str) -> Result<Value, String> {
    let normalized = raw.replace("\\\n", " ");
    if !normalized.trim_start().starts_with("curl ") && normalized.trim() != "curl" {
        return Err(
            "shell fence is ignored unless it starts with curl; use a curl or json fence".into(),
        );
    }
    let flags = ["--data-raw", "--data-binary", "--data", "-d"];
    let mut found = None;
    for flag in flags {
        if let Some(pos) = find_token(&normalized, flag) {
            found = Some((pos, flag.len()));
            break;
        }
    }
    let Some((pos, len)) = found else {
        return Err("curl example has no JSON body; add --data or -d".into());
    };
    let tail = normalized[pos + len..].trim_start();
    let body = take_shell_value(tail)?;
    serde_json::from_str(&body).map_err(|e| format!("curl request body is not valid JSON: {e}"))
}

fn find_token(text: &str, token: &str) -> Option<usize> {
    text.match_indices(token).find_map(|(i, _)| {
        let before = text[..i].chars().next_back();
        let after = text[i + token.len()..].chars().next();
        if before.is_none_or(char::is_whitespace) && after.is_none_or(char::is_whitespace) {
            Some(i)
        } else {
            None
        }
    })
}

fn take_shell_value(text: &str) -> Result<String, String> {
    let Some(first) = text.chars().next() else {
        return Err("curl data flag has no value".into());
    };
    if first == '\'' || first == '"' {
        let mut escaped = false;
        let mut output = String::new();
        for ch in text[1..].chars() {
            if first == '"' && escaped {
                output.push(ch);
                escaped = false;
                continue;
            }
            if first == '"' && ch == '\\' {
                escaped = true;
                continue;
            }
            if ch == first {
                return Ok(output);
            }
            output.push(ch);
        }
        Err("curl data value has an unclosed quote".into())
    } else {
        Ok(text.split_whitespace().next().unwrap_or("").to_string())
    }
}

fn extract_openapi_examples(
    path: &Path,
    root: &Value,
    options: &CheckOptions,
    examples: &mut Vec<Example>,
) -> Result<(), AppError> {
    if root.get("openapi").is_none() && root.get("swagger").is_none() {
        return Ok(());
    }
    if let Some(name) = &options.schema {
        let schema = resolve_named_schema(root, name)?;
        collect_schema_examples(
            schema,
            path,
            Mapping {
                schema: Some(name.clone()),
                ..Mapping::default()
            },
            examples,
        );
        return Ok(());
    }
    if let Some(id) = &options.operation {
        let op = find_operation(root, id)?;
        let directions: Vec<Direction> = options
            .direction
            .map(|v| vec![v])
            .unwrap_or_else(|| vec![Direction::Request, Direction::Response]);
        for direction in directions {
            collect_operation_examples(op.value, path, id, direction, examples);
        }
        return Ok(());
    }
    if let Some(paths) = root.get("paths").and_then(Value::as_object) {
        for item in paths.values().filter_map(Value::as_object) {
            for method in [
                "get", "put", "post", "delete", "patch", "options", "head", "trace",
            ] {
                let Some(operation) = item.get(method) else {
                    continue;
                };
                let id = operation
                    .get("operationId")
                    .and_then(Value::as_str)
                    .unwrap_or("(operation without operationId)");
                collect_operation_examples(operation, path, id, Direction::Request, examples);
                collect_operation_examples(operation, path, id, Direction::Response, examples);
            }
        }
    }
    Ok(())
}

fn collect_schema_examples(
    schema: &Value,
    path: &Path,
    mapping: Mapping,
    examples: &mut Vec<Example>,
) {
    if let Some(value) = schema.get("example") {
        examples.push(Example {
            value: value.clone(),
            file: path.display().to_string(),
            line: 1,
            mapping: mapping.clone(),
        });
    }
    if let Some(values) = schema.get("examples").and_then(Value::as_array) {
        for value in values {
            examples.push(Example {
                value: value.clone(),
                file: path.display().to_string(),
                line: 1,
                mapping: mapping.clone(),
            });
        }
    }
}

fn collect_operation_examples(
    operation: &Value,
    path: &Path,
    id: &str,
    direction: Direction,
    examples: &mut Vec<Example>,
) {
    let mapping = Mapping {
        operation: Some(id.into()),
        direction: Some(direction),
        schema: None,
    };
    let contents: Vec<&Value> = match direction {
        Direction::Request => operation
            .pointer("/requestBody/content")
            .into_iter()
            .collect(),
        Direction::Response => operation
            .get("responses")
            .and_then(Value::as_object)
            .into_iter()
            .flat_map(|responses| responses.values())
            .filter_map(|r| r.get("content"))
            .collect(),
    };
    for content in contents {
        let Some(media) = choose_media(content) else {
            continue;
        };
        if let Some(value) = media.get("example") {
            examples.push(Example {
                value: value.clone(),
                file: path.display().to_string(),
                line: 1,
                mapping: mapping.clone(),
            });
        }
        if let Some(named) = media.get("examples").and_then(Value::as_object) {
            for item in named.values() {
                if let Some(value) = item.get("value") {
                    examples.push(Example {
                        value: value.clone(),
                        file: path.display().to_string(),
                        line: 1,
                        mapping: mapping.clone(),
                    });
                }
            }
        }
        if let Some(schema) = media.get("schema") {
            collect_schema_examples(schema, path, mapping.clone(), examples);
        }
    }
}

fn choose_media(content: &Value) -> Option<&Value> {
    let object = content.as_object()?;
    object
        .get("application/json")
        .or_else(|| {
            object
                .iter()
                .find(|(key, _)| key.ends_with("+json"))
                .map(|(_, v)| v)
        })
        .or_else(|| object.values().next())
}

fn resolve_named_schema<'a>(root: &'a Value, name: &str) -> Result<&'a Value, AppError> {
    root.pointer(&format!("/components/schemas/{}", escape_pointer(name)))
        .ok_or_else(|| AppError {
            message: format!("schema '{name}' was not found in components.schemas"),
        })
}

fn find_operation<'a>(root: &'a Value, id: &str) -> Result<Operation<'a>, AppError> {
    let paths = root
        .get("paths")
        .and_then(Value::as_object)
        .ok_or_else(|| AppError {
            message: "OpenAPI document has no paths object".into(),
        })?;
    for (path, item) in paths {
        for method in [
            "get", "put", "post", "delete", "patch", "options", "head", "trace",
        ] {
            if let Some(value) = item.get(method) {
                if value.get("operationId").and_then(Value::as_str) == Some(id) {
                    return Ok(Operation {
                        method,
                        path,
                        value,
                    });
                }
            }
        }
    }
    Err(AppError {
        message: format!("operationId '{id}' was not found"),
    })
}

fn schema_for_mapping<'a>(
    root: Option<&'a Value>,
    mapping: &Mapping,
) -> Result<(&'a Value, Option<Operation<'a>>), String> {
    let root = root.ok_or_else(|| {
        "example has no schema; pass --spec with --schema or --operation".to_string()
    })?;
    if let Some(name) = &mapping.schema {
        return resolve_named_schema(root, name)
            .map(|s| (s, None))
            .map_err(|e| e.message);
    }
    let id = mapping.operation.as_ref().ok_or_else(|| {
        "example has no mapping; add fence metadata or pass --operation/--schema".to_string()
    })?;
    let op = find_operation(root, id).map_err(|e| e.message)?;
    let direction = mapping.direction.unwrap_or(Direction::Request);
    let schema = operation_schema(op.value, direction).ok_or_else(|| {
        format!(
            "operation '{id}' has no {} JSON schema",
            match direction {
                Direction::Request => "request",
                Direction::Response => "response",
            }
        )
    })?;
    Ok((schema, Some(op)))
}

fn operation_schema(operation: &Value, direction: Direction) -> Option<&Value> {
    match direction {
        Direction::Request => {
            choose_media(operation.pointer("/requestBody/content")?)?.get("schema")
        }
        Direction::Response => {
            let responses = operation.get("responses")?.as_object()?;
            let response = responses
                .iter()
                .find(|(k, _)| k.starts_with('2'))
                .map(|(_, v)| v)
                .or_else(|| responses.get("default"))?;
            choose_media(response.get("content")?)?.get("schema")
        }
    }
}

fn validate_value(
    value: &Value,
    schema: &Value,
    root: &Value,
    pointer: &str,
    issues: &mut Vec<(String, String)>,
    depth: usize,
) {
    if depth > 64 {
        issues.push((pointer.into(), "schema reference depth exceeded 64".into()));
        return;
    }
    if let Some(reference) = schema.get("$ref").and_then(Value::as_str) {
        if !reference.starts_with("#/") {
            issues.push((
                pointer.into(),
                format!("remote $ref '{reference}' is not supported"),
            ));
            return;
        }
        let target = root.pointer(&reference[1..]);
        match target {
            Some(target) => validate_value(value, target, root, pointer, issues, depth + 1),
            None => issues.push((pointer.into(), format!("unresolved $ref '{reference}'"))),
        }
        return;
    }
    for key in ["allOf"] {
        if let Some(items) = schema.get(key).and_then(Value::as_array) {
            for item in items {
                validate_value(value, item, root, pointer, issues, depth + 1);
            }
        }
    }
    for key in ["anyOf", "oneOf"] {
        if let Some(items) = schema.get(key).and_then(Value::as_array) {
            let matches = items
                .iter()
                .filter(|item| {
                    let mut local = Vec::new();
                    validate_value(value, item, root, pointer, &mut local, depth + 1);
                    local.is_empty()
                })
                .count();
            let valid = if key == "anyOf" {
                matches >= 1
            } else {
                matches == 1
            };
            if !valid {
                issues.push((
                    pointer.into(),
                    format!(
                        "must match {} schema in {key}, matched {matches}",
                        if key == "anyOf" {
                            "at least one"
                        } else {
                            "exactly one"
                        }
                    ),
                ));
            }
        }
    }
    if let Some(inner) = schema.get("not") {
        let mut local = Vec::new();
        validate_value(value, inner, root, pointer, &mut local, depth + 1);
        if local.is_empty() {
            issues.push((pointer.into(), "must not match the forbidden schema".into()));
        }
    }
    if schema.get("nullable") == Some(&Value::Bool(true)) && value.is_null() {
        return;
    }
    if let Some(expected) = schema.get("type") {
        let valid = match expected {
            Value::String(kind) => matches_type(value, kind),
            Value::Array(kinds) => kinds
                .iter()
                .filter_map(Value::as_str)
                .any(|kind| matches_type(value, kind)),
            _ => true,
        };
        if !valid {
            issues.push((
                pointer.into(),
                format!(
                    "expected {}, found {}",
                    display_type(expected),
                    value_type(value)
                ),
            ));
            return;
        }
    }
    if let Some(values) = schema.get("enum").and_then(Value::as_array) {
        if !values.contains(value) {
            issues.push((
                pointer.into(),
                format!(
                    "value is not one of {}",
                    serde_json::to_string(values).unwrap_or_default()
                ),
            ));
        }
    }
    if let Some(expected) = schema.get("const") {
        if value != expected {
            issues.push((pointer.into(), format!("expected constant {}", expected)));
        }
    }
    if let Some(object) = value.as_object() {
        validate_object(object, schema, root, pointer, issues, depth);
    }
    if let Some(array) = value.as_array() {
        validate_array(array, schema, root, pointer, issues, depth);
    }
    if let Some(string) = value.as_str() {
        validate_string(string, schema, pointer, issues);
    }
    if let Some(number) = value.as_f64() {
        validate_number(number, schema, pointer, issues);
    }
}

fn validate_object(
    value: &Map<String, Value>,
    schema: &Value,
    root: &Value,
    pointer: &str,
    issues: &mut Vec<(String, String)>,
    depth: usize,
) {
    if let Some(required) = schema.get("required").and_then(Value::as_array) {
        for name in required.iter().filter_map(Value::as_str) {
            if !value.contains_key(name) {
                issues.push((
                    pointer.into(),
                    format!("missing required property '{name}'"),
                ));
            }
        }
    }
    let properties = schema.get("properties").and_then(Value::as_object);
    for (name, item) in value {
        let child = format!("{pointer}/{}", escape_pointer(name));
        if let Some(property_schema) = properties.and_then(|p| p.get(name)) {
            validate_value(item, property_schema, root, &child, issues, depth + 1);
        } else if schema.get("additionalProperties") == Some(&Value::Bool(false)) {
            issues.push((child, format!("property '{name}' is not allowed")));
        } else if let Some(additional) =
            schema.get("additionalProperties").filter(|v| v.is_object())
        {
            validate_value(item, additional, root, &child, issues, depth + 1);
        }
    }
    if let Some(min) = schema.get("minProperties").and_then(Value::as_u64) {
        if value.len() < min as usize {
            issues.push((
                pointer.into(),
                format!("must have at least {min} properties"),
            ));
        }
    }
    if let Some(max) = schema.get("maxProperties").and_then(Value::as_u64) {
        if value.len() > max as usize {
            issues.push((
                pointer.into(),
                format!("must have at most {max} properties"),
            ));
        }
    }
}

fn validate_array(
    value: &[Value],
    schema: &Value,
    root: &Value,
    pointer: &str,
    issues: &mut Vec<(String, String)>,
    depth: usize,
) {
    if let Some(min) = schema.get("minItems").and_then(Value::as_u64) {
        if value.len() < min as usize {
            issues.push((pointer.into(), format!("must contain at least {min} items")));
        }
    }
    if let Some(max) = schema.get("maxItems").and_then(Value::as_u64) {
        if value.len() > max as usize {
            issues.push((pointer.into(), format!("must contain at most {max} items")));
        }
    }
    if schema.get("uniqueItems") == Some(&Value::Bool(true)) {
        let mut seen = HashSet::new();
        for item in value {
            if !seen.insert(item.to_string()) {
                issues.push((pointer.into(), "array items must be unique".into()));
                break;
            }
        }
    }
    if let Some(item_schema) = schema.get("items") {
        for (index, item) in value.iter().enumerate() {
            validate_value(
                item,
                item_schema,
                root,
                &format!("{pointer}/{index}"),
                issues,
                depth + 1,
            );
        }
    }
}

fn validate_string(value: &str, schema: &Value, pointer: &str, issues: &mut Vec<(String, String)>) {
    let length = value.chars().count() as u64;
    if let Some(min) = schema.get("minLength").and_then(Value::as_u64) {
        if length < min {
            issues.push((
                pointer.into(),
                format!("string must contain at least {min} characters"),
            ));
        }
    }
    if let Some(max) = schema.get("maxLength").and_then(Value::as_u64) {
        if length > max {
            issues.push((
                pointer.into(),
                format!("string must contain at most {max} characters"),
            ));
        }
    }
    if let Some(pattern) = schema.get("pattern").and_then(Value::as_str) {
        match Regex::new(pattern) {
            Ok(regex) if !regex.is_match(value) => issues.push((
                pointer.into(),
                format!("string does not match pattern {pattern:?}"),
            )),
            Err(_) => issues.push((
                pointer.into(),
                format!("schema contains invalid pattern {pattern:?}"),
            )),
            _ => {}
        }
    }
    if let Some(format) = schema.get("format").and_then(Value::as_str) {
        let valid = match format {
            "email" => value
                .split_once('@')
                .is_some_and(|(a, b)| !a.is_empty() && b.contains('.')),
            "uuid" => {
                let p: Vec<_> = value.split('-').map(str::len).collect();
                p == [8, 4, 4, 4, 12] && value.chars().all(|c| c == '-' || c.is_ascii_hexdigit())
            }
            "date" => Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap().is_match(value),
            "date-time" => value.split_once('T').is_some_and(|(_, time)| {
                value.ends_with('Z') || time.contains('+') || time.contains('-')
            }),
            _ => true,
        };
        if !valid {
            issues.push((pointer.into(), format!("string is not a valid {format}")));
        }
    }
}

fn validate_number(value: f64, schema: &Value, pointer: &str, issues: &mut Vec<(String, String)>) {
    if let Some(min) = schema.get("minimum").and_then(Value::as_f64) {
        if value < min {
            issues.push((pointer.into(), format!("number must be at least {min}")));
        }
    }
    if let Some(max) = schema.get("maximum").and_then(Value::as_f64) {
        if value > max {
            issues.push((pointer.into(), format!("number must be at most {max}")));
        }
    }
    if let Some(min) = schema.get("exclusiveMinimum").and_then(Value::as_f64) {
        if value <= min {
            issues.push((pointer.into(), format!("number must be greater than {min}")));
        }
    }
    if let Some(max) = schema.get("exclusiveMaximum").and_then(Value::as_f64) {
        if value >= max {
            issues.push((pointer.into(), format!("number must be less than {max}")));
        }
    }
    if let Some(step) = schema.get("multipleOf").and_then(Value::as_f64) {
        if step > 0.0 && ((value / step).round() - value / step).abs() > 1e-9 {
            issues.push((
                pointer.into(),
                format!("number must be a multiple of {step}"),
            ));
        }
    }
}

fn matches_type(value: &Value, kind: &str) -> bool {
    match kind {
        "null" => value.is_null(),
        "object" => value.is_object(),
        "array" => value.is_array(),
        "string" => value.is_string(),
        "boolean" => value.is_boolean(),
        "number" => value.is_number(),
        "integer" => value.as_i64().is_some() || value.as_u64().is_some(),
        _ => true,
    }
}
fn value_type(value: &Value) -> &'static str {
    if value.is_null() {
        "null"
    } else if value.is_object() {
        "object"
    } else if value.is_array() {
        "array"
    } else if value.is_string() {
        "string"
    } else if value.is_boolean() {
        "boolean"
    } else {
        "number"
    }
}
fn display_type(value: &Value) -> String {
    match value {
        Value::String(v) => v.clone(),
        _ => value.to_string(),
    }
}
fn escape_pointer(value: &str) -> String {
    value.replace('~', "~0").replace('/', "~1")
}

fn mock_check(
    base: &str,
    allowed_hosts: &[String],
    method: &str,
    path: &str,
    value: &Value,
) -> Result<(), String> {
    if path.contains('{') {
        return Err("mock path contains parameters; operation paths with {parameters} are not sent automatically".into());
    }
    let base = Url::parse(base).map_err(|e| format!("invalid mock base URL: {e}"))?;
    if base.scheme() != "http" {
        return Err(
            "mock base URL must use http; TLS and redirects are intentionally unsupported".into(),
        );
    }
    let host = base
        .host_str()
        .ok_or_else(|| "mock base URL has no hostname".to_string())?;
    let safe = matches!(host, "localhost" | "127.0.0.1" | "::1")
        || allowed_hosts.iter().any(|h| h == host);
    if !safe {
        return Err(format!(
            "mock host '{host}' is not allowed; add --allow-host {host} explicitly"
        ));
    }
    let url = base
        .join(path.trim_start_matches('/'))
        .map_err(|e| format!("cannot join mock URL: {e}"))?;
    let port = url.port_or_known_default().unwrap_or(80);
    let address = (host, port)
        .to_socket_addrs()
        .map_err(|e| format!("cannot resolve mock host: {e}"))?
        .next()
        .ok_or_else(|| "mock host did not resolve".to_string())?;
    let mut stream = TcpStream::connect_timeout(&address, Duration::from_secs(3))
        .map_err(|e| format!("cannot connect to mock server: {e}"))?;
    stream.set_read_timeout(Some(Duration::from_secs(5))).ok();
    stream.set_write_timeout(Some(Duration::from_secs(5))).ok();
    let body = value.to_string();
    let target = if let Some(query) = url.query() {
        format!("{}?{query}", url.path())
    } else {
        url.path().to_string()
    };
    let request = format!(
        "{} {} HTTP/1.1\r\nHost: {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        method.to_ascii_uppercase(),
        target,
        host,
        body.len(),
        body
    );
    stream
        .write_all(request.as_bytes())
        .map_err(|e| format!("cannot write mock request: {e}"))?;
    let mut response = String::new();
    stream
        .read_to_string(&mut response)
        .map_err(|e| format!("cannot read mock response: {e}"))?;
    let status = response
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|v| v.parse::<u16>().ok())
        .ok_or_else(|| "mock server returned an invalid HTTP response".to_string())?;
    if (200..400).contains(&status) {
        Ok(())
    } else {
        Err(format!("mock server returned HTTP {status}"))
    }
}

pub fn render_report(report: &Report, format: OutputFormat) -> String {
    match format {
        OutputFormat::Json => {
            serde_json::to_string_pretty(report).expect("report is serializable") + "\n"
        }
        OutputFormat::Github => {
            let mut out = String::new();
            for d in &report.diagnostics {
                out.push_str(&format!(
                    "::{} file={},line={},col={},title={}::{}\n",
                    if d.severity == Severity::Error {
                        "error"
                    } else {
                        "warning"
                    },
                    github_escape_property(&d.file),
                    d.line,
                    d.column,
                    github_escape_property(&d.code),
                    github_escape_message(&format!(
                        "{}{}",
                        d.message,
                        d.pointer
                            .as_ref()
                            .map(|p| format!(" at {p}"))
                            .unwrap_or_default()
                    ))
                ));
            }
            out.push_str(&format!(
                "Checked {} example(s): {} passed, {} failed.\n",
                report.summary.discovered, report.summary.passed, report.summary.failed
            ));
            out
        }
        OutputFormat::Auto | OutputFormat::Text => {
            let mut out = String::new();
            for d in &report.diagnostics {
                out.push_str(&format!(
                    "{}:{}:{}  {}  {}{}\n",
                    d.file,
                    d.line,
                    d.column,
                    d.code,
                    d.message,
                    d.pointer
                        .as_ref()
                        .map(|p| format!(" ({p})"))
                        .unwrap_or_default()
                ));
            }
            let mark = if report.diagnostics.is_empty() {
                "PASS"
            } else {
                "FAIL"
            };
            out.push_str(&format!(
                "{mark}  {} example(s) checked · {} passed · {} failed\n",
                report.summary.discovered, report.summary.passed, report.summary.failed
            ));
            out
        }
    }
}

fn github_escape_property(value: &str) -> String {
    value
        .replace('%', "%25")
        .replace('\r', "%0D")
        .replace('\n', "%0A")
        .replace(':', "%3A")
        .replace(',', "%2C")
}
fn github_escape_message(value: &str) -> String {
    value
        .replace('%', "%25")
        .replace('\r', "%0D")
        .replace('\n', "%0A")
}

pub fn error_json(message: &str) -> String {
    serde_json::to_string_pretty(
        &json!({"error": {"code": "CONFIGURATION_ERROR", "message": message}}),
    )
    .unwrap()
        + "\n"
}

#[cfg(test)]
mod tests {
    use super::*;

    fn specimen() -> Value {
        serde_yaml::from_str(
            r#"
openapi: 3.1.0
paths:
  /pets:
    post:
      operationId: createPet
      requestBody:
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/Pet'
      responses:
        '201':
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/Pet'
components:
  schemas:
    Pet:
      type: object
      additionalProperties: false
      required: [name]
      properties:
        name: { type: string, minLength: 2 }
        age: { type: integer, minimum: 0 }
"#,
        )
        .unwrap()
    }

    #[test]
    fn extracts_json_and_safe_curl() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("guide.md");
        fs::write(&path, "```json operation=createPet direction=request\n{\"name\":\"Ada\"}\n```\n```curl operation=createPet\ncurl http://x -d '{\"name\":\"Bo\"}'\n```\n").unwrap();
        let mut examples = Vec::new();
        let mut diagnostics = Vec::new();
        extract_markdown(&path, &mut examples, &mut diagnostics).unwrap();
        assert_eq!(examples.len(), 2);
        assert!(diagnostics.is_empty());
        assert_eq!(examples[1].value["name"], "Bo");
    }

    #[test]
    fn reports_precise_schema_problems() {
        let root = specimen();
        let schema = resolve_named_schema(&root, "Pet").unwrap();
        let value = json!({"name":"A", "age":-1, "extra":true});
        let mut issues = Vec::new();
        validate_value(&value, schema, &root, "$", &mut issues, 0);
        assert!(
            issues
                .iter()
                .any(|(p, m)| p == "$/name" && m.contains("at least 2"))
        );
        assert!(issues.iter().any(|(p, _)| p == "$/age"));
        assert!(issues.iter().any(|(p, _)| p == "$/extra"));
    }

    #[test]
    fn operation_schema_resolves_local_ref() {
        let root = specimen();
        let op = find_operation(&root, "createPet").unwrap();
        let schema = operation_schema(op.value, Direction::Request).unwrap();
        let mut issues = Vec::new();
        validate_value(&json!({"name":"Milo"}), schema, &root, "$", &mut issues, 0);
        assert!(issues.is_empty());
    }

    #[test]
    fn github_output_escapes_workflow_commands() {
        let report = Report {
            summary: Summary {
                discovered: 1,
                passed: 0,
                failed: 1,
            },
            diagnostics: vec![Diagnostic {
                severity: Severity::Error,
                code: "BAD,VALUE".into(),
                message: "line\nnew".into(),
                file: "docs/a:b.md".into(),
                line: 4,
                column: 1,
                pointer: None,
            }],
        };
        let out = render_report(&report, OutputFormat::Github);
        assert!(out.contains("file=docs/a%3Ab.md"));
        assert!(out.contains("title=BAD%2CVALUE"));
        assert!(out.contains("line%0Anew"));
    }

    #[test]
    fn curl_text_is_never_executed() {
        assert!(parse_curl_body("curl x; touch /tmp/nope").is_err());
    }

    #[test]
    fn ordinary_shell_blocks_are_ignored() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("guide.md");
        fs::write(&path, "```sh\nnpm test\n```\n").unwrap();
        let mut examples = Vec::new();
        let mut diagnostics = Vec::new();
        extract_markdown(&path, &mut examples, &mut diagnostics).unwrap();
        assert!(examples.is_empty());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn mock_hosts_require_explicit_permission() {
        let error = mock_check("http://example.com", &[], "post", "/pets", &json!({})).unwrap_err();
        assert!(error.contains("not allowed"));
    }

    #[test]
    fn mock_check_accepts_a_local_success_response() {
        use std::net::TcpListener;
        use std::thread;

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut request = [0_u8; 1024];
            let read = stream.read(&mut request).unwrap();
            assert!(String::from_utf8_lossy(&request[..read]).starts_with("POST /pets HTTP/1.1"));
            stream
                .write_all(
                    b"HTTP/1.1 204 No Content\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                )
                .unwrap();
        });
        mock_check(
            &format!("http://127.0.0.1:{port}/"),
            &[],
            "post",
            "/pets",
            &json!({"name":"Ada"}),
        )
        .unwrap();
        server.join().unwrap();
    }
}
