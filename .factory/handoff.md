# Review 2 handoff

## Outcome

Adversarial first-read review 2 is complete at commit `bf275586e2a17a66bf4ebf968f17d16317605aab`.

Verdict: **FAIL**. The complete report is `.factory/review-2.md`.

No product code was changed. This work order changes only the review and this handoff.

## Verification performed

- Opened the live site cold at 390×844 and 1440×1000.
- Exercised the one-click demo, banner, Reset demo, Start for real, storage isolation, and live offline reload.
- Recorded all live requests; they were same-origin, with no cookies or console/page errors.
- Ran Playwright axe checks at both sizes with zero WCAG A/AA violations.
- Crawled every site link and checked all route titles, h1/main counts, canonical/OG/Twitter metadata, the designed 404, browser Back, focus, and mobile target sizes.
- Ran `/opt/fleet/lib/verify-url.sh`; the live page passed its baseline checks in 564 ms.
- Cloned the exact candidate to `/tmp/ael-review2-clean.FA2f3E` and ran every one of the 13 `.factory/claims.json` commands separately; all passed.
- From that clean clone, ran `npm test`, `npm run build`, `cargo fmt --check`, and `cargo clippy --all-targets -- -D warnings`; all passed.
- Ran the release CLI demo from a separate temporary directory and confirmed its workspace was removed.
- Confirmed live HTML/JS/CSS hashes match the clean build.
- Rechecked every finding in review 1 plus both earlier verification reports against live behavior and source.

## Open findings

- F-1-2 reopened (blocking): the hard-coded web recording differs from actual CLI output.
- F-1-4 reopened (blocking): visitor-facing behavior remains unlisted or under-tested in the claim registry.
- F-2-1: README offers a release binary, but the repository has no release/tag or download link.
- F-2-2: route changes and Back leave focus on `<body>`.
- F-2-3: several mobile links are smaller than 44×44px.
- F-2-4: demo/legal/404 social metadata is incomplete.

## How to verify the review

```sh
npm ci
npm test
npm run build
cargo fmt --check
cargo clippy --all-targets -- -D warnings
./dist/bin/api-example-linter demo
```

Compare the last command’s exact output with the terminal at <https://api-example-linter.sociobot.in/demo/?demo=1>. Then audit every README and landing claim against `.factory/claims.json` rather than relying only on the current passing test count.
