# Adversarial first-read review 1 — FAIL

**Reviewed:** 2026-08-28 UTC  
**Live URL:** <https://api-example-linter.sociobot.in/>  
**Commit reviewed:** `62771c6e08379102af327ded04c6c97779ade4f9`

## Verdict

**FAIL.** There are blocking findings. The CLI itself passes its existing checks, but the product is not honestly tryable through the required sample sandbox, its visitor-facing claims have no claim registry or claim tests, and the live site's routing has no real 404.

## Cold first read

I opened a fresh Chromium context at 390×844 and 1440×1000 before scrolling. The first mobile viewport has no horizontal overflow (390px document width), but it does not answer all three required questions.

- **What it does:** I can infer that it checks JSON and curl documentation snippets against an OpenAPI schema, but the headline itself does not say that.
- **For whom:** I cannot identify API maintainers from the first screen. The only audience cue is the unexplained phrase “CI gate.”
- **What to click first:** The primary action is “Add the CI gate,” which only scrolls to setup. There is no “Try it with sample data” action or adjacent explanation of the result.

The exact first-screen text that fails the test is “Catch the example your schema left behind.” and “Add the CI gate.” The former is a metaphor; the latter asks for installation before showing the result.

## Findings

### F-1-1 — BLOCKING — first screen does not state the job, audience, or first try-out action

**Location / quote:** landing hero, `site/index.html:33-42`: “Contract-aware documentation”; “Catch the example your schema left behind.”; “Add the CI gate”.

**Why this fails:** A cold mobile visitor is not told this is for API maintainers, nor that it validates copied examples against OpenAPI. The primary action skips the required try-before-install path.

**Concrete fix:** Use headline **“Lint API examples against OpenAPI.”** Use the supporting sentence **“For API maintainers whose copied JSON or curl examples drift from their OpenAPI contract.”** Make the primary control **“Try it with sample data”** and place **“Runs the included failing API example in a temporary folder.”** beside it. Keep installation as the secondary action.

### F-1-2 — BLOCKING — no one-click demo sandbox exists

**Location / quote:** landing hero has no sample action; `/demo` returns the landing page; `site/index.html:70-91` labels a static terminal as “Recorded run”.

**Evidence:** In a fresh browser context, `https://api-example-linter.sociobot.in/demo` returned HTTP 200 but rendered the unchanged landing headline, an empty terminal (“Ready to lint 2 examples.”), no sample output, no “Demo — sample data, nothing is saved” banner, no Reset control, and no Start-for-real control. The only demo interaction is an opt-in animation of hard-coded frames.

**Why this fails:** The required first click does not run the real job with realistic sample data. A recording may supplement a CLI demo but cannot replace the documented `tool demo`/`--demo` workflow and isolated sample output.

**Concrete fix:** Ship `api-example-linter demo` (or `--demo`) that copies bundled `examples/` inputs to a fresh temporary directory, runs the real binary, and prints the directory and result. Add `/demo` as a real page containing the recording of that exact command, a visible sandbox banner, Reset, Start for real, and the sample command. Add `.factory/demo.md` documenting the URL/command, samples, reset behavior, and storage isolation. Test that demo mode neither reads nor writes real user paths.

### F-1-3 — BLOCKING — the CLI has no demo command or bundled demo directory

**Location / quote:** `api-example-linter --help` lists only `check`, `init`, and `help`; `api-example-linter demo` returns “error: unrecognized subcommand 'demo'” with exit 2. The repository contains `fixtures/`, but no `examples/` directory or documented demo entry point.

**Why this fails:** This is the required executable demo path for this artifact class. A first-time evaluator cannot run the advertised task without providing their own OpenAPI and Markdown files.

**Concrete fix:** Add a deterministic `demo` subcommand backed by shipped, opinionated example Markdown and OpenAPI files. Cover its output, temporary-directory location, and no-write behavior in an integration test.

### F-1-4 — BLOCKING — no claim registry or claim tests exist

**Location / quote:** `.factory/claims.json` is absent. The landing and README make observable claims, including “curl is parsed, never run”, “Nothing is uploaded”, “MIT licensed · No telemetry”, “It has no analytics, external scripts, hosted fonts, cookies, or data collection”, and “Remote `$ref` fetching is intentionally disabled for deterministic, offline-safe CI.”

**Evidence:** There were zero listed claim tests to run from the clean clone because the required registry does not exist. Existing `npm test` passed, but none of its seven site tests is tagged or mapped to a visitor-facing claim.

