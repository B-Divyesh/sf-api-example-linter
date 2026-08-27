# Independent verification — FAIL

**Work order:** `api-example-linter-verify-1`  
**Candidate:** `908e9b0dfd3482947d69db2d8d4b39231d02f9c1` (`main`)  
**Live URL:** <https://api-example-linter.sociobot.in/>  
**Verified:** 2026-08-27 UTC  
**Scope:** clean checkout, CLI/package, generated site, and live deployment. No product source was changed.

## Verdict

**FAIL — do not release this candidate.** The core promise includes extracting JSON bodies from curl examples, but a valid conventional curl form, `--data='…'`, is not recognized. In addition, the live deployment exactly serves the candidate content but does not apply the candidate's declared CSP, Permissions-Policy, or immutable cache policy.

## Release-blocking defects

### P1 — valid `curl --data=…` examples are rejected instead of linted

Reproduction from an independent temporary fixture (OpenAPI 3.1 operation `createPet`):

~~~~md
```curl operation=createPet direction=request
curl --data='{"name":"A","kind":"cat"}' https://example.invalid/pets
```
~~~~

```sh
api-example-linter check curl-equals.md --spec openapi.yaml \
  --operation createPet --direction request --format json
```

Expected: one discovered, valid JSON curl body and exit `0`. Actual: exit `1`, `discovered: 0`, `INVALID_EXAMPLE` (`curl example has no JSON body`) followed by `NO_EXAMPLES`. `--data=VALUE` is ordinary curl syntax, so this fails the brief's JSON/curl extraction job rather than producing a schema result and line-level fix.

### P1 — live deployment does not apply the committed response-security/cache policy

The exact built candidate HTML, JS, and CSS match production byte-for-byte:

- `index.html` SHA-256: `36601dc7cee2a42d5f0e86382ab330f6e6d9429ccbf394a4f6fb8145be8398fb`
- `assets/index-BnFUFOZU.js` SHA-256: `423e930f7700b46cc43b4c8681d4c04a3dbe32265d8b4192c06656be7571175b`
- `assets/style-DbmZ2ZSH.css` SHA-256: `90e3e44caf036ce28a8b0854db0f8fa254e296e4f615a27a1b5b40b08df4fb9b`

Yet production responses omit the `Content-Security-Policy` and `Permissions-Policy` declared in `site/public/_headers`. The returned policy includes HSTS, `nosniff`, and a referrer policy, but neither required policy is present. The HTML, hashed JS/CSS, WebP, and service worker all return `cache-control: public, must-revalidate, max-age=30`, not the committed immutable one-year asset policy. This is a deployment/configuration mismatch, but it means the candidate is not verified live against its privacy/security/performance contract.

## Non-blocking defects

### P2 — malformed examples report a redundant `NO_EXAMPLES` finding

For a malformed JSON fence, the tool correctly emits `INVALID_EXAMPLE`, then also emits `NO_EXAMPLES`. The latter is misleading because an example was found but could not be parsed. It produces a duplicate CI annotation for one underlying problem.

### P2 — the visible primary install command is not executable as displayed

The landing page displays `cargo install --git github.com/…/sf-api-example-linter`. The ellipsized repository is not a usable Git URL. Its Copy button supplies the complete HTTPS URL, so installation is recoverable, but the displayed command should also be valid and copyable.

## Clean-checkout quality gates

Detached clean worktree created at the candidate SHA. All of these passed:

```sh
npm ci                         # 0 npm audit vulnerabilities
npm test                       # cargo tests plus 5 site tests
cargo test --locked            # 8 unit, 4 integration, 1 doctest
npm run test:site              # tsc --noEmit + 5/5 node tests
cargo fmt --check
cargo clippy --all-targets -- -D warnings
npm run build                  # release binary + dist/site
cargo package --locked         # api-example-linter-0.1.0.crate
```

The exact production build produced `dist/bin/api-example-linter` and `dist/site/`. A clean consumer unpacked the generated crate, ran `cargo install --path <unpacked-crate> --root <clean-root> --locked`, then ran `api-example-linter --version` (`0.1.0`) and the normal OpenAPI/Markdown check successfully.

## CLI end-to-end evidence

Independent OpenAPI 3.1 fixtures exercised the brief's smallest useful workflow:

| Case | Result |
| --- | --- |
| Two normal Markdown examples (JSON and separated `--data` curl), mapped to `createPet` | 2 discovered / 2 passed / exit 0 |
| Boundary string at `maxLength: 4` | passed / exit 0 |
| String at length 5 | `SCHEMA_MISMATCH`, pointer `$/name`, exit 1 |
| Stale additional property | GitHub annotation with file/line/pointer, exit 1 |
| Malformed JSON | `INVALID_EXAMPLE`, exit 1 (plus the P2 redundant diagnostic) |
| Shell fence containing `$(touch /tmp/api-linter-e2e/SHOULD_NOT_EXIST)` after a curl body | check passed and marker was not created; curl text was not executed |
| `--mock-base-url https://example.com` | rejected as non-HTTP; no request made |
| `--mock-base-url http://example.com` without allowlist | rejected as disallowed host; no request made |
| `init config.json`, then config-driven check | starter written; normal check passed |
| Missing input with `--format json` | JSON configuration error / exit 2 |

## Live-site evidence

- Desktop and 390×844 mobile were visually inspected. Mobile had no horizontal overflow (`scrollWidth = 390`).
- Keyboard smoke passed: skip link was first focus with a solid visible outline; all visible interactive controls were reachable in order; Enter started the demo and operated Copy (label changed to `Copied`).
- Reduced-motion at 390px displayed the demo's final state immediately after activation. Normal motion advanced the recording one frame at a time. No console errors or page errors were recorded.
- Playwright axe scan: **0 serious/critical violations** on desktop and 390px reduced-motion mobile.
- Runtime requests were only same-origin HTML, self-hosted fonts, JS, CSS, image, favicon, and service worker; no analytics/third-party runtime requests or cookies were observed.
- Service-worker check: `registration.update()` succeeded; after an online reload it controlled the page; cache `api-example-linter-v1` existed; an offline reload returned HTTP 200 from the shell cache, kept `<main>`, and displayed the offline state.
- Lighthouse mobile: Performance **94**, Accessibility **100**, Best Practices **100**, SEO **100**; LCP **2296 ms**, TBT **122 ms**, CLS **0.048**.
- Built initial assets meet size budgets: JS **3,205 B**, CSS **11,965 B**, fonts **88,660 B total**, hero WebP **70,412 B**.

## Required next steps

1. Extend the curl tokenizer to accept `--data=VALUE`, `--data-raw=VALUE`, `--data-binary=VALUE`, and compact short-flag forms without executing the text; add coverage for them and for the malformed-example summary.
2. Configure the actual hosting platform to serve the policies contained in `_headers` (or provide its equivalent): CSP, Permissions-Policy, and one-year immutable caching for hashed static assets. Re-verify live response headers.
3. Replace the ellipsized visible install command with the exact public command.
