# Adversarial first-read review 2 — FAIL

**Reviewed:** 2026-08-29 UTC

**Live URL:** <https://api-example-linter.sociobot.in/>

**Commit reviewed:** `bf275586e2a17a66bf4ebf968f17d16317605aab`

## Verdict

**FAIL.** Two earlier blocking findings are only half-fixed. The browser demo presents invented terminal lines as a recording of the CLI, and the claim registry still omits documented behavior. Four additional minor findings remain. All 13 registered claim commands pass; that does not cover the unlisted claims.

## Cold first read

I opened fresh Chromium contexts at 390×844 and 1440×1000 and did not scroll.

- **What it does:** It checks JSON and curl examples against OpenAPI.
- **For whom:** API maintainers whose documentation examples can drift from the contract.
- **What to click first:** “Try it with sample data”; the adjacent text says it will run the included failing example in a temporary folder.

This passes the first-screen comprehension test at both sizes. The exact useful copy is “Lint API examples against OpenAPI.”, “For API maintainers whose copied JSON or curl examples drift from their OpenAPI contract.”, and “Try it with sample data”. At 390px, all three and the stated result are visible without scrolling. There is no horizontal overflow.

## Findings

### F-1-2 — BLOCKING — reopened: the web “recording” is not the real CLI output

**Location / quote:** `/demo/?demo=1`, terminal labelled “Recording of the bundled command-line demo”; banner copy “This page uses only the bundled sample output”; home action note “Runs the included failing API example in a temporary folder.”

**Evidence:** The live terminal shows invented lines including “CHECK docs/create-pet.md:6 JSON request” and “PASS name and status match createPet”, with `$ api-example-linter demo` first. The real clean-clone command prints the temporary-folder notice first, then the command, one `SCHEMA_MISMATCH` line, and the summary. It never prints either `CHECK` line or the `PASS` line. `site/src/main.ts` hard-codes the browser frames independently of the binary.

**Why this fails:** Review 1 required a recording of that exact command. The banner, seeded result, reset, and CLI sandbox now exist, but the displayed recording is not an execution or faithful recording of the product. A first-time visitor is shown diagnostics the CLI does not produce.

**Concrete fix:** Generate the terminal transcript by running the release binary against `examples/` during the site build, or record the exact output verbatim. Add a test that compares the normalized browser transcript with `api-example-linter demo`. Change the home note to “Opens a recording of the bundled CLI sample” so it names what the click actually does.

### F-1-4 — BLOCKING — reopened: visitor-facing claims remain outside the claim registry

**Location / quotes:** landing and `README.md`:

| Unlisted or inadequately tested claim | Gap | Concrete fix |
| --- | --- | --- |
| “The output comes from the bundled failing example.” | No tagged test proves that the web transcript matches the bundled sample or binary; it currently does not. | Add a web-transcript parity claim and test after fixing F-1-2. |
| “Never invoke a shell or follow a redirect.” | `shell-non-execution` tests shell non-execution, but no registered claim or tagged assertion covers redirects. | Expand that claim and use a redirecting probe server, or remove “or follow a redirect.” |
| “Download a release binary, or install from source with Rust 1.85 or newer.” | No release/MSRV claim exists. The repository currently has no GitHub release or tag. | Remove the release clause until a linked binary exists; add an MSRV claim tested with Rust 1.85. |
| “Exit code `0` means all examples passed.” / “Exit code `1` means validation findings exist.” / “Exit code `2` means the input or configuration is invalid.” | The diagnostic test happens to assert these statuses, but the registry claim does not state the exit-code contract. | Add an `exit-codes` entry and one tagged test for all three outcomes, or expand the registered diagnostic claim. |
| “Curl fences support `--data`, `--data-raw`, `--data-binary`, and `-d`.” / “Separated, equals, and compact flag forms are accepted.” | The tagged `supported-inputs` test covers only separated `--data`; other forms live in an untagged unit test. | Add every documented form to the tagged claim test and claim text. |
| “Metadata keys are `operation`, `schema`, and `direction`.” / “Direction can be `request` or `response`.” / “Global flags provide defaults for those values.” | No registry entry states this configuration contract, and the tagged mapping test does not cover all keys, both directions, and override/default behavior. | Add a mapping-metadata claim with those cases. |
| “Use `--allow-host` to permit another hostname.” | `mock-host-gating` tests rejection and loopback, not a non-loopback hostname explicitly enabled with `--allow-host`. | Extend the tagged test to prove the allow-list path. |
| “CLI flags override configuration.” | No entry or tagged test covers precedence. | Add a configuration-precedence claim and conflicting file/flag fixture. |
| “`npm test` runs Rust, site, claim, browser, accessibility, privacy, and offline checks.” | No registry entry defines or checks this advertised verification bundle. | Remove this claim from visitor documentation or add a deterministic meta-test. |
| “`npm run build` writes the binary to `dist/bin` and the static site to `dist/site`.” / “`npm run build:site` builds only the static site.” | Build-output behavior is not registered. | Add one build-output claim or keep these instructions outside the visitor-facing README. |
| “Every visitor-facing claim is mapped to one tagged test in `.factory/claims.json`.” | This statement is false because of the rows above. | Add a registry-to-copy coverage check before restoring this sentence. |

