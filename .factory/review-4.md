# Adversarial first-read review 4 — PASS

**Reviewed:** 2026-08-29 UTC  
**Live URL:** <https://api-example-linter.sociobot.in/>  
**Repository commit:** `8a5b50e76d759a69f17f4ebd4b92c444eb588dca`

## Verdict

**PASS.** There are zero blocking or minor findings. The product is clear on a first mobile screen, has an isolated one-click sample path, and every published behavioural claim is registered and passed from a clean clone.

## Cold first read

I opened the live home page in separate fresh Chromium contexts at 390×844 and 1440×1000, before scrolling. At 390px the document width was exactly 390px. The answer to all three first-read questions was visible:

- **What it does:** It lints JSON and curl API documentation examples against OpenAPI.
- **For whom:** API maintainers whose copied examples drift from their OpenAPI contract.
- **What to click first:** **Try it with sample data**; it opens a recording of the bundled CLI sample.

The exact useful text is “Lint API examples against OpenAPI.”, “For API maintainers whose copied JSON or curl examples drift from their OpenAPI contract.”, and “Try it with sample data”. This passes on mobile and desktop. There was no horizontal overflow, console error, or page error.

## Demo and sandbox verification

| Check | Result | Evidence |
| --- | --- | --- |
| One-click entry | Pass | The hero action opens `/demo/?demo=1`. |
| Immediate real use | Pass | The first demo screen already contains the completed build-generated CLI transcript: two examples checked, one passed, one failed on `retired_field`. |
| Banner and controls | Pass | “Demo — sample data, nothing is saved”, Reset demo, and Start for real remain visible. |
| Transcript honesty | Pass | The normalized web transcript exactly matches `api-example-linter demo`. |
| Web isolation | Pass | Reset removed `demo:api-example-linter:dirty`, restored the demo frame, retained `real:sentinel`, and left localStorage empty. |
| Browser privacy | Pass | The whole demo flow requested only same-origin resources, set no cookies, and exposed no file/form control. |
| CLI sandbox | Pass | From a fresh temporary caller directory, `api-example-linter demo` created `/tmp/api-example-linter-demo-eQdnlH`, checked its bundled samples, removed the folder, and left the caller empty. |
| Offline | Pass | The registered offline test reloads a visited demo with service-worker control and no network. |

`.factory/demo.md` documents both entry points, the bundled inputs, reset behaviour, and the `demo:api-example-linter:` session-storage namespace.

## Claims verification

I made a fresh depth-one clone of `origin/main` in `/tmp/api-example-linter-review4-clean.6XLYHo`, ran `npm ci`, and ran every exact command in `.factory/claims.json` independently. All 16 passed.

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

The registry covers local/offline checking, curl parsing without execution, inputs, mappings, diagnostics, configuration, mock-host restrictions, transcript parity, browser privacy, demo storage, offline reload, and MIT licensing. No unlisted live or README behavioural claim was found.

## Copy audit

Counts treat hyphenated and code terms as one word. Code blocks are commands or data, not prose. The landing audit includes labels, headings, and controls so no visible copy escapes review. Every entry is ≤22 words, has no banned marketing adjective or mood heading, and uses consistent terms. All results are **Pass**; `location — words — copy` follows.

### Landing page

