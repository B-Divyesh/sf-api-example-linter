# Independent verification 2 — PASS

**Work order:** `api-example-linter-verify-2`  
**Candidate:** `66775269074677b0b16ab8c4c2826c759c4a6175` (`main`)  
**Live URL:** <https://api-example-linter.sociobot.in/>  
**Verified:** 2026-08-28 UTC  
**Scope:** Fresh clean-checkout verification of the CLI package, production build, and the live static site. Product source was not changed.

## Verdict

**PASS — candidate is releasable.** The previous report's curl-equals extraction, misleading malformed-example annotation, live security-policy/cache, and visible install-command findings are fixed. The exact production build is the content served by the live URL.

No release-blocking or non-blocking defects were found.

## Clean-checkout gates

All commands below passed from this checkout at the candidate SHA:

```sh
npm ci                         # 0 vulnerabilities
npm test                       # 9 Rust unit + 6 CLI integration + 1 doctest + 7 site tests
cargo test --locked
cargo fmt --check
cargo clippy --all-targets -- -D warnings
npm run build                  # release CLI -> dist/bin; static site -> dist/site
cargo package --locked         # 17 files, 122.1 KiB unpacked / 31.9 KiB compressed
```

The shipped static assets are within the stated budgets: JavaScript 3,205 B, CSS 11,965 B, self-hosted fonts 88,660 B total, and hero WebP 70,412 B. Mobile Lighthouse (live URL, Chromium remote-debug session) measured Performance **98**, Accessibility **100**, Best Practices **100**, SEO **100**; LCP 1,674 ms, TBT 145 ms, CLS 0.

## CLI/package end-to-end evidence

A clean consumer unpacked `target/package/api-example-linter-0.1.0.crate`, installed it with:

```sh
cargo install --path <unpacked-crate> --root <clean-root> --locked
```

Its public binary reported `api-example-linter 0.1.0`, showed useful `--help`, passed the documented Markdown/OpenAPI check (`2 discovered / 2 passed`), wrote a starter config, and returned exit `2` without overwriting it on a second `init`.

Independent OpenAPI 3.1 fixtures exercised the acceptance workflow:

| Case | Evidence |
| --- | --- |
| Normal JSON plus `--data='…'` and compact `-d'…'` curl bodies | 3 discovered / 3 passed / exit 0 |
| Boundary `name` at maximum length 4 | passed |
| Length 5 | exit 1; `SCHEMA_MISMATCH` at `$/name` |
| Stale additional property | exit 1; line-level GitHub Actions annotation at `$/unexpected` |
| Malformed JSON | exit 1; one actionable `INVALID_EXAMPLE`, with no redundant `NO_EXAMPLES` |
| Shell fence with `$(touch …)` after curl | exit 0; marker was not created (curl text was never executed) |
| Disallowed `http://example.com` mock host | exit 1; explicit allow-host error and no request |
| HTTPS mock URL | exit 1; rejected as intentionally unsupported |
| Local `http://127.0.0.1:4010` POST mock | 3 discovered / 3 passed / exit 0 |
| Missing input with JSON format | exit 2 with structured `CONFIGURATION_ERROR` |

## Live deployment, privacy, and browser evidence

The SHA-256 values of built and live content match exactly:

| Resource | SHA-256 |
| --- | --- |
| `index.html` | `5d5307e3e334fc466a6f5cff2ca206c7f286a3cef00fecc6e76816f62e7034c2` |
| `assets/index-BnFUFOZU.js` | `423e930f7700b46cc43b4c8681d4c04a3dbe32265d8b4192c06656be7571175b` |
| `assets/style-DbmZ2ZSH.css` | `90e3e44caf036ce28a8b0854db0f8fa254e296e4f615a27a1b5b40b08df4fb9b` |

Live HTML is HTTP 200 with HSTS, `X-Content-Type-Options: nosniff`, `Referrer-Policy: no-referrer`, `Permissions-Policy: camera=(), microphone=(), geolocation=()`, and a restrictive same-origin CSP. Hashed JavaScript and CSS return `Cache-Control: public, max-age=31536000, immutable`; HTML is short-cached. `/privacy`, `/terms`, service worker, manifest, and robots all returned HTTP 200. No `Set-Cookie` header, analytics, third-party runtime request, or third-party font/script was observed; browser requests were same-origin only.

Playwright checks on 1440px desktop and 390×844 mobile passed:

- title, `lang=en`, exactly one `<h1>`, one `<main>`, and meaningful image alt text present;
- first keyboard focus is the skip link with a designed coral 3px outline on desktop; all tested controls worked with Enter, including Copy and demo playback;
- mobile document/body `scrollWidth` is exactly 390px; visually inspected screenshots show intentional single-column stacking;
- reduced motion immediately reaches the final demo state; no page errors or console errors;
- axe-core WCAG A/AA scan found **0 serious/critical** violations at both widths;
- service-worker `registration.update()` succeeded, it controlled the reloaded page and used cache `api-example-linter-v1`; offline reload retained `<main>` and displayed the offline status.

## Handoff

Ready to release. The factory remains responsible for registry publication and deployment promotion; no product-source or deployment change was made by this verification.