**Why this fails:** The claims contract says every visitor-facing claim must be listed and tested. A passing test that does not state or exercise the full published promise does not close that requirement. This is the same incomplete claim-control defect as review 1.

### F-2-1 — MINOR — the install section offers a release binary that does not exist

**Location / quote:** `README.md:24`, “Download a release binary, or install from source with Rust 1.85 or newer:”

**Evidence:** The repository has no Git tags, and GitHub’s latest-release endpoint returned 404. The sentence provides no download link.

**Why this fails:** A first-time user is told to choose an unavailable installation path.

**Concrete fix:** Until a release is published, write “Install from source with Rust 1.85 or newer:” and keep the working `cargo install --git …` command. When binaries exist, link the exact release page and supported platforms.

### F-2-2 — MINOR — route changes do not move focus to the new page heading

**Location:** navigation from `/` to `/demo/` and browser Back.

**Evidence:** After both the forward navigation and Back, Playwright reported `document.activeElement` as `<body>`, not the new `<h1>` or `<main>`. `site/src/main.ts` makes `<main>` programmatically focusable only for the skip link; it does not focus or announce the destination on normal route changes.

**Why this fails:** Keyboard and screen-reader users receive no programmatic indication that the page changed, contrary to the required route-change behavior.

**Concrete fix:** On each page load/navigation, focus the destination `<h1>` (or `<main>` with an announced heading) without scrolling unexpectedly. Add a forward-and-back browser test that asserts the destination heading is announced/focused.

### F-2-3 — MINOR — several mobile touch targets are below 44×44px

**Location / evidence at 390px:** the brand link is 166×32px; “Install the CLI” is 125×26px; “Open the focused demo page” is 249×21px; legal-page return links are 120×20px; footer links are 20–22px high, with “Terms” also only 43px wide.

**Why this fails:** These links miss the required 44px touch-target baseline and are easier to miss on a phone.

**Concrete fix:** Add padding or a minimum block size so every interactive target is at least 44×44px while preserving visible spacing. Add a 390px bounding-box assertion for all non-hidden links and buttons.

### F-2-4 — MINOR — non-home routes have incomplete social metadata

**Location:** `/demo/`, `/privacy/`, `/terms/`, and the designed 404.

**Evidence:** Demo, Privacy, and Terms include `twitter:card` but omit `twitter:image`, and omit `og:image:alt`, width, and height. The 404 has no Open Graph or Twitter metadata. The current metadata test only checks that the filename appears somewhere, so it does not catch the missing fields.

**Why this fails:** The required route metadata is incomplete and shared previews can lose the product image or its description.

**Concrete fix:** Add the full 1200×630 image metadata set to each public route and appropriate noindex-aware share metadata to the 404. Strengthen tests to query each exact meta name/property.

## Demo and sandbox verification

| Check | Result | Evidence |
| --- | --- | --- |
| One click from first screen | Pass | “Try it with sample data” opened `/demo/?demo=1`. |
| Product visible after click | Pass with F-1-2 caveat | At 390px the terminal begins at y=534 and already shows the command, sample path, and a passing check. The transcript is not the real CLI output. |
| Persistent demo banner | Pass | “Demo — sample data, nothing is saved”, Reset demo, and Start for real are present. |
| Reset isolation | Pass | Reset removed `demo:api-example-linter:dirty`, retained `real:user-setting`, and restored only the demo frame key. |
| Browser privacy | Pass | Fresh live contexts made same-origin requests only, set no cookies, and logged no console/page errors. |
| Offline reload | Pass | A visited live demo reloaded under `context.setOffline(true)` with its h1 and sample intact. |
| CLI sandbox | Pass | Running the release binary from `/tmp/ael-review2-demo.heA9qI` created `/tmp/api-example-linter-demo-XFr5ur`, reported 2 checked / 1 passed / 1 failed, removed the temporary folder, and left the caller directory empty. |
| Real browser/CLI parity | **Fail** | F-1-2. |

