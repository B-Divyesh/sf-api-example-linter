# Polish round 2 — review closure map

**Candidate repaired:** `bf275586e2a17a66bf4ebf968f17d16317605aab`  
**Review:** `64f5b2669deedc2d3aae71f55fc336421a9831da`  
**Repair commit:** recorded in the final handoff

No finding is deferred. The live recheck is recorded in the handoff after deployment.

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-1-1 | Kept the plain first-screen job, audience, one-click sample action, secondary install path, and three facts. | `first screen states the job, audience, demo action, and next result`; `.factory/evidence/home-mobile.png`; live cold `/` check. |
| F-1-2 | The Vite build now runs `api-example-linter demo` and embeds its transcript. Every temporary-path occurrence is normalized only for display; the terminal uses text content, not invented HTML frames. The hero now accurately says it opens a recording. | `@claim:demo-transcript-parity`; `.factory/evidence/demo-mobile.png`; live `/demo/?demo=1` transcript check. |
| F-1-3 | Retained the real CLI `demo` command and bundled samples; it creates and removes its own temporary workspace. | Rust `demo_uses_real_linter_and_removes_its_temporary_workspace`; `@claim:demo-temp-isolation`; live demo links to the same command. |
| F-1-4 | Expanded `.factory/claims.json` from 13 to 16 claims. Added transcript parity, mapping metadata/defaults, configuration precedence, all documented curl forms, redirect-safe/no-network curl parsing, explicit host allow-list, exit-code coverage, and copy/registry regression coverage. Removed the unavailable release-binary and verification-bundle promises. | All 16 individual `@claim:` commands from a clean clone; `published behavioral copy is covered by the claims registry`; live demo/privacy copy check. |
| F-1-5 | Retained the static response override and styled 404 recovery page. | `unknown routes return a designed 404 response`; `.factory/evidence/404-desktop.png`; live unknown-route HTTP 404 check. |
| F-1-6 | Retained complete home metadata and original Contract Loom social art. | static metadata suite; live `/` metadata check. |
| F-1-7 | Retained the common header/footer and both legal links on every route. | `every internal link resolves and legal links appear in every footer`; live route crawl. |
| F-1-8 | Retained removal of unsupported five-minute setup wording. | copy-regression test; source search. |
| F-1-9 | Retained the plain h1. | first-screen static and browser checks; live `/` h1 check. |
| F-1-10 | Retained useful section names and the explicit “Restart recording” control. | static copy regression and mobile browser test; live demo control check. |
| F-1-11 | Retained the factual footer line. | shared footer static checks; live footer check. |
| F-1-12 | Retained short README sentences; removed the unavailable release wording. | `.factory/copy-audit.md`; source audit. |
| F-2-1 | Replaced the nonexistent release-binary option with the working source install command. | `published behavioral copy is covered by the claims registry`; `rg 'Download a release binary'` has no result; live README source link check. |
| F-2-2 | Internal route clicks set a same-origin route-focus marker. New pages and Back focus the destination h1 and announce it. | `route navigation and Back focus and announce the destination heading`; live `/ → /demo/ → Back` keyboard check. |
| F-2-3 | All visible links now have a 44×44px minimum touch target, retaining the Contract Loom clipped controls and layout. | `mobile links and buttons meet the 44px touch target baseline`; `.factory/evidence/home-mobile.png`; live 390px check. |
| F-2-4 | Added exact OG image, dimensions, alt text, Twitter image, canonical, and share metadata to demo, legal, and 404 pages. | five-route `has share and canonical metadata` tests; live route metadata check. |
| Verification P1: curl equals/compact forms | Retained and folded every documented data-flag spelling into the tagged supported-input test. | `@claim:supported-inputs`; Rust curl-form regression tests. |
| Verification P1: response headers/cache | Retained Static Web Apps headers and immutable asset route. | `Azure deployment config provides real 404 handling and response policy`; live header check. |
| Verification P2: malformed input duplicate finding | No change required; retained regression behavior. | Rust `malformed_example_does_not_add_a_redundant_no_examples_finding`. |
| Verification P2: ellipsized install command | No change required; retained exact visible and copied command. | `visible install command is the exact executable command copied by the control`; live home check. |

## Local verification before deployment

- `npm test`: 9 Rust unit tests, 7 CLI integration tests, 1 doctest, 17 static-site tests, 16 claim tests, and 6 browser tests passed.
- `npm run build` produced `dist/bin/api-example-linter` and `dist/site`.
- `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings` passed.
- Browser tests produced `.factory/evidence/home-mobile.png`, `.factory/evidence/demo-mobile.png`, and `.factory/evidence/404-desktop.png`.