**Why this fails:** A visitor is asked to rely on privacy, offline, safety, format, and CI behavior that cannot be audited through the required claim contract. Every claim-like sentence below is currently an unlisted claim.

**Concrete fix:** Create `.factory/claims.json`. Give every retained claim one `@claim:<id>` test that starts from the shipped demo data. At minimum cover shell non-execution, JSON/curl/OpenAPI extraction, GitHub output, no third-party browser requests, service-worker offline reload, and demo temporary-path isolation. Remove claims that cannot be tested.

### F-1-5 — BLOCKING — unknown URLs render the landing page instead of a designed 404

**Location / quote:** `https://api-example-linter.sociobot.in/no-such-page` and `/404` both return HTTP 200 and the h1 “Catch the example your schema left behind.” `staticwebapp.config.json` has only a navigation fallback; there is no 404 document or `responseOverrides` entry.

**Why this fails:** A mistyped deep link looks like a successful page and silently loses the requested location. This is broken routing, not a designed recovery route.

**Concrete fix:** Add a product-styled `404.html` with a plain h1 such as **“Page not found”**, a home link, and the same header/footer. Configure a true 404 response override without combining `rewrite` and `statusCode` in a route. Add an automated HTTP-status and browser-content test.

### F-1-6 — MINOR — metadata is incomplete and the home title is not a plain statement of the job

**Location / quote:** `site/index.html:7-10` has title, description, favicon, and manifest, but no canonical URL, Open Graph fields, Twitter card, or 180px Apple touch icon. Its title is “API Example Linter — Keep docs examples executable”.

**Why this fails:** Shared links have no product art/description, and “Keep docs examples executable” is a slogan rather than the clearest description of the job.

**Concrete fix:** Set the home title to **“API Example Linter — Lint API examples”**; add canonical, OG, and Twitter title/description/image fields; create a real 1200×630 derivative of the existing Contract Loom asset; add an Apple touch icon. Check each static route separately.

### F-1-7 — MINOR — header/footer structure is inconsistent across routes

**Location / quote:** the home footer exposes both “Privacy” and “Terms” (`site/index.html:128`); the privacy footer only exposes “Terms”; the terms footer only exposes “Privacy”. The legal-page header drops the home page’s “How it works”/“Demo” navigation.

**Why this fails:** The skeleton changes between pages and users cannot always reach both required legal pages from the footer.

**Concrete fix:** Use one shared header and footer on all routes. Retain a compact, consistent navigation and include both Privacy and Terms in every footer.

### F-1-8 — MINOR — unsupported five-minute setup promise

**Location / quote:** `site/index.html:109`: “Five-minute setup”; `README.md:75`: “keep CI under five minutes”.

**Why this fails:** This is a quantitative claim with neither a defined start/end nor a claim test.

**Concrete fix:** Either measure and claim the defined task in `.factory/claims.json`, or replace both with **“CI setup”**.

### F-1-9 — MINOR — informationless hero headline

**Location / quote:** `site/index.html:34`: “Catch the example your schema left behind.”

**Why this fails:** It is a metaphor and does not name the section or explain the tool to a screen-reader heading list or a cold visitor.

**Concrete fix:** **“Lint API examples against OpenAPI.”**

### F-1-10 — MINOR — jargon/mood labels conceal useful section names

**Location / quote:** “Contract-aware documentation” (`site/index.html:33`), “The shortest path to trustworthy docs” (59), “Examples enter the contract.” (60), “Align” (65), “See stale become actionable.” (73), “Small surface, useful coverage” (97), “The formats between the cracks.” (98), and “Put copied examples on the build path.” (110). The button “Restart” (81) also omits what will restart.

**Why this fails:** These labels use unexplained jargon or metaphor rather than saying what a visitor will find in the section.

**Concrete fix:** Replace them respectively with **“OpenAPI example checks”**, **“How it works”**, **“How API example checks work”**, **“Choose a schema or operation”**, **“Sample lint result”**, **“Supported input formats”**, **“Supported input formats”**, and **“Add the linter to CI”**. Rename the control **“Restart recording”**.

### F-1-11 — MINOR — non-informative slogan in the footer

**Location / quote:** `site/index.html:127`: “Examples should compile, too.”

**Why this fails:** It could describe many developer tools and does not explain this product.

**Concrete fix:** **“Checks documentation examples against OpenAPI.”**