## Registered claim results

I cloned commit `bf275586e2a17a66bf4ebf968f17d16317605aab` to `/tmp/ael-review2-clean.FA2f3E`, ran `npm ci`, then ran every command from `.factory/claims.json` separately.

| Claim id | Result |
| --- | --- |
| `demo-temp-isolation` | Pass |
| `shell-non-execution` | Pass |
| `supported-inputs` | Pass |
| `schema-mapping` | Pass |
| `diagnostic-output` | Pass |
| `config-init` | Pass |
| `core-schema-checks` | Pass |
| `local-by-default` | Pass |
| `mock-host-gating` | Pass |
| `browser-privacy` | Pass |
| `demo-web-isolation` | Pass |
| `offline-site` | Pass |
| `mit-license` | Pass |

Result: **13/13 registered commands passed.** F-1-4 remains because the registry is not complete and several tagged tests are narrower than their published copy.

## Copy audit — landing page

Counts treat hyphenated terms and code-like tokens as one word. Code samples are not sentences. No landing sentence exceeds 22 words and no banned marketing adjective appears.

| Location | Copy | Words | Result |
| --- | --- | ---: | --- |
| skip link | Skip to content | 3 | Pass |
| offline state | You are offline. | 3 | Pass |
| offline state | The guide and sample remain available. | 6 | Pass |
| wordmark | API Example Linter | 3 | Pass |
| nav | How it works | 3 | Pass |
| nav | Demo | 1 | Pass |
| nav | Privacy | 1 | Pass |
| nav | GitHub | 1 | Pass |
| demo banner | Demo — sample data, nothing is saved | 6 | Pass |
| demo banner | The output comes from the bundled failing example. | 8 | F-1-2, F-1-4 |
| demo control | Reset demo | 2 | Pass |
| demo action | Start for real | 3 | Pass |
| hero label | OpenAPI example checks | 3 | Pass |
| h1 | Lint API examples against OpenAPI. | 5 | Pass |
| hero sentence | For API maintainers whose copied JSON or curl examples drift from their OpenAPI contract. | 14 | Pass |
| primary action | Try it with sample data | 5 | Pass |
| action note | Runs the included failing API example in a temporary folder. | 10 | F-1-2 |
| secondary action | Install the CLI | 3 | Pass |
| fact | Default checks make no network requests. | 6 | Pass |
| fact | The CLI works without a network connection. | 8 | Pass |
| fact | Free under the MIT License. | 5 | Pass |
| art label | Input / docs | 2 | Pass |
| art label | Gate / OpenAPI | 2 | Pass |
| art label | Output / checked | 2 | Pass |
| proof | Curl is text and is never executed | 7 | Pass |
| proof | Local references resolve without remote fetching | 6 | Pass |
| proof | GitHub output points to the failing line | 7 | Pass |
| section label | How it works | 3 | Pass |
| h2 | How API example checks work | 5 | Pass |
| sentence | Point the linter at existing documentation. | 7 | Pass |
| sentence | Then choose the operation or schema for each example. | 9 | Pass |
| h3 | Extract examples | 2 | Pass |
| sentence | Find fenced JSON and curl bodies in Markdown and examples inside OpenAPI 3.x files. | 14 | Pass |
| h3 | Choose a schema or operation | 5 | Pass |
| sentence | Use a named schema or an operation request or response. | 10 | Pass |
| h3 | Report each mismatch | 3 | Pass |
| sentence | Return the file, line, JSON pointer, and mismatch in text, JSON, or GitHub format. | 14 | Pass |
| section label | Bundled CLI demo | 3 | Pass |
| h2 | Sample lint result | 3 | Pass |
| sentence | Run `api-example-linter demo`. | 2 | Pass |
| sentence | It creates, checks, and removes a temporary sample folder. | 9 | Pass |
| link | Open the focused demo page | 5 | Pass |
| control | Play recording | 2 | Pass |
| control | Restart recording | 2 | Pass |
| empty state | Ready to check two bundled examples. | 6 | Pass |
| status | Recording stopped before the command runs. | 7 | Pass |
| section label / h2 | Supported input formats | 3 | Pass |
| h3 | Fenced JSON | 2 | Pass |
| sentence | Validate request and response payloads with optional metadata on the fence. | 11 | Pass |
| h3 | Curl request bodies | 3 | Pass |
| sentence | Parse JSON data flags as text. | 6 | Pass |
| sentence | Never invoke a shell or follow a redirect. | 8 | F-1-4 |
| h3 | OpenAPI examples | 2 | Pass |
| sentence | Check examples inside OpenAPI 3.0 and 3.1 documents against their operation schema. | 12 | Pass |
| section label | Scope and privacy | 3 | Pass |
| h2 | What the linter does not do | 6 | Pass |
| sentence | It does not run curl commands or fetch remote references. | 10 | Pass |
| sentence | Mock requests happen only when you provide a permitted local host. | 11 | Pass |
| section label | CI setup | 2 | Pass |
| h2 | Add the linter to CI | 5 | Pass |
| sentence | Add one configuration file and one CI command. | 8 | Pass |
| sentence | A failed check names the example that needs repair. | 9 | Pass |
| control | Copy command | 2 | Pass |
| control | Copy config | 2 | Pass |
| footer | API Example Linter | 3 | Pass |
| footer sentence | Checks documentation examples against OpenAPI. | 5 | Pass |
| footer links | Privacy / Terms / Source | 3 | Pass |
| footer | Built by Param Factory · v0.1.0 | 6 | Pass |

