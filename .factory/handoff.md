# API Example Linter — polish round 1 handoff

## Status

Polish round 1 is complete.
Every finding in `.factory/review-1.md` is fixed, tested, pushed, deployed, and checked cold on the live domain.
No earlier verified repair regressed.

- Live site: <https://api-example-linter.sociobot.in/>
- One-click sample: <https://api-example-linter.sociobot.in/demo/?demo=1>
- Product commits: `7639146`, `17e6c16`
- Deployment: `50bef5e6-bf9d-434e-9e7a-d1df184b7639`

## What changed

- Added `api-example-linter demo` with two bundled examples in `examples/`.
- The demo runs the real validator in a fresh temporary folder and removes it afterward.
- Added the isolated browser demo route, banner, reset, exit, offline behavior, and demo-only session namespace.
- Rewrote the first screen and all reviewed jargon in plain words.
- Added `.factory/claims.json` with 13 individually selectable claim tests.
- Added unique route metadata, canonical links, social art, a touch icon, consistent legal links, and a true 404 response.
- Preserved and extended the Contract Loom visual system across demo, legal, mobile, and 404 states.
- Added browser, axe, privacy, offline, keyboard, mobile, routing, link, metadata, and response-policy tests.
- Rewrote README documentation and added `.factory/demo.md`, `.factory/copy-audit.md`, and the ≤120-character catalog description.

The complete finding-to-change-to-evidence map is in `.factory/polish-1.md`.

## Run and verify

```sh
npm ci
npm test
npm run build
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo package --locked
```

Run one claim exactly as the registry does:

```sh
npm run test:claims -- --test-name-pattern=@claim:demo-temp-isolation
```

Try the shipped CLI sample:

```sh
./dist/bin/api-example-linter demo
```

The build writes the binary to `dist/bin/api-example-linter`.
It writes the deployable static site to `dist/site`.

## Exact verification

Fresh clone `/tmp/api-linter-polish-clean-wDgn4l` passed:

- all 13 claim commands separately;
- 9 Rust unit tests;
- 7 CLI integration tests;
- 1 Rust doctest;
- 15 static site tests;
- 13 full claim tests;
- 4 browser suites;
- Rust formatting and clippy with warnings denied;
- release build and Cargo package verification.

`cargo package --locked` produced 19 files.
The package is 129.0 KiB unpacked and 33.4 KiB compressed.

The final live cold check passed:

- primary sample action reached `/demo/?demo=1` in one click;
- banner, seeded result, Reset demo, and Start for real were present;
- terminal output began within the 390×844 first viewport;
- Reset demo removed only demo session keys;
- unknown routes returned HTTP 404 with “Page not found”;
- home, demo, privacy, and terms had unique titles, one h1, and one main;
- no horizontal overflow, console errors, cookies, or third-party requests;
- visited demo reloaded offline;
- zero serious or critical axe findings.

The factory verifier loaded the live home page in 641 ms with no errors.
Live Lighthouse mobile scored 100 in Performance, Accessibility, Best Practices, and SEO.
LCP was 1.8 seconds, TBT was 30 ms, and CLS was 0.

Initial built assets are 4.70 KB JavaScript, 14.44 KB CSS, 88.66 KB fonts, and a 70.41 KB hero image.
Live HTML, JavaScript, and CSS SHA-256 values exactly match `dist/site`.

## Evidence

- Mobile home: `.factory/evidence/live/screenshot-mobile.png`
- One-click demo first viewport: `.factory/evidence/live/demo-final-mobile.png`
- Desktop home: `.factory/evidence/live/screenshot-desktop.png`
- Designed 404: `.factory/evidence/404-desktop.png`
- Verifier report: `.factory/evidence/live/verify.json`
- Lighthouse report: `.factory/evidence/live/lighthouse-final.json`

Evidence files are intentionally ignored by Git and remain in the worktree.

## Open findings and next steps

There are no open review or acceptance findings.
Remote references and production-host probing remain explicit product boundaries, not unfinished work.
Registry publication remains a factory operation; the verified Cargo package is ready.
