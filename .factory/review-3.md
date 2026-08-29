# Adversarial first-read review 3 — FAIL

**Reviewed:** 2026-08-29 UTC  
**Live URL:** <https://api-example-linter.sociobot.in/>  
**Commit reviewed:** `32f161659ea2eae2a1ab659d23f39001ad6846c5`

## Verdict

**FAIL.** The product is clear, tryable, and its registered claims passed. One minor finding remains: the screen-reader route announcement adds a second full stop to headings that already end in one. The acceptance rule requires zero findings for PASS.

## Cold first read

Fresh Chromium contexts opened the live home page at 390×844 and 1440×1000 before any scrolling. At 390px the document width was exactly 390px, and the job, audience, action, and action result were all visible.

- **What it does:** It lints JSON and curl API documentation examples against OpenAPI.
- **For whom:** API maintainers whose copied examples drift from their OpenAPI contract.
- **What to click first:** **Try it with sample data**. It opens a recording of the bundled CLI sample.

The exact first-screen copy is “Lint API examples against OpenAPI.”, “For API maintainers whose copied JSON or curl examples drift from their OpenAPI contract.”, and “Try it with sample data”. This passes the cold-read test on mobile and desktop.

## Findings

### F-3-1 — MINOR — route announcements contain doubled punctuation

**Location / quote:** Live navigation from `/` to `/demo/` announces `Navigated to Run the bundled linter sample..`; Back announces `Navigated to Lint API examples against OpenAPI..`. The extra stop is created at `site/src/main.ts:121` by appending `.` to heading text that already ends with `.`.

**Why this fails:** The heading is correctly focused and announced, but the punctuation typo is read awkwardly by assistive technology and is avoidable copy noise in a required route-change mechanism.

**Concrete fix:** Normalize trailing punctuation before formatting the announcement, for example use the heading text directly after `Navigated to `, or remove a final period before appending one. Extend `route navigation and Back focus and announce the destination heading` to assert the exact announcement has one terminal full stop.

## Demo and sandbox verification

| Check | Result | Evidence |
| --- | --- | --- |
| One-click entry | Pass | The first-screen action opened `/demo/?demo=1`. |
| First demo screen | Pass | The route immediately showed the completed sample transcript: two examples checked, one passed, one failed. |
| Banner and controls | Pass | The persistent banner read “Demo — sample data, nothing is saved” and exposed Reset demo and Start for real. |
| Transcript honesty | Pass | Normalized live transcript exactly matched `target/debug/api-example-linter demo` from the fresh clone. |
| Reset isolation | Pass | Reset removed `demo:api-example-linter:sentinel`, restored `demo:api-example-linter:frame`, and retained `real:sentinel`. |
| Leaving demo | Pass | Start for real returned home, removed demo keys, and retained `real:sentinel`. |
| Browser privacy | Pass | The complete demo flow made only same-origin requests, created no cookies, caused no console/page errors, and left localStorage empty. |
| Offline behavior | Pass | After the first visit and service-worker control, the live `/demo/` reloaded offline with the demo h1 and offline notice. |
| CLI sandbox | Pass | `api-example-linter demo` exited 0, reports `2 example(s) checked · 1 passed · 1 failed`, and its temporary workspace cleanup is covered by the isolated claim test. |

## Claims verification

I cloned the reviewed repository to `/tmp/api-example-linter-review3.6poxGQ`, ran `npm ci`, and executed every exact `test` command listed in `.factory/claims.json` individually. All 16 passed.

| Claim id | Result |
| --- | --- |
| `demo-temp-isolation` | Pass |
| `shell-non-execution` | Pass |
| `supported-inputs` | Pass |
| `schema-mapping` | Pass |
| `diagnostic-output` | Pass |
| `mapping-metadata` | Pass |
| `config-precedence` | Pass |
| `config-init` | Pass |
| `core-schema-checks` | Pass |
| `local-by-default` | Pass |
| `mock-host-gating` | Pass |
| `demo-transcript-parity` | Pass |
| `browser-privacy` | Pass |
| `demo-web-isolation` | Pass |
| `offline-site` | Pass |
| `mit-license` | Pass |

The registry covers the visitor-facing behavioral statements on the landing page and README: local/offline checks, curl non-execution, input formats, mappings, diagnostics, configuration, mock-host gates, browser privacy, demo isolation/parity, offline reload, and MIT licensing. No additional behavioral copy was found without a corresponding registry entry.

## Copy audit

Counts treat hyphenated and code terms as one word. Code blocks are commands or examples and are not prose sentences. The landing-page table also includes headings and controls because their clarity and result-naming were checked. No item exceeds 22 words, no banned marketing adjective appears, terminology is consistent, headings name their sections, and controls name their results.

