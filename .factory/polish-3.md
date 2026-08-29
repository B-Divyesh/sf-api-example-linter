# Polish round 3 — complete closure map

**Candidate repaired:** `32f161659ea2eae2a1ab659d23f39001ad6846c5`  
**Review reports read:** `review-1.md`, `review-2.md`, and `review-3.md`  
**Repair commit:** `eeeb8d75d19ea6e9954eb4f577073c3b901f35db`  
**Live site:** <https://api-example-linter.sociobot.in/>

No blocking or minor finding is deferred. Earlier changes were retained and reverified instead of being assumed closed.

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-1-1 | Retained the plain job h1, API-maintainer audience sentence, first-screen sample action, result note, install secondary action, and three facts. | `first screen states the job, audience, demo action, and next result`; `.factory/evidence/live-polish-3/home-mobile-cold.png`; cold <https://api-example-linter.sociobot.in/> check at 390px. |
| F-1-2 | Retained the one-click `/demo/?demo=1` sandbox, banner, reset/start controls, immediate bundled result, and build-generated CLI transcript. | `@claim:demo-transcript-parity`, `@claim:demo-web-isolation`, `@claim:browser-privacy`; `.factory/evidence/live-polish-3/demo-mobile-cold.png`; cold <https://api-example-linter.sociobot.in/demo/?demo=1> check. |
| F-1-3 | Retained `api-example-linter demo`, realistic `examples/` inputs, temporary workspace cleanup, and configuration isolation. | Rust `demo_uses_real_linter_and_removes_its_temporary_workspace`; `@claim:demo-temp-isolation`; live demo transcript check. |
| F-1-4 | Retained the 16-entry claims registry and its one-tag-per-claim browser/CLI tests. | Every exact `.factory/claims.json` command passed independently from `/tmp/api-example-linter-polish3-clean-aPbOWl`; full `npm test` passed. |
| F-1-5 | Retained the styled `404.html` and Static Web Apps response override. | `unknown routes return a designed 404 response`; `.factory/evidence/live-polish-3/404-desktop-cold.png`; live `/no-such-page` returned HTTP 404. |
| F-1-6 | Retained plain route titles, canonical/description/OG/Twitter metadata, local social art, favicon, and Apple touch icon. | five `has share and canonical metadata` tests; live cold metadata check on `/`, `/demo/`, `/privacy/`, `/terms/`, and `/404.html`. |
| F-1-7 | Retained one header/footer skeleton and both Privacy and Terms links on all routes. | `every internal link resolves and legal links appear in every footer`; live route crawl. |
| F-1-8 | Retained removal of unsupported five-minute setup wording. | `published behavioral copy is covered by the claims registry`; source audit has no five-minute wording. |
| F-1-9 | Retained the factual hero heading. | `first screen states the job, audience, demo action, and next result`; live home h1 check. |
| F-1-10 | Retained informative section names and the explicit Restart recording control. | static copy assertions; cold demo control check. |
| F-1-11 | Retained the factual footer one-liner. | shared-footer static checks; live footer inspection. |
| F-1-12 | Retained concise README and landing prose, terminology table, and copy audit. | `.factory/copy-audit.md`; static copy regression test. |
| F-2-1 | Retained only the working source install path; no nonexistent release-binary option remains. | `published behavioral copy is covered by the claims registry`; source audit. |
| F-2-2 | Retained destination-h1 focus and added exact route-announcement assertions, including browser Back. | `route navigation and Back focus and announce the destination heading`; live `/ → /demo/ → Back` exact-announcement check. |
| F-2-3 | Retained 44px link/button targets across the responsive routes. | `mobile links and buttons meet the 44px touch target baseline`; 390px cold live check. |
| F-2-4 | Retained full share metadata on Demo, legal, and 404 routes. | five `has share and canonical metadata` tests; live cold route metadata checks. |
| Verification P1 — curl forms | Retained separated, equals, and compact data-flag parsing without executing curl. | `@claim:supported-inputs`; Rust `conventional_curl_data_equals_is_discovered_and_validated`. |
| Verification P1 — headers/cache | Retained CSP, no-referrer, nosniff, permissions policy, immutable asset caching, and 404 deployment policy. | `Azure deployment config provides real 404 handling and response policy`; live response-header check. |
| Verification P2 — malformed input | Retained the no-duplicate-`NO_EXAMPLES` behavior. | Rust `malformed_example_does_not_add_a_redundant_no_examples_finding`. |
| Verification P2 — install command | Retained the complete visible command and exact Copy command content. | `visible install command is the exact executable command copied by the control`. |
| F-3-1 | Changed `focusRouteHeading` to announce `Navigated to ${heading text}` without manufacturing another `.`. Added exact equality assertions for forward and Back navigation. | `route navigation and Back focus and announce the destination heading`; live cold `/ → /demo/ → Back` check announced `Navigated to Run the bundled linter sample.` and `Navigated to Lint API examples against OpenAPI.` exactly. |

## Final evidence

- Clean clone at `eeeb8d75d19ea6e9954eb4f577073c3b901f35db`: `npm ci`, `npm test`, `npm run build`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo package --locked` all passed.
- Every one of the 16 registry commands passed independently from the clean clone.
- Production verification evidence is under `.factory/evidence/live-polish-3/`; the cold mobile, demo, and 404 screenshots are `home-mobile-cold.png`, `demo-mobile-cold.png`, and `404-desktop-cold.png`.
- Live Lighthouse mobile scores: 100 Performance, 100 Accessibility, 100 Best Practices, and 100 SEO; LCP 1.53s and CLS 0.00042.