| Location | Words | Copy |
| --- | ---: | --- |
| Skip link | 3 | Skip to content |
| Offline | 3 | You are offline. |
| Offline | 6 | The guide and sample remain available. |
| Header | 3 | API Example Linter |
| Header | 3 | How it works |
| Header | 1 | Demo |
| Header | 1 | Privacy |
| Header | 1 | GitHub |
| Demo banner | 6 | Demo — sample data, nothing is saved |
| Demo banner | 6 | Recording of the bundled CLI sample. |
| Demo banner | 3 | Nothing is saved. |
| Demo control | 2 | Reset demo |
| Demo action | 3 | Start for real |
| Hero label | 3 | OpenAPI example checks |
| H1 | 5 | Lint API examples against OpenAPI. |
| Hero sentence | 14 | For API maintainers whose copied JSON or curl examples drift from their OpenAPI contract. |
| Primary action | 5 | Try it with sample data |
| Action result | 8 | Opens a recording of the bundled CLI sample. |
| Secondary action | 3 | Install the CLI |
| Fact | 6 | Default checks make no network requests. |
| Fact | 8 | The CLI works without a network connection. |
| Fact | 5 | Free under the MIT License. |
| Art label | 2 | Input / docs |
| Art label | 2 | Gate / OpenAPI |
| Art label | 2 | Output / checked |
| Proof | 7 | Curl is text and is never executed |
| Proof | 6 | Local references resolve without remote fetching |
| Proof | 7 | GitHub output points to the failing line |
| Section label | 3 | How it works |
| H2 | 5 | How API example checks work |
| Sentence | 7 | Point the linter at existing documentation. |
| Sentence | 9 | Then choose the operation or schema for each example. |
| H3 | 2 | Extract examples |
| Sentence | 14 | Find fenced JSON and curl bodies in Markdown and examples inside OpenAPI 3.x files. |
| H3 | 5 | Choose a schema or operation |
| Sentence | 10 | Use a named schema or an operation request or response. |
| H3 | 3 | Report each mismatch |
| Sentence | 14 | Return the file, line, JSON pointer, and mismatch in text, JSON, or GitHub format. |
| Section label | 3 | Bundled CLI demo |
| H2 | 3 | Sample lint result |
| Sentence | 2 | Run `api-example-linter demo`. |
| Sentence | 9 | It creates, checks, and removes a temporary sample folder. |
| Link | 5 | Open the focused demo page |
| Terminal action | 2 | Play recording |
| Terminal action | 2 | Restart recording |
| Empty state | 6 | Ready to check two bundled examples. |
| Status | 7 | Recording stopped before the command runs. |
| Section label / H2 | 3 | Supported input formats |
| H3 | 2 | Fenced JSON |
| Sentence | 11 | Validate request and response payloads with optional metadata on the fence. |
| H3 | 3 | Curl request bodies |
| Sentence | 6 | Parse JSON data flags as text. |
| Sentence | 9 | Never invoke a shell or make a network request. |
| H3 | 2 | OpenAPI examples |
| Sentence | 12 | Check examples inside OpenAPI 3.0 and 3.1 documents against their operation schema. |
| Section label | 3 | Scope and privacy |
| H2 | 6 | What the linter does not do |
| Sentence | 10 | It does not run curl commands or fetch remote references. |
| Sentence | 12 | Mock requests happen only when you opt in and use a permitted host. |
| Section label | 2 | CI setup |
| H2 | 5 | Add the linter to CI |
| Sentence | 8 | Add one configuration file and one CI command. |
| Sentence | 9 | A failed check names the example that needs repair. |
| Button | 2 | Copy command |
| Button | 2 | Copy config |
| Footer | 3 | API Example Linter |
| Footer sentence | 5 | Checks documentation examples against OpenAPI. |
| Footer links | 1; 1; 1 | Privacy; Terms; Source |
| Footer | 6 | Built by Param Factory · v0.1.0 |

### README

| Line | Words | Copy |
| ---: | ---: | --- |
| 1 | 3 | API Example Linter |
| 3 | 9 | `api-example-linter` checks copied API examples against an OpenAPI contract. |
| 4 | 13 | It reads fenced JSON, curl request bodies, and examples inside OpenAPI 3.x files. |
| 5 | 10 | Failed checks name the file, line, JSON pointer, and mismatch. |
| 7 | 12 | It is for API maintainers who want documentation examples checked in CI. |
| 8 | 9 | Curl blocks are parsed as text and never executed. |
| 10 | 4 | Try the bundled sample |
| 12 | 7 | Open the isolated web demo, or run: |
| 18 | 9 | The command copies examples into a fresh temporary folder. |
| 19 | 13 | It runs the real validator against one current example and one stale example. |
| 20 | 9 | It then removes the folder without reading project configuration. |
| 22 | 1 | Install |
| 24 | 3 | Install from source: |
| 30 | 1 | Usage |
| 32 | 7 | Check Markdown against a named component schema: |
| 40 | 6 | Check request examples against an operation: |
| 49 | 7 | Check examples embedded in an OpenAPI file: |
| 55 | 6 | Use JSON or GitHub Actions output: |
| 62 | 9 | Text, JSON, and GitHub output identify each failed example. |
| 63 | 7 | Exit code `0` means all examples passed. |
| 64 | 7 | Exit code `1` means validation findings exist. |
| 65 | 10 | Exit code `2` means the input or configuration is invalid. |
| 67 | 2 | Markdown conventions |
| 69 | 5 | JSON fences are validated directly. |
| 70 | 8 | Curl fences support `--data`, `--data-raw`, `--data-binary`, and `-d`. |
| 71 | 8 | Separated, equals, and compact flag forms are accepted. |
| 72 | 5 | No shell command is run. |
| 74 | 12 | Add mapping metadata to a fence when global flags are not suitable: |
| 82 | 7 | Metadata keys are `operation`, `schema`, and `direction`. |
| 83 | 6 | Direction can be `request` or `response`. |
| 84 | 7 | Global flags provide defaults for those values. |
| 86 | 3 | Optional mock request |
| 88 | 13 | `--mock-base-url` sends validated requests to a mock server only after you opt in. |
| 89 | 7 | Loopback HTTP hosts are accepted by default. |
| 90 | 6 | Use `--allow-host` to permit another hostname. |
| 98 | 1 | Configuration |
| 100 | 6 | Put `.api-example-linter.json` at the repository root: |
| 112 | 4 | CLI flags override configuration. |
| 113 | 13 | `api-example-linter init` writes a starter file and refuses to overwrite an existing file. |
| 115 | 1 | CI |
| 122 | 4 | Privacy and offline behavior |
| 124 | 10 | Default checks use local files and make no network requests. |
| 125 | 6 | Local references resolve without remote fetching. |
| 126 | 10 | The optional mock request is the only CLI network path. |
| 128 | 10 | The website makes only same-origin requests and sets no cookies. |
| 129 | 14 | Its service worker keeps the guide and bundled demo available after the first visit. |
| 130 | 8 | The demo uses a separate `demo:` session-storage namespace. |
| 132 | 3 | Develop and verify |
| 143 | 8 | The demo contract documents isolation and reset behavior. |
| 145 | 1 | Scope |
| 147 | 13 | Version 0.1 checks JSON values against the selected OpenAPI operation or named schema. |
| 148 | 11 | It checks required fields, unknown fields, scalar types, and local references. |
| 149 | 7 | Remote references are reported instead of fetched. |
| 151 | 1 | License |
| 153 | 1 | MIT. |
| 153 | 2 | See LICENSE. |