### F-1-12 — MINOR — README sentences exceed the 22-word cap and combine multiple ideas

**Location / quote:** README sentence audit below: line 3 (35 words), line 51 (24), and line 112 (37).

**Why this fails:** The first documentation paragraph is dense before the reader reaches a working command; the long enumerations also obscure the precise supported boundary.

**Concrete fix:** Split the line-3 sentence into function and output sentences; split line 51 into extraction and supported-flag sentences; turn line 112 into a short list of supported schema keywords plus one sentence about local references.

## Claims and sandbox check

| Check | Result | Evidence |
| --- | --- | --- |
| Claim registry | **FAIL** | `.factory/claims.json` does not exist. |
| Listed claim tests from clean clone | Not runnable | There are no listed tests. |
| Browser request log, fresh 390px context | Same-origin only | Landing loaded only the page, self-hosted fonts, JS, CSS, hero image, and `/demo`; no third-party request occurred. This is evidence, not a substitute for a claim test. |
| Demo storage namespace | **FAIL** | `/demo` has no demo implementation, banner, reset, or real-data boundary to inspect. The observed local/session storage was empty. |
| CLI demo in temporary directory | **FAIL** | `api-example-linter demo` is unrecognized (exit 2). |

The no-upload/no-telemetry and offline copy is not accepted as verified until the registry maps it to request-log/offline tests.

## Earlier-report regression check

No earlier `.factory/review-*.md` or `.factory/polish-*.md` files exist. I read `verification.md`, `verification-2.md`, and the prior handoff. Each previously reported defect is actually fixed:

| Earlier finding | Verification in this review |
| --- | --- |
| `curl --data=VALUE` was not extracted | The clean-clone test suite includes `conventional_curl_data_equals_is_discovered_and_validated` and passed. `src/lib.rs` includes equals/compact extraction coverage. |
| Malformed input emitted redundant `NO_EXAMPLES` | The clean-clone test `malformed_example_does_not_add_a_redundant_no_examples_finding` passed. |
| Live CSP/Permissions-Policy/cache mismatch | Live HTML and hashed JS return the configured restrictive CSP, Permissions-Policy, and immutable JS cache header. |
| Visible install command was ellipsized | The live visible command and Copy value are the same complete Git URL. |

These fixes are not regressed. The findings in this report are new first-read, demo, claims, routing, and copy failures.

## Structural, accessibility, and link checks

| Check | Result |
| --- | --- |
| `lang`, one h1, one main, favicon, meta description, keyboard skip link, 390px width | Pass on the home page. |
| Title pattern | Partial: legal titles fit; home title uses a vague slogan (F-1-6). |
| Canonical, OG/Twitter image/card, Apple touch icon | Fail (F-1-6). |
| `/privacy`, `/terms`, `/robots.txt`, `/sitemap.xml`, manifest | HTTP 200. |
| Crawled live links | Home, privacy, terms, GitHub source, and GitHub license returned 200; in-page anchors resolve. |
| `/demo` deep link | HTTP 200 but is the ordinary landing page rather than a demo (F-1-2). |
| Unknown route/404 | Fail (F-1-5). |
| Visual identity | Pass: the Contract Loom illustration, drafting-paper grid, and clipped controls are product-specific rather than a stock SaaS hero. |
| AI leverage | No finding. The brief describes deterministic local validation; an AI action would not be an obvious or honest improvement. |

## Copy audit — landing page

Word counts treat hyphenated and code-like terms as one word. Navigation, buttons, and labels are included so that no visible copy escapes review.