Interactive states are also short: “Copied” (1), “Copy failed” (2), “Pause recording” (2), “Resume recording” (2), “Play again” (2), “Sample result complete.” (3), “One stale field was found.” (5), “Sample result shown without animation.” (5), “Recording reset.” (2), “Press Play recording to run it.” (6), and “Demo reset to the bundled sample result.” (7). The terminal’s hard-coded output is covered by F-1-2.

## Copy audit — README

Code blocks are excluded because they are commands or sample data, not sentences. No README sentence exceeds 22 words and no banned marketing adjective appears.

| Line | Sentence, heading, or instruction | Words | Result |
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
| 24 | Download a release binary, or install from source with Rust 1.85 or newer: | 13 | F-1-4, F-2-1 |
| 30 | Usage | 1 | Pass |
| 32 | Check Markdown against a named component schema: | 7 | Pass |
| 40 | Check request examples against an operation: | 6 | Pass |
| 49 | Check examples embedded in an OpenAPI file: | 7 | Pass |
| 55 | Use JSON or GitHub Actions output: | 6 | Pass |
| 62 | Text, JSON, and GitHub output identify each failed example. | 9 | Pass |
| 63 | Exit code `0` means all examples passed. | 7 | F-1-4 |
| 64 | Exit code `1` means validation findings exist. | 7 | F-1-4 |
| 65 | Exit code `2` means the input or configuration is invalid. | 10 | F-1-4 |
| 67 | Markdown conventions | 2 | Pass |
| 69 | JSON fences are validated directly. | 5 | Pass |
| 70 | Curl fences support `--data`, `--data-raw`, `--data-binary`, and `-d`. | 8 | F-1-4 |
| 71 | Separated, equals, and compact flag forms are accepted. | 8 | F-1-4 |
| 72 | No shell command is run. | 5 | Pass |
| 74 | Add mapping metadata to a fence when global flags are not suitable: | 12 | Pass |
| 82 | Metadata keys are `operation`, `schema`, and `direction`. | 7 | F-1-4 |
| 83 | Direction can be `request` or `response`. | 6 | F-1-4 |
| 84 | Global flags provide defaults for those values. | 7 | F-1-4 |
| 86 | Optional mock request | 3 | Pass |
| 88 | `--mock-base-url` sends validated requests to a mock server only after you opt in. | 13 | Pass |
| 89 | Loopback HTTP hosts are accepted by default. | 7 | Pass |
| 90 | Use `--allow-host` to permit another hostname. | 6 | F-1-4 |
| 98 | Configuration | 1 | Pass |
| 100 | Put `.api-example-linter.json` at the repository root: | 6 | Pass |
| 112 | CLI flags override configuration. | 4 | F-1-4 |
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
| 143 | `npm test` runs Rust, site, claim, browser, accessibility, privacy, and offline checks. | 12 | F-1-4 |
| 144 | `npm run build` writes the binary to `dist/bin` and the static site to `dist/site`. | 14 | F-1-4 |
| 145 | `npm run build:site` builds only the static site. | 8 | F-1-4 |
| 147 | Every visitor-facing claim is mapped to one tagged test in `.factory/claims.json`. | 11 | F-1-4 |
| 148 | The demo contract documents isolation and reset behavior. | 8 | Pass |
| 150 | Scope | 1 | Pass |
| 152 | Version 0.1 checks JSON values against the selected OpenAPI operation or named schema. | 13 | Pass |
| 153 | It checks required fields, unknown fields, scalar types, and local references. | 11 | Pass |
| 154 | Remote references are reported instead of fetched. | 7 | Pass |
| 156 | License | 1 | Pass |
| 158 | MIT. See LICENSE. | 3 | Pass |