Terminology is consistent: **CLI**, **OpenAPI contract**, **example**, **schema/operation**, **mismatch**, **JSON pointer**, and **demo**. Headings name their sections and every control names its result.

## Earlier findings and regression check

I read `review-1.md`, `review-2.md`, `review-3.md`, every `polish-*.md`, both verification reports, the former handoff, and the demo contract. I confirmed each finding in production and current code rather than relying on closure labels.

| Earlier finding | Current verification |
| --- | --- |
| F-1-1 | Fixed: cold mobile/desktop check confirms job, audience, action, and result. |
| F-1-2 / F-1-3 | Fixed: one-click demo has a seeded real transcript; `api-example-linter demo` uses and removes bundled samples in a temporary directory. |
| F-1-4 | Fixed: all 16 registry commands pass independently, including parity and privacy. |
| F-1-5 | Fixed: `/no-such-page` returns an HTTP 404 styled recovery page. |
| F-1-6 / F-2-4 | Fixed: every reviewed route has route title, description, canonical, favicon/touch icon, OG/Twitter image, dimensions, and alt text. |
| F-1-7 | Fixed: the shared header/footer keeps Privacy and Terms on every route. |
| F-1-8 | Fixed: no unsupported five-minute promise remains. |
| F-1-9 / F-1-10 / F-1-11 | Fixed: hero/section/footer language is factual and the control is “Restart recording”. |
| F-1-12 | Fixed: the full audit above has no sentence above 22 words. |
| F-2-1 | Fixed: README offers only the working source install command. |
| F-2-2 / F-3-1 | Fixed: live forward and Back focus the new h1 and announce exactly one terminal full stop. |
| F-2-3 | Fixed: mobile 44px control tests pass. |
| Verification P1 | Fixed: documented curl flag forms pass; live CSP, no-referrer, nosniff, permissions policy, cache, and 404 policy are present. |
| Verification P2 | Fixed: malformed-input regression and exact visible/copied install command tests pass. |

## Structure, accessibility, links, and identity

| Check | Result |
| --- | --- |
| Titles, descriptions, canonical/OG/Twitter, favicon, `lang`, one h1, main | Pass on `/`, `/demo/`, `/privacy/`, `/terms/`, and `/404.html`. |
| Deep links, Back, focus, announcement | Pass. |
| 404 | Pass. An unknown URL returns HTTP 404 with a recovery action. The browser resource error for that intended 404 is expected. |
| Header/footer and legal links | Pass. |
| Link crawl | Pass: all internal routes, robots, sitemap, manifest, repository, and linked MIT License returned HTTP 200. |
| Keyboard and accessibility | Pass: skip link works; live axe scans found zero violations across six reviewed responses. |
| Privacy / requests | Pass: demo flow was same-origin only, with no cookies or localStorage. |
| Build/live parity | Pass: production JS SHA-256 `d8349565…682f3e` and CSS SHA-256 `621a3883…1d6e67` match the clean build. |
| Visual identity | Pass: original Contract Loom art, drafting-paper grid, indigo schema gate, coral/mint status marks, clipped controls, and terminal treatment are product-specific, not a generic SaaS template. |
| Missed leverage / AI | No finding. The brief is deterministic local validation; AI would add cost and uncertainty without improving the core job. Inputs and GitHub output cover the implied import/CI workflow. |

## Quality-gate evidence

From the clean clone, `npm test` passed 9 Rust unit tests, 7 CLI integration tests, 1 doctest, 17 static-site tests, 16 claim tests, and 6 browser tests. `npm run build` produced `dist/bin/api-example-linter` and `dist/site`. `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo package --locked` passed.

## What would make this perfect

Keep the current claim-to-test coverage and repeat this complete cold-browser and clean-clone review for future releases. No product change is identified in this round.
