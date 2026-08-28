# Polish round 1 — all findings closed

**Reviewed candidate:** `62771c6e08379102af327ded04c6c97779ade4f9`  
**Review report:** `76139a736e171649a56b23686414fab898efa345`  
**Repair commits:** `7639146`, `17e6c16`  
**Live site:** <https://api-example-linter.sociobot.in/>  
**Final product deployment:** `50bef5e6-bf9d-434e-9e7a-d1df184b7639`

No finding is deferred.

## Review finding map

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-1-1 | Replaced the metaphor headline with “Lint API examples against OpenAPI.” Added the API-maintainer sentence, a one-click sample action, its exact outcome, a secondary install link, and three plain facts. | `first screen states the job, audience, demo action, and next result`; live 390px cold screenshot `.factory/evidence/live/screenshot-mobile.png`; live h1 and audience assertions passed. |
| F-1-2 | Added one-click `/demo/?demo=1`, root `?demo=1` compatibility, a focused demo route, persistent banner, Reset demo, Start for real, preloaded real CLI output, and isolated `demo:api-example-linter:` session keys. The focused sample output begins in the first 844px viewport. | `@claim:demo-web-isolation`, `@claim:browser-privacy`, `@claim:offline-site`; `mobile first screen, query demo, controls, and keyboard focus work without overflow`; live screenshot `.factory/evidence/live/demo-final-mobile.png`; live `primaryOneClick=true`, `terminalInFirstViewport=true`, `resetIsolated=true`. |
| F-1-3 | Added `api-example-linter demo` and shipped `examples/openapi.yaml` plus `examples/create-pet.md`. Each run creates a fresh temporary folder, runs the real validator, prints one pass and one mismatch, ignores project config, and removes the folder. | Rust integration `demo_uses_real_linter_and_removes_its_temporary_workspace`; `@claim:demo-temp-isolation`; clean-clone output: `2 example(s) checked · 1 passed · 1 failed`. |
| F-1-4 | Added `.factory/claims.json` with 13 retained claims and one uniquely tagged test per claim. Added an individual-test runner so each registry command selects exactly one test. | Every registry command passed independently from `/tmp/api-linter-polish-clean-wDgn4l`; full `npm run test:claims` passed 13/13. |
| F-1-5 | Added the Contract Loom `404.html`, removed the catch-all landing fallback, and configured `responseOverrides.404` with a 404 status. | `unknown routes return a designed 404 response`; live `GET /not-a-real-page` returned 404 with h1 “Page not found”; screenshot `.factory/evidence/404-desktop.png`. |
| F-1-6 | Set the home title to “API Example Linter — Lint API examples”. Added unique route titles, canonical URLs, descriptions, Open Graph/Twitter fields, a 1200×630 Contract Loom image, and a 180px touch icon. | Static metadata tests for all four public routes; live verifier reports the exact home title; built assets `contract-loom-social.png` and `apple-touch-icon.png`. |
| F-1-7 | Applied one header/footer skeleton to home, demo, privacy, terms, and 404. Every footer includes Privacy, Terms, Source, Param Factory, and the version. | `every internal link resolves and legal links appear in every footer`; live privacy/terms checks returned 200 with both legal links. |
| F-1-8 | Replaced “Five-minute setup” and “under five minutes” with “CI setup”. | Static first-screen/copy regression test; repository search finds neither removed phrase. |
| F-1-9 | Replaced the hero slogan with the required job headline. | Static exact-copy assertion and live h1 assertion. |
| F-1-10 | Replaced mood/jargon labels with “How it works”, “How API example checks work”, “Choose a schema or operation”, “Sample lint result”, “Supported input formats”, and “Add the linter to CI”. Renamed “Restart” to “Restart recording”. | `.factory/copy-audit.md`; static test rejects the old phrases; mobile browser test operates “Restart recording”. |
| F-1-11 | Replaced the footer slogan with “Checks documentation examples against OpenAPI.” | Shared-footer static assertions and live page screenshots. |
| F-1-12 | Rewrote the README into short single-idea sentences and lists. The three reported long sentences are gone. | `.factory/copy-audit.md`; banned-word search returned no matches; README documents demo, install, use, privacy, verification, scope, and license. |

## Earlier findings protected

| Earlier report item | Regression evidence |
| --- | --- |
| Equals and compact curl data flags | Rust `conventional_curl_data_equals_is_discovered_and_validated` and unit `extracts_equals_and_compact_curl_data_forms_without_shelling_out` passed. |
| Redundant `NO_EXAMPLES` after malformed JSON | Rust `malformed_example_does_not_add_a_redundant_no_examples_finding` passed. |
| Missing CSP, Permissions-Policy, and immutable asset caching | Live responses include the restrictive CSP, Permissions-Policy, `nosniff`, `no-referrer`, and `max-age=31536000, immutable` on hashed assets. |
| Ellipsized install command | `visible install command is the exact executable command copied by the control` passed. |

## Verification evidence

- Final fresh clone: `/tmp/api-linter-polish-final-cWyFOt`.
- Every `.factory/claims.json` command: 13/13 passed separately.
- Full clean-clone suite: 9 Rust unit, 7 CLI integration, 1 doctest, 15 site, 13 claim, and 4 browser tests passed.
- `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `npm run build`, and `cargo package --locked` passed.
- Package: 19 files, 129.1 KiB unpacked, 33.4 KiB compressed.
- Live verifier: 641 ms load, no console errors, one h1, main landmark, `lang=en`, no missing alt text, and no unlabeled buttons.
- Live axe: zero serious or critical WCAG A/AA findings.
- Live privacy: zero third-party requests and zero cookies through the cold demo flow.
- Live offline: the visited demo reloaded with the browser offline.
- Live Lighthouse mobile: Performance 100, Accessibility 100, Best Practices 100, SEO 100; LCP 1.8 s, TBT 30 ms, CLS 0.
- Built initial assets: JavaScript 4.70 KB, CSS 14.44 KB, fonts 88.66 KB, hero WebP 70.41 KB.
- Final live hashes match `dist/site`: HTML `72f3ba4727dd6a3e98625c4b305f6800ea99626fe831aeb1c0a1d7b5fa43912f`; JS `e0f5d41d37da11f59e42d99fa03b64c567dc4c53de76a5f21807c54389ea501a`; CSS `831862ad6f8faacee89e3ff2cd4e966a14771918da1e99a5557c8f739999c734`.

## Result

All blocking and minor findings from review 1 are resolved.
All earlier repaired defects remain fixed.
There are no open acceptance findings.
