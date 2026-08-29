# Polish round 3 handoff

## Outcome

Repair commit `eeeb8d75d19ea6e9954eb4f577073c3b901f35db` fixes the final review finding. Route announcements now use the already-punctuated destination heading instead of appending a second full stop. The production route and browser-Back checks assert the exact accessible announcement.

The static site was built with the configured command, deployed through `/opt/fleet/lib/deploy-static.sh api-example-linter dist/site`, and rechecked cold at <https://api-example-linter.sociobot.in/>.

## Verification

- Clean clone: `/tmp/api-example-linter-polish3-clean-aPbOWl`, cloned from `origin/main` at `eeeb8d75d19ea6e9954eb4f577073c3b901f35db`.
- `npm ci`, `npm test`, `npm run build`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo package --locked` passed from that clone.
- `npm test` passed 9 Rust unit tests, 7 CLI integration tests, 1 doctest, 17 static-site tests, 16 claim tests, and 6 browser tests.
- All 16 exact commands listed in `.factory/claims.json` passed independently from the clean clone.
- Live factory URL verification: HTTP 200; title, `lang`, one h1, main landmark, image alt text, and labelled controls all passed with no console errors. Evidence: `.factory/evidence/live-polish-3/verify.json`.
- Live cold Playwright recheck passed at 390px and desktop: first-screen copy, one-click `/demo/?demo=1`, completed real transcript, demo reset isolation, same-origin-only requests, no cookies/localStorage, 390px layout, route focus and one-period announcements, metadata, all-route axe checks, real 404, and offline demo reload. Evidence: `.factory/evidence/live-polish-3/home-mobile-cold.png`, `demo-mobile-cold.png`, and `404-desktop-cold.png`.
- Lighthouse mobile report: Performance 100, Accessibility 100, Best Practices 100, SEO 100; LCP 1,532ms and CLS 0.00042. Evidence: `.factory/evidence/live-polish-3/lighthouse-final.json`.

## How to verify

```sh
npm ci
npm test
npm run build
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo package --locked
```

Open <https://api-example-linter.sociobot.in/demo/?demo=1>, then use Reset demo and Start for real. The route change announcement is covered by `route navigation and Back focus and announce the destination heading` in `site/tests/browser.test.mjs`.

## Known gaps

None. Every finding in reviews 1–3 and their verification reports remains covered; `.factory/polish-3.md` maps each to current evidence.