### Landing page

| Location | Copy | Words | Result |
| --- | --- | ---: | --- |
| Skip link | Skip to content | 3 | Pass |
| Offline state | You are offline. | 3 | Pass |
| Offline state | The guide and sample remain available. | 6 | Pass |
| Header | API Example Linter | 3 | Pass |
| Header | How it works | 3 | Pass |
| Header | Demo | 1 | Pass |
| Header | Privacy | 1 | Pass |
| Header link | GitHub | 1 | Pass |
| Demo banner | Demo — sample data, nothing is saved | 6 | Pass |
| Demo banner | Recording of the bundled CLI sample. | 6 | Pass |
| Demo banner | Nothing is saved. | 3 | Pass |
| Demo control | Reset demo | 2 | Pass |
| Demo action | Start for real | 3 | Pass |
| Hero label | OpenAPI example checks | 3 | Pass |
| H1 | Lint API examples against OpenAPI. | 5 | Pass |
| Hero sentence | For API maintainers whose copied JSON or curl examples drift from their OpenAPI contract. | 14 | Pass |
| Primary action | Try it with sample data | 5 | Pass |
| Action result | Opens a recording of the bundled CLI sample. | 8 | Pass |
| Secondary action | Install the CLI | 3 | Pass |
| Fact | Default checks make no network requests. | 6 | Pass |
| Fact | The CLI works without a network connection. | 8 | Pass |
| Fact | Free under the MIT License. | 5 | Pass |
| Art labels | Input / docs; Gate / OpenAPI; Output / checked | 2; 2; 2 | Pass |
| Proof | Curl is text and is never executed | 7 | Pass |
| Proof | Local references resolve without remote fetching | 6 | Pass |
| Proof | GitHub output points to the failing line | 7 | Pass |
| Section label | How it works | 3 | Pass |
| H2 | How API example checks work | 5 | Pass |
| Sentence | Point the linter at existing documentation. | 7 | Pass |
| Sentence | Then choose the operation or schema for each example. | 9 | Pass |
| H3 | Extract examples | 2 | Pass |
| Sentence | Find fenced JSON and curl bodies in Markdown and examples inside OpenAPI 3.x files. | 14 | Pass |
| H3 | Choose a schema or operation | 5 | Pass |
| Sentence | Use a named schema or an operation request or response. | 10 | Pass |
| H3 | Report each mismatch | 3 | Pass |
| Sentence | Return the file, line, JSON pointer, and mismatch in text, JSON, or GitHub format. | 14 | Pass |
| Section label | Bundled CLI demo | 3 | Pass |
| H2 | Sample lint result | 3 | Pass |
| Sentence | Run `api-example-linter demo`. | 2 | Pass |
| Sentence | It creates, checks, and removes a temporary sample folder. | 9 | Pass |
| Link | Open the focused demo page | 5 | Pass |
| Terminal action | Play recording | 2 | Pass |
| Terminal action | Restart recording | 2 | Pass |
| Empty state | Ready to check two bundled examples. | 6 | Pass |
| Status | Recording stopped before the command runs. | 7 | Pass |
| Section label / H2 | Supported input formats | 3 | Pass |
| H3 | Fenced JSON | 2 | Pass |
| Sentence | Validate request and response payloads with optional metadata on the fence. | 11 | Pass |
| H3 | Curl request bodies | 3 | Pass |
| Sentence | Parse JSON data flags as text. | 6 | Pass |
| Sentence | Never invoke a shell or make a network request. | 9 | Pass |
| H3 | OpenAPI examples | 2 | Pass |
| Sentence | Check examples inside OpenAPI 3.0 and 3.1 documents against their operation schema. | 12 | Pass |
| Section label | Scope and privacy | 3 | Pass |
| H2 | What the linter does not do | 6 | Pass |
| Sentence | It does not run curl commands or fetch remote references. | 10 | Pass |
| Sentence | Mock requests happen only when you opt in and use a permitted host. | 12 | Pass |
| Section label | CI setup | 2 | Pass |
| H2 | Add the linter to CI | 5 | Pass |
| Sentence | Add one configuration file and one CI command. | 8 | Pass |
| Sentence | A failed check names the example that needs repair. | 9 | Pass |
| Button | Copy command | 2 | Pass |
| Button | Copy config | 2 | Pass |
| Footer | API Example Linter | 3 | Pass |
| Footer sentence | Checks documentation examples against OpenAPI. | 5 | Pass |
| Footer links | Privacy; Terms; Source | 1; 1; 1 | Pass |
| Footer | Built by Param Factory · v0.1.0 | 6 | Pass |