Terminology is otherwise consistent: **CLI** for the executable, **OpenAPI contract** for the document, **schema/operation** for the selected mapping, **example** for a payload, **mismatch** for a validation result, **JSON pointer** for its location, and **demo** for the isolated sample. Technical terms are appropriate for the stated API-maintainer audience. All buttons use result-naming verbs.

## Earlier finding verification

I reread `.factory/review-1.md`, `.factory/polish-1.md`, both verification reports, and the prior handoff. Each earlier finding was checked against both live behavior and current source.

| Earlier id | Result in round 2 |
| --- | --- |
| F-1-1 | Fixed: mobile and desktop first screens state job, audience, action, and result. |
| F-1-2 | **Reopened:** banner/reset/route/CLI sandbox exist, but the claimed real recording is hard-coded and differs from actual output. |
| F-1-3 | Fixed: `api-example-linter demo` ships with realistic examples and cleans its temporary workspace. |
| F-1-4 | **Reopened:** 13 registered tests pass, but the copy-to-registry audit above finds unlisted and under-tested claims. |
| F-1-5 | Fixed: an unknown live URL returns HTTP 404 and the designed “Page not found” page. |
| F-1-6 | Home fixed; new route-specific metadata omissions are F-2-4. |
| F-1-7 | Fixed: headers/footers are consistent and every footer links Privacy and Terms. |
| F-1-8 | Fixed: “Five-minute” and “under five minutes” are absent. |
| F-1-9 | Fixed: the h1 is “Lint API examples against OpenAPI.” |
| F-1-10 | Fixed: the reported mood/jargon headings and “Restart” label are gone. |
| F-1-11 | Fixed: the footer says “Checks documentation examples against OpenAPI.” |
| F-1-12 | Fixed: no landing or README sentence exceeds 22 words. |
| Verification P1 curl equals forms | Fixed: clean tests and direct coverage pass. |
| Verification P1 response headers/cache | Fixed: live CSP, Permissions-Policy, `nosniff`, referrer policy, and immutable hashed-asset caching are present. |
| Verification P2 redundant `NO_EXAMPLES` | Fixed: regression test passes. |
| Verification P2 ellipsized install command | Fixed: the visible command is complete and matches Copy command. |

## Structure, accessibility, and links

| Check | Result |
| --- | --- |
| Titles | Pass: unique, plain titles on home, demo, privacy, terms, and 404. |
| `lang`, one h1, one main, heading order, alt text | Pass on every route. |
| Meta description, canonical, favicon | Pass on normal public routes; the 404 correctly uses `noindex`. |
| OG/Twitter metadata | Partial; F-2-4. |
| Deep links and browser Back | Pages and scroll state restore, but focus remains on `<body>`; F-2-2. |
| Designed 404 | Pass: unknown URL returns HTTP 404 with a styled recovery page. |
| Dead-link crawl | Pass: all internal and GitHub links returned 200; the intentionally unknown route returned 404. |
| Header/footer and Privacy/Terms | Pass. |
| Keyboard/axe/console | Skip link works; zero WCAG A/AA axe violations and zero console/page errors in live mobile and desktop contexts. |
| Touch targets | Fail; F-2-3. |
| Reduced motion | Pass in the clean browser suite. |
| First-load size | Pass: built JS is 4.70 KB and CSS is 14.44 KB before gzip. |
| Live/source parity | Pass: live HTML, JS, and CSS hashes match the clean build. |
| Visual identity | Pass: the drafting-paper grid, indigo schema gate, coral/mint status language, clipped controls, and original Contract Loom art are product-specific rather than a generic SaaS template. |

The factory URL verifier loaded the live home page in 564 ms with no errors, one h1, `lang=en`, a main landmark, complete image alt text, and labelled buttons.

## Missed leverage

No finding. Deterministic local validation is the job; an AI step would add cost and uncertainty without improving the core check. Import/sync is not implied for a CLI that already reads Markdown/OpenAPI paths, and GitHub annotations cover the brief’s CI integration.

## What would make this perfect

Use a build-generated, byte-faithful transcript of the shipped CLI demo and test its parity. Register and test every retained behavior sentence, remove the nonexistent release option, focus the destination heading after navigation, raise all mobile targets to 44×44px, and complete per-route social metadata. Then rerun this entire review from a fresh clone and fresh browser contexts; PASS requires zero findings.