| Location | Copy | Words | Flag |
| --- | --- | ---: | --- |
| skip link | Skip to content | 3 | — |
| offline state | You’re offline. | 2 | Claim; F-1-4 |
| offline state | The guide and demo still work; install commands need a connection. | 11 | Claim; F-1-4 |
| wordmark | API Example Linter | 3 | — |
| nav | How it works | 3 | — |
| nav | Demo | 1 | Misleading destination; F-1-2 |
| nav | GitHub | 1 | — |
| hero eyebrow | Contract-aware documentation | 2 | Jargon; F-1-10 |
| hero h1 | Catch the example your schema left behind. | 7 | Metaphor; F-1-9 |
| hero lede | Lint the JSON and curl snippets humans actually copy. | 9 | Claim; F-1-4 |
| hero lede | One small CI gate connects Markdown, OpenAPI, and clear line-level fixes. | 11 | Claim/jargon; F-1-4 |
| hero button | Copy | 1 | Result is implied by adjacent command; acceptable |
| hero button | Add the CI gate | 4 | Not the required try-out action; F-1-1 |
| hero link | Watch a stale field fail | 5 | Recording only; F-1-2 |
| art caption | Input / docs | 2 | — |
| art caption | Gate / OpenAPI | 2 | — |
| art caption | Output / trusted | 2 | Vague claim; F-1-4 |
| proof strip | Shell-safe curl is parsed, never run | 5 | Claim/adjective; F-1-4 |
| proof strip | Offline-first remote refs stay off | 5 | Claim/jargon; F-1-4 |
| proof strip | CI-native precise annotations | 3 | Claim/jargon; F-1-4 |
| how eyebrow | The shortest path to trustworthy docs | 6 | Unsupported/mood heading; F-1-10 |
| how h2 | Examples enter the contract. | 4 | Metaphor; F-1-10 |
| how text | No fixture migration. | 3 | Claim; F-1-4 |
| how text | Point the linter at the documentation you already maintain and choose the operation or schema that gives each block meaning. | 20 | Claim; F-1-4 |
| step h3 | Extract | 1 | — |
| step text | Find fenced JSON and curl bodies in Markdown, plus examples embedded in OpenAPI 3.x. | 14 | Claim; F-1-4 |
| step h3 | Align | 1 | Jargon; F-1-10 |
| step text | Resolve the operation or named schema, including local references and request/response direction. | 11 | Claim/jargon; F-1-4 |
| step h3 | Explain | 1 | — |
| step text | Return the file, line, JSON pointer, and exact mismatch in text, JSON, or GitHub format. | 15 | Claim; F-1-4 |
| demo eyebrow | Recorded run | 2 | — |
| demo h2 | See stale become actionable. | 4 | Mood heading; F-1-10 |
| demo text | This recording uses the real CLI output contract. | 8 | Claim/jargon; F-1-4 |
| demo text | Nothing is uploaded, and playback begins only when you ask. | 10 | Privacy claim; F-1-4 |
| demo button | Play recording | 2 | Result-naming; acceptable |
| demo button | Restart | 1 | Does not name its result; F-1-10 |
| demo empty state | Ready to lint 2 examples. | 5 | Static recording, not sample result; F-1-2 |
| demo state | Recording stopped at the first frame. | 6 | — |
| formats eyebrow | Small surface, useful coverage | 4 | Vague adjective/mood; F-1-10 |
| formats h2 | The formats between the cracks. | 5 | Metaphor; F-1-10 |
| format h3 | Fenced JSON | 2 | — |
| format text | Validate literal request and response payloads, with optional metadata right on the fence. | 12 | Claim; F-1-4 |
| format h3 | Safe curl | 2 | Unsupported adjective; F-1-4 |
| format text | Extract JSON data flags as text. | 6 | Claim; F-1-4 |
| format text | Never invoke a shell, source an environment, or follow a redirect. | 11 | Claim; F-1-4 |
| format h3 | OpenAPI examples | 2 | — |
| format text | Check media examples already inside 3.0 and 3.1 documents against their own operation schema. | 14 | Claim; F-1-4 |
| install eyebrow | Five-minute setup | 2 | Untested quantitative claim; F-1-8 |
| install h2 | Put copied examples on the build path. | 7 | Jargon; F-1-10 |
| install text | Add one config file and one CI command. | 8 | Claim; F-1-4 |
| install text | When a field drifts, the pull request points to the example that needs repair. | 14 | Claim; F-1-4 |
| config button | Copy | 1 | Result is implied by adjacent config; acceptable |
| footer slogan | Examples should compile, too. | 4 | Slogan; F-1-11 |
| footer | Source | 1 | — |
| footer | Privacy | 1 | — |
| footer | Terms | 1 | — |
| footer | MIT licensed · No telemetry | 4 | Claim; F-1-4 |

## Copy audit — README

