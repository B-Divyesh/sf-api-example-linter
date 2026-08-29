# Demo contract

## Entry points

- One-click web demo: <https://api-example-linter.sociobot.in/demo/?demo=1>
- Compatible home query: <https://api-example-linter.sociobot.in/?demo=1>
- Focused web page: <https://api-example-linter.sociobot.in/demo/>
- CLI: `api-example-linter demo`

The home action opens `/demo/?demo=1` in one click.
Both web routes load the completed sample result immediately.

## Sample data

`examples/openapi.yaml` defines the `createPet` request.
`examples/create-pet.md` contains one matching JSON request and one stale curl body.
The stale body includes `retired_field`, which produces a line-level mismatch.

## Isolation and reset

The CLI creates a new operating-system temporary directory for every run.
It copies the bundled samples there and ignores project configuration.
The directory is removed when the command finishes.

The web recording requests no user files.
Its state uses only session-storage keys beginning with `demo:api-example-linter:`.
`Reset demo` clears that namespace and reloads the bundled result.
`Start for real` clears the namespace and returns home.
