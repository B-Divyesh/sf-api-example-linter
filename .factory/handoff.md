# Handoff — API Example Linter v0.1.0

## What shipped

- A single Rust binary with `check`, `init`, `--help`, `--version`, deterministic exit codes (`0` pass, `1` findings, `2` configuration/input error), and text, JSON, or GitHub Actions output.
- Safe extraction of fenced JSON and curl request bodies from Markdown. Curl blocks are parsed as data and never executed; unrelated shell fences are ignored.
- OpenAPI 3.0/3.1 JSON or YAML support for named component schemas, operation request/response schemas, embedded examples, local `$ref` values, and a practical JSON Schema validation subset.
- Optional mock-server request checks over HTTP, restricted to loopback unless a hostname is explicitly allowlisted. Redirects are never followed and parameterized paths are rejected rather than guessed.
- A static Vite documentation site with the original “Contract Loom” generative illustration, exact install/config examples, an opt-in recorded terminal demo, responsive 390px layout, light/dark themes, offline state, service-worker shell cache, and privacy/terms pages.
- Self-hosted Inter and JetBrains Mono variable font subsets; no analytics, cookies, third-party runtime code, or telemetry.

## Run and verify

```sh
npm ci
npm test
npm run build
cargo package --locked
```

`npm run build` writes the release binary to `dist/bin/api-example-linter` and the deployable static site to `dist/site` (with `dist/site/index.html` at its root). `npm run build:site` builds only the deploy target. The crate packaging check creates `target/package/api-example-linter-0.1.0.crate`; publishing is intentionally left to the factory.

Manual smoke command:

```sh
./target/release/api-example-linter check fixtures/valid.md \
  --spec fixtures/openapi.yaml --operation createPet
```

## Verification results

- `npm test`: 8 Rust unit tests, 4 CLI integration tests, 1 Rust doctest, and 5 site contract tests passed.
- `cargo clippy --all-targets -- -D warnings`: passed.
- `cargo package --allow-dirty --locked`: passed; 119.4 KiB package, 31.2 KiB compressed.
- Factory URL verifier: HTTP 200, no page/console errors, title/lang/main/alt/button checks passed; desktop and 390×844 screenshots reviewed.
- Keyboard/mobile smoke: skip link receives first focus, demo runs from Enter, offline status appears, and the 390px page has no horizontal overflow.
- axe-core/Playwright at 390×844: 0 violations in light mode and 0 in dark mode.
- Lighthouse mobile: Performance 99, Accessibility 100, Best Practices 100, SEO 100; LCP 2.0 s, total blocking time 0 ms, CLS 0.
- Initial assets: JS 3.21 KB, CSS 11.97 KB, fonts 88.66 KB total, hero WebP 70.41 KB. These are raw transfer sizes before compression except the already-compressed fonts/image.
- `npm audit`: 0 vulnerabilities.

## Asset provenance

`site/public/assets/contract-loom.webp` was generated with `/opt/fleet/lib/gen-image.sh` using the `factory-image` deployment and the exact prompt recorded in `.factory/design.md`, visually inspected, then locally converted from PNG to a 70.41 KB WebP. Inter and JetBrains Mono are self-hosted OFL-licensed font files from the Fontsource distributions. The favicon and other geometry are original repository-native SVG/CSS.

## Known gaps and next steps

- Remote `$ref` values are reported and not fetched, by design, to keep CI deterministic and offline-safe. A future release could add an explicit vendored-reference resolver.
- Mock checks support plain HTTP only, do not fill `{path}` parameters, and treat 2xx/3xx as success without response-body schema validation. They are intended for local mock servers, not production probing.
- Markdown diagnostics have exact fence lines. Embedded OpenAPI examples currently annotate line 1 because YAML/JSON source-location retention is not yet implemented.
- Release archives and registry publication are factory responsibilities; no package was published and no deployment infrastructure was changed.
