# API Example Linter

`api-example-linter` checks copied API examples against an OpenAPI contract.
It reads fenced JSON, curl request bodies, and examples inside OpenAPI 3.x files.
Failed checks name the file, line, JSON pointer, and mismatch.

It is for API maintainers who want documentation examples checked in CI.
Curl blocks are parsed as text and never executed.

## Try the bundled sample

Open the [isolated web demo](https://api-example-linter.sociobot.in/demo/?demo=1), or run:

```sh
api-example-linter demo
```

The command copies [examples](examples/) into a fresh temporary folder.
It runs the real validator against one current example and one stale example.
It then removes the folder without reading project configuration.

## Install

Download a release binary, or install from source with Rust 1.85 or newer:

```sh
cargo install --git https://github.com/B-Divyesh/sf-api-example-linter.git
```

## Usage

Check Markdown against a named component schema:

```sh
api-example-linter check docs/quickstart.md \
  --spec openapi.yaml \
  --schema Pet
```

Check request examples against an operation:

```sh
api-example-linter check docs/create-pet.md \
  --spec openapi.yaml \
  --operation createPet \
  --direction request
```

Check examples embedded in an OpenAPI file:

```sh
api-example-linter check openapi.yaml --operation createPet
```

Use JSON or GitHub Actions output:

```sh
api-example-linter check docs --spec openapi.yaml --schema Pet --format json
api-example-linter check docs --spec openapi.yaml --schema Pet --format github
```

Text, JSON, and GitHub output identify each failed example.
Exit code `0` means all examples passed.
Exit code `1` means validation findings exist.
Exit code `2` means the input or configuration is invalid.

### Markdown conventions

JSON fences are validated directly.
Curl fences support `--data`, `--data-raw`, `--data-binary`, and `-d`.
Separated, equals, and compact flag forms are accepted.
No shell command is run.

Add mapping metadata to a fence when global flags are not suitable:

````md
```json operation=createPet direction=request
{"name":"Ada","tag":"rescue"}
```
````

Metadata keys are `operation`, `schema`, and `direction`.
Direction can be `request` or `response`.
Global flags provide defaults for those values.

### Optional mock request

`--mock-base-url` sends validated requests to a mock server only after you opt in.
Loopback HTTP hosts are accepted by default.
Use `--allow-host` to permit another hostname.

```sh
api-example-linter check docs/create-pet.md \
  --spec openapi.yaml --operation createPet --direction request \
  --mock-base-url http://127.0.0.1:4010
```

### Configuration

Put `.api-example-linter.json` at the repository root:

```json
{
  "spec": "openapi.yaml",
  "inputs": ["docs"],
  "operation": "createPet",
  "direction": "request",
  "format": "auto"
}
```

CLI flags override configuration.
`api-example-linter init` writes a starter file and refuses to overwrite an existing file.

## CI

```yaml
- name: Lint API examples
  run: api-example-linter check --format github
```

## Privacy and offline behavior

Default checks use local files and make no network requests.
Local references resolve without remote fetching.
The optional mock request is the only CLI network path.

The website makes only same-origin requests and sets no cookies.
Its service worker keeps the guide and bundled demo available after the first visit.
The demo uses a separate `demo:` session-storage namespace.

## Develop and verify

```sh
npm ci
npm test
npm run build
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo package --locked
```

`npm test` runs Rust, site, claim, browser, accessibility, privacy, and offline checks.
`npm run build` writes the binary to `dist/bin` and the static site to `dist/site`.
`npm run build:site` builds only the static site.

Every visitor-facing claim is mapped to one tagged test in [.factory/claims.json](.factory/claims.json).
The [demo contract](.factory/demo.md) documents isolation and reset behavior.

## Scope

Version 0.1 checks JSON values against the selected OpenAPI operation or named schema.
It checks required fields, unknown fields, scalar types, and local references.
Remote references are reported instead of fetched.

## License

MIT. See [LICENSE](LICENSE).
