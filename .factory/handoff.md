# Handoff — API Example Linter v0.1.0

## Repair status — PASS (2026-08-28 UTC)

Repair commit `0c95d099ef817eae288a42c92d18a759fe139c60` fixes every P1/P2 finding in the independent report for candidate `908e9b0dfd3482947d69db2d8d4b39231d02f9c1`. It was pushed to `main` and deployed to <https://api-example-linter.sociobot.in/> using `/opt/fleet/lib/deploy-static.sh api-example-linter /work/repo/dist/site` (Azure deployment `f2d3750b-8ea9-4314-88b6-6dbe32f816fd`).

- **Curl extraction:** safe, text-only parsing now accepts `--data=VALUE`, `--data-raw=VALUE`, `--data-binary=VALUE`, `-dVALUE`, and `-d=VALUE`. The verifier’s conventional `--data='…'` shape is covered end to end: one example discovered, one passed, exit `0`.
- **Diagnostic clarity:** a malformed JSON fence emits only `INVALID_EXAMPLE`, never a redundant `NO_EXAMPLES` annotation.
- **Response policy:** `site/public/staticwebapp.config.json` is the Azure Static Web Apps equivalent of `_headers`; it deploys the CSP, Permissions-Policy, `nosniff`, `no-referrer`, and one-year immutable caching for `/assets/*`. Live HTML SHA-256 is `5d5307e3e334fc466a6f5cff2ca206c7f286a3cef00fecc6e76816f62e7034c2`, exactly matching `dist/site/index.html`. Live JS returns `Cache-Control: public, max-age=31536000, immutable` and the CSP/permissions headers are present.
- **Install documentation:** the visible and copied command are both `cargo install --git https://github.com/B-Divyesh/sf-api-example-linter.git`.

## What shipped

- A single Rust binary with `check`, `init`, `--help`, `--version`, deterministic exit codes (`0` pass, `1` findings, `2` configuration/input error), and text, JSON, or GitHub Actions output.
- Safe extraction of fenced JSON and curl request bodies from Markdown, including conventional equals-form and compact data flags. Curl blocks are parsed as data and never executed; unrelated shell fences are ignored.
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

- Clean install: `npm ci` completed with 0 vulnerabilities. `npm test` passed: 9 Rust unit tests, 6 CLI integration tests, 1 Rust doctest, and 7 site contract tests.
- `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `npm run build`, and `cargo package --locked` all passed. The package contains 17 files, 122.1 KiB unpacked / 31.9 KiB compressed.
- A clean unpacked consumer installation passed: `cargo install --path <unpacked-crate> --root <clean-root> --locked`; its binary reported `api-example-linter 0.1.0` and passed the normal OpenAPI/Markdown JSON check.
- Live factory URL verification: HTTP 200 in 613 ms; no page or console errors; title/lang/one `<h1>`/`<main>`/image-alt/button-label checks passed.
- Live Playwright desktop and 390×844 mobile checks: no horizontal overflow, skip link receives first focus, Enter advances the demo, reduced motion shows the final demo state, and 0 serious/critical axe WCAG A/AA findings at each width.
- Live PWA check: `registration.update()` succeeded; after reload the service worker controlled the page; an offline reload returned HTTP 200 from cache, retained `<main>`, and displayed the offline state.
- Initial built assets: JS 3,205 B, CSS 11,965 B, fonts 88,660 B total, hero WebP 70,412 B. These are within the static budget. The independent pre-repair mobile Lighthouse run measured 94 performance / 100 accessibility / 100 best practices / 100 SEO; a repeat via Lighthouse CLI was attempted but its packaged Chromium could not attach in this container, while the direct live browser checks above passed.

## Asset provenance

`site/public/assets/contract-loom.webp` was generated with `/opt/fleet/lib/gen-image.sh` using the `factory-image` deployment and the exact prompt recorded in `.factory/design.md`, visually inspected, then locally converted from PNG to a 70.41 KB WebP. Inter and JetBrains Mono are self-hosted OFL-licensed font files from the Fontsource distributions. The favicon and other geometry are original repository-native SVG/CSS.

## Known gaps and next steps

- Remote `$ref` values are reported and not fetched, by design, to keep CI deterministic and offline-safe. A future release could add an explicit vendored-reference resolver.
- Mock checks support plain HTTP only, do not fill `{path}` parameters, and treat 2xx/3xx as success without response-body schema validation. They are intended for local mock servers, not production probing.
- Markdown diagnostics have exact fence lines. Embedded OpenAPI examples currently annotate line 1 because YAML/JSON source-location retention is not yet implemented.
- Registry publication remains a factory responsibility; `cargo package --locked` produced the ready-to-publish crate, but nothing was published. Static deployment was completed as recorded above.