| Line | Sentence or label | Words | Flag |
| ---: | --- | ---: | --- |
| 3 | `api-example-linter` keeps the request and response examples people copy from API documentation aligned with the OpenAPI contract. | 17 | Claim; F-1-4 |
| 3 | It extracts fenced JSON and curl bodies from Markdown, reads examples embedded in OpenAPI 3.x documents, validates them against a selected operation or named schema, and emits concise local diagnostics or line-level GitHub Actions annotations. | 35 | >22 words and claim; F-1-12, F-1-4 |
| 5 | It is built for API maintainers who want one small, deterministic CI gate—not a documentation host, API fuzzer, or shell runner. | 21 | Claim; F-1-4 |
| 5 | Curl blocks are parsed as text and are never executed. | 10 | Claim; F-1-4 |
| 9 | Download a release binary, or install from source with Rust 1.85+. | 11 | — |
| 17 | Validate examples from Markdown against a named component schema. | 9 | — |
| 25 | Validate request examples against an operation. | 6 | — |
| 34 | Validate examples already embedded in an OpenAPI file. | 8 | — |
| 40 | Machine-readable and GitHub Actions output: | 5 | — |
| 47 | In `auto` format, the CLI emits GitHub workflow commands when `GITHUB_ACTIONS=true`; otherwise it uses readable terminal output. | 17 | Claim; F-1-4 |
| 47 | Exit code `0` means every discovered example passed, `1` means validation findings were found, and `2` means configuration or input failed. | 19 | Claim; F-1-4 |
| 51 | JSON fences are validated directly. | 5 | Claim; F-1-4 |
| 51 | Curl fences are parsed safely; JSON bodies following `--data`, `--data-raw`, `--data-binary`, or `-d` are extracted in their normal separated, `--flag=VALUE`, and compact `-dVALUE` forms. | 24 | >22 words, “safely” adjective, and claim; F-1-12, F-1-4 |
| 51 | No command is run. | 4 | Claim; F-1-4 |
| 53 | To map a block to an operation without global flags, add metadata to the fence: | 15 | — |
| 61 | Supported metadata keys are `operation`, `schema`, and `direction` (`request` or `response`). | 11 | Claim; F-1-4 |
| 61 | A global `--operation`, `--schema`, or `--direction` supplies defaults. | 8 | Claim; F-1-4 |
| 65 | `--mock-base-url` sends validated request examples to a mock server using the selected operation method and path. | 15 | Claim; F-1-4 |
| 65 | It accepts `http://localhost`, `http://127.0.0.1`, and `http://[::1]` by default. | 5 | Claim; F-1-4 |
| 65 | Add `--allow-host example.internal` explicitly for another hostname. | 6 | — |
| 65 | Redirects are disabled, private credentials are never inferred, and curl text is still never executed. | 15 | Claim; F-1-4 |
| 75 | Put `.api-example-linter.json` at the repository root to keep CI under five minutes: | 11 | Untested quantitative claim; F-1-8 |
| 87 | CLI flags override configuration. | 4 | Claim; F-1-4 |
| 87 | Run `api-example-linter init` to write a documented starter file without overwriting an existing one. | 13 | Claim; F-1-4 |
| 108 | The static documentation site is deployed from `dist/site`. | 8 | — |
| 108 | It has no analytics, external scripts, hosted fonts, cookies, or data collection; the live demo runs entirely in the browser. | 19 | Privacy/demo claim; F-1-4 |
| 112 | Version 0.1 validates the practical JSON Schema subset commonly used by OpenAPI examples: types, required properties, properties, additional properties, arrays/items, enums, const, nullable, string patterns/lengths, numeric bounds, composition (`allOf`, `anyOf`, `oneOf`, `not`), and local `$ref` values. | 37 | >22 words and claim; F-1-12, F-1-4 |
| 112 | OpenAPI 3.0 `nullable` and 3.1 JSON Schema are accepted. | 8 | Claim; F-1-4 |
| 112 | Remote `$ref` fetching is intentionally disabled for deterministic, offline-safe CI. | 10 | Claim; F-1-4 |
| 116 | MIT. See [LICENSE](LICENSE). | 3 | — |

## Verification run

From a fresh local clone in `/tmp/api-linter-review-ro6KSD`:

```sh
npm ci
npm test
npm run build
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```

All commands passed. `npm test` reported 9 Rust unit tests, 6 CLI integration tests, 1 doctest, and 7 site tests. This proves the current internal suite is green; it does not close F-1-2 through F-1-5.

## What would make this perfect

Ship a real, temporary-directory CLI demo with realistic bundled examples and a matching `/demo` page, then make that sample action the first thing a visitor can do. Replace the metaphor-led hero and section copy with the plain job/audience/result language above. Add claim inventory/tests for every retained promise, a true 404, complete social/canonical metadata, and one shared header/footer. Re-run this entire first-read review with a fresh browser context and zero findings.
