# API Example Linter

`api-example-linter` keeps the request and response examples people copy from API documentation aligned with the OpenAPI contract. It extracts fenced JSON and curl bodies from Markdown, reads examples embedded in OpenAPI 3.x documents, validates them against a selected operation or named schema, and emits concise local diagnostics or line-level GitHub Actions annotations.

It is built for API maintainers who want one small, deterministic CI gate—not a documentation host, API fuzzer, or shell runner. Curl blocks are parsed as text and are **never executed**.

## Install

Download a release binary, or install from source with Rust 1.85+:

```sh
cargo install --path .
```

## Usage

Validate examples from Markdown against a named component schema:

```sh
api-example-linter check docs/quickstart.md \
  --spec openapi.yaml \
  --schema Pet
```

Validate request examples against an operation:

```sh
api-example-linter check docs/create-pet.md \
  --spec openapi.yaml \
  --operation createPet \
  --direction request
```

Validate examples already embedded in an OpenAPI file:

```sh
api-example-linter check openapi.yaml --operation createPet
```

Machine-readable and GitHub Actions output:

```sh
api-example-linter check docs --spec openapi.yaml --schema Pet --format json
api-example-linter check docs --spec openapi.yaml --schema Pet --format github
```

In `auto` format, the CLI emits GitHub workflow commands when `GITHUB_ACTIONS=true`; otherwise it uses readable terminal output. Exit code `0` means every discovered example passed, `1` means validation findings were found, and `2` means configuration or input failed.

### Markdown conventions

JSON fences are validated directly. Curl fences are parsed safely; JSON bodies following `--data`, `--data-raw`, `--data-binary`, or `-d` are extracted in their normal separated, `--flag=VALUE`, and compact `-dVALUE` forms. No command is run.

To map a block to an operation without global flags, add metadata to the fence:

````md
```json operation=createPet direction=request
{"name":"Ada","tag":"rescue"}
```
````

Supported metadata keys are `operation`, `schema`, and `direction` (`request` or `response`). A global `--operation`, `--schema`, or `--direction` supplies defaults.

### Optional safe HTTP check

`--mock-base-url` sends validated request examples to a mock server using the selected operation method and path. It accepts `http://localhost`, `http://127.0.0.1`, and `http://[::1]` by default. Add `--allow-host example.internal` explicitly for another hostname. Redirects are disabled, private credentials are never inferred, and curl text is still never executed.

```sh
api-example-linter check docs/create-pet.md \
  --spec openapi.yaml --operation createPet --direction request \
  --mock-base-url http://127.0.0.1:4010
```

### Configuration

Put `.api-example-linter.json` at the repository root to keep CI under five minutes:

```json
{
  "spec": "openapi.yaml",
  "inputs": ["docs"],
  "operation": "createPet",
  "direction": "request",
  "format": "auto"
}
```

CLI flags override configuration. Run `api-example-linter init` to write a documented starter file without overwriting an existing one.

## CI

```yaml
- name: Lint API examples
  run: api-example-linter check --format github
```

## Develop and verify

```sh
cargo test
cargo build --release
cargo package --allow-dirty
npm install
npm test
npm run build       # binary + site -> dist/
npm run build:site  # static site only -> dist/site/
```

The static documentation site is deployed from `dist/site`. It has no analytics, external scripts, hosted fonts, cookies, or data collection; the live demo runs entirely in the browser.

## Scope

Version 0.1 validates the practical JSON Schema subset commonly used by OpenAPI examples: types, required properties, properties, additional properties, arrays/items, enums, const, nullable, string patterns/lengths, numeric bounds, composition (`allOf`, `anyOf`, `oneOf`, `not`), and local `$ref` values. OpenAPI 3.0 `nullable` and 3.1 JSON Schema are accepted. Remote `$ref` fetching is intentionally disabled for deterministic, offline-safe CI.

## License

MIT. See [LICENSE](LICENSE).