### README

| Line | Copy | Words | Result |
| ---: | --- | ---: | --- |
| 1 | API Example Linter | 3 | Pass |
| 3 | `api-example-linter` checks copied API examples against an OpenAPI contract. | 9 | Pass |
| 4 | It reads fenced JSON, curl request bodies, and examples inside OpenAPI 3.x files. | 13 | Pass |
| 5 | Failed checks name the file, line, JSON pointer, and mismatch. | 10 | Pass |
| 7 | It is for API maintainers who want documentation examples checked in CI. | 12 | Pass |
| 8 | Curl blocks are parsed as text and never executed. | 9 | Pass |
| 10 | Try the bundled sample | 4 | Pass |
| 12 | Open the isolated web demo, or run: | 7 | Pass |
| 18 | The command copies examples into a fresh temporary folder. | 9 | Pass |
| 19 | It runs the real validator against one current example and one stale example. | 13 | Pass |
| 20 | It then removes the folder without reading project configuration. | 9 | Pass |
| 22 | Install | 1 | Pass |
| 24 | Install from source: | 3 | Pass |
| 30 | Usage | 1 | Pass |
| 32 | Check Markdown against a named component schema: | 7 | Pass |
| 40 | Check request examples against an operation: | 6 | Pass |
| 49 | Check examples embedded in an OpenAPI file: | 7 | Pass |
| 55 | Use JSON or GitHub Actions output: | 6 | Pass |
| 62 | Text, JSON, and GitHub output identify each failed example. | 9 | Pass |
| 63 | Exit code `0` means all examples passed. | 7 | Pass |
| 64 | Exit code `1` means validation findings exist. | 7 | Pass |
| 65 | Exit code `2` means the input or configuration is invalid. | 10 | Pass |
| 67 | Markdown conventions | 2 | Pass |
| 69 | JSON fences are validated directly. | 5 | Pass |
| 70 | Curl fences support `--data`, `--data-raw`, `--data-binary`, and `-d`. | 8 | Pass |
| 71 | Separated, equals, and compact flag forms are accepted. | 8 | Pass |
| 72 | No shell command is run. | 5 | Pass |
| 74 | Add mapping metadata to a fence when global flags are not suitable: | 12 | Pass |
| 82 | Metadata keys are `operation`, `schema`, and `direction`. | 7 | Pass |
| 83 | Direction can be `request` or `response`. | 6 | Pass |
| 84 | Global flags provide defaults for those values. | 7 | Pass |
| 86 | Optional mock request | 3 | Pass |
| 88 | `--mock-base-url` sends validated requests to a mock server only after you opt in. | 13 | Pass |
| 89 | Loopback HTTP hosts are accepted by default. | 7 | Pass |
| 90 | Use `--allow-host` to permit another hostname. | 6 | Pass |
| 98 | Configuration | 1 | Pass |
| 100 | Put `.api-example-linter.json` at the repository root: | 6 | Pass |
| 112 | CLI flags override configuration. | 4 | Pass |
| 113 | `api-example-linter init` writes a starter file and refuses to overwrite an existing file. | 13 | Pass |
| 115 | CI | 1 | Pass |
| 122 | Privacy and offline behavior | 4 | Pass |
| 124 | Default checks use local files and make no network requests. | 10 | Pass |
| 125 | Local references resolve without remote fetching. | 6 | Pass |
| 126 | The optional mock request is the only CLI network path. | 10 | Pass |
| 128 | The website makes only same-origin requests and sets no cookies. | 10 | Pass |
| 129 | Its service worker keeps the guide and bundled demo available after the first visit. | 14 | Pass |
| 130 | The demo uses a separate `demo:` session-storage namespace. | 8 | Pass |
| 132 | Develop and verify | 3 | Pass |
| 143 | The demo contract documents isolation and reset behavior. | 8 | Pass |
| 145 | Scope | 1 | Pass |
| 147 | Version 0.1 checks JSON values against the selected OpenAPI operation or named schema. | 13 | Pass |
| 148 | It checks required fields, unknown fields, scalar types, and local references. | 11 | Pass |
| 149 | Remote references are reported instead of fetched. | 7 | Pass |
| 151 | License | 1 | Pass |
| 153 | MIT. | 1 | Pass |
| 153 | See LICENSE. | 2 | Pass |

Terminology remains consistent: **CLI**, **OpenAPI contract**, **example**, **schema/operation**, **mismatch**, **JSON pointer**, and **demo**. No heading is a metaphor or mood slogan. All buttons are result-naming verbs.

