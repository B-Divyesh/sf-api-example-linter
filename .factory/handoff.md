# Review 3 handoff

## Outcome

This was a review-only work order. No product code was changed. `.factory/review-3.md` records a strict **FAIL** with one minor remaining issue: route announcements add a second terminal full stop when the destination h1 already has one.

## Verification completed

- Cold live checks at 390×844 and 1440×1000 confirmed the first screen states the job, audience, first action, and result.
- Live demo checks confirmed one-click seeded output, banner, reset isolation, Start-for-real cleanup, real CLI transcript parity, same-origin-only requests, no cookies, and offline reload after the first visit.
- Live forward and Back navigation correctly focused destination h1 elements. The exact announcement punctuation is the outstanding issue.
- All discovered internal and GitHub links returned HTTP 200; an unknown route returned the designed HTTP 404.
- From fresh clone `/tmp/api-example-linter-review3.6poxGQ`, `npm test` passed (9 Rust unit, 7 CLI integration, 1 doctest, 17 site, 16 claim, 6 browser tests). Every one of the 16 `.factory/claims.json` commands also passed independently.

## To verify or close the finding

```sh
npm ci
npm test
npm run build
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```

Fix `site/src/main.ts:121` so the live announcement is, for example, `Navigated to Run the bundled linter sample.` rather than `...sample..`, and add an exact announcement assertion to the browser test. Rerun the full review afterward.

## Known gap

`F-3-1` in `.factory/review-3.md` remains open. No other finding was identified.