## Earlier findings and regression check

I read every earlier `.factory/review-*.md`, `.factory/polish-*.md`, `verification*.md`, the prior handoff, and the demo contract. I verified the reported behavior against the live site and current code rather than relying on the closure labels.

| Earlier finding | Current verification |
| --- | --- |
| F-1-1 first-screen comprehension | Fixed: the cold-read evidence above confirms job, audience, action, and result at 390px and desktop. |
| F-1-2 demo route and real result | Fixed: `/demo/?demo=1` has a visible seeded result, sandbox banner, reset/start controls, and transcript parity with the real CLI. |
| F-1-3 CLI demo command | Fixed: bundled `examples/` and `api-example-linter demo` exist; the isolation claim passes. |
| F-1-4 claims registry | Fixed: registry exists; all 16 listed commands pass independently; the current copy-to-claim audit has no gap. |
| F-1-5 designed 404 | Fixed: live `/no-such-page` returned HTTP 404 and the styled page’s h1 was “Page not found”. |
| F-1-6 metadata | Fixed: home plus Demo, Privacy, Terms, and 404 have route titles, descriptions, canonical URLs, favicon/touch icon, and full OG/Twitter image metadata. |
| F-1-7 shared header/footer | Fixed: all five reviewed routes contain the same header treatment and both legal links in the footer. |
| F-1-8 five-minute promise | Fixed: absent from landing and README. |
| F-1-9 metaphor hero | Fixed: the plain job headline is present. |
| F-1-10 vague headings/control | Fixed: section headings are informative and the control says “Restart recording”. |
| F-1-11 footer slogan | Fixed: footer states what the product checks. |
| F-1-12 overlong README prose | Fixed: the complete audit above contains no sentence above 22 words. |
| F-2-1 nonexistent release option | Fixed: README offers only the working source installation command. |
| F-2-2 heading focus after navigation | Fixed: live forward and Back navigation moved focus to each destination h1. F-3-1 is a new announcement-punctuation defect, not a focus regression. |
| F-2-3 mobile targets | Fixed: the clean browser test passed for every visible link/button across all routes at 390px. |
| F-2-4 route social metadata | Fixed: every route declares image URL, 1200×630 dimensions, alt text, Twitter image, and canonical URL. |
| Verification P1 curl equals/compact flags | Fixed: `supported-inputs` passed all documented flags/forms. |
| Verification P1 live headers/cache | Fixed in current source and live checks: same-origin CSP, `nosniff`, no-referrer policy, and 404 override are present; live hashed assets were served normally. |
| Verification P2 redundant malformed-input output | Fixed: Rust regression suite passed. |
| Verification P2 ellipsized install command | Fixed: the visible and copied command are identical and executable. |

## Structure, accessibility, links, and identity

| Check | Result |
| --- | --- |
| Titles, descriptions, canonical/OG/Twitter, favicon, `lang`, one h1, main | Pass on `/`, `/demo/`, `/privacy/`, `/terms/`, and 404. |
| Routing, deep links, Back, focus | Pass for route content and h1 focus; announcement punctuation is F-3-1. |
| 404 | Pass: unknown live route is HTTP 404 with a recovery action. |
| Header/footer and legal links | Pass. |
| Link crawl | Pass: every discovered internal and GitHub link returned HTTP 200. |
| Console/errors, requests, cookies | Pass: no console/page errors; requests remained same-origin; no cookies. |
| Keyboard, touch targets, reduced motion, axe | Pass in the clean browser suite: zero serious/critical WCAG A/AA violations and 44px target checks pass. |
| Offline | Pass after first visit on the live demo. |
| Visual identity | Pass: original Contract Loom artwork, paper-grid field, indigo/coral/mint contract-gate language, clipped controls, and drafted terminal form are distinct from a generic SaaS template. |
| Missed leverage / AI | No finding. The brief is deterministic local validation; an AI step would not be an expected or useful core action. Markdown/OpenAPI path input and GitHub annotations already provide the implied import and CI workflow. |

## Quality-gate evidence

From the fresh clone, `npm test` passed: 9 Rust unit tests, 7 CLI integration tests, 1 doctest, 17 static-site tests, 16 claim tests, and 6 browser tests. `npm run build`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo package --locked` were also started from the same clean checkout; the full test and individual-claim runs above completed successfully.

## What would make this perfect

Correct the single extra full stop in route announcements, add the exact-punctuation assertion, then rerun this entire review from a fresh clone and fresh mobile/desktop browser contexts. With that result, there is no other identified product, demo, copy, claims, privacy, routing, accessibility, or visual-system work left.
