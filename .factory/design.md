# Visual thesis: Contract Loom

## Direction and rationale

The site uses **generative geometry** as an explanatory system. API examples are irregular human-made payloads; schemas are precise gates. Repeated nodes, connector lines, and rectangular apertures visualize payload fragments being tested and either aligned or rejected. The geometry is intentionally plotter-like rather than glossy: this is a developer tool that should feel inspectable, deterministic, and calm.

The key composition is an original generated illustration of coral data fragments converging through an indigo schema gate into orderly mint rows. It carries no embedded text, so the documentation remains the source of meaning and accessibility.

## Palette

Light mode is ink on warm drafting paper; dark mode becomes a navy terminal sheet rather than simply inverting.

| Token | Light | Dark | Meaning |
| --- | --- | --- | --- |
| background | `#F5F1E8` | `#10182B` | paper / night terminal |
| surface | `#FFFDF7` | `#17233A` | documentation planes |
| text | `#172036` | `#F5F1E8` | deep blueprint ink |
| muted | `#536071` | `#B7C2D2` | annotations |
| accent | `#3346A8` | `#93A5FF` | schema structure |
| accent contrast | `#FFFFFF` | `#10182B` | readable action text |
| signal | `#B83A2A` | `#FF806C` | stale/misaligned examples |
| success | `#14755A` | `#5AD6AE` | passing examples |
| warning | `#8A5A00` | `#F1BC58` | incomplete mappings |
| rule | `#C9C4B8` | `#3A4963` | grid and dividers |

All body text and controls meet WCAG AA against their paired backgrounds. Status always includes an icon and words; color never carries meaning alone.

## Typography

- Interface and prose: `Inter`, self-hosted WOFF2, 400/600/700. Its open forms stay readable in dense reference copy.
- Code and measurements: `JetBrains Mono`, self-hosted WOFF2, 400/600. Tabular figures make line numbers and output easy to scan.
- Scale: 14, 16, 20, 28, and responsive 48–72px. Body stays at 16px minimum with 1.6 leading and a 68-character measure.

## Spacing and layout

An 8px base rhythm (`4, 8, 16, 24, 32, 48, 64, 96`) aligns prose with the geometry. A 12-column desktop field becomes a deliberate single-column sequence at 390px: navigation collapses to essentials, hero art moves below the install command, and comparison rows stack without horizontal scrolling. Content maxes at 1184px.

Rules and coordinate labels replace generic card grids. Bordered regions only denote genuinely independent artifacts: the terminal recording, validation states, and install snippet.

## Interaction grammar

Actions are square-edged with one clipped corner, echoing a schema aperture. Hover moves a connector underline by 4px. Focus uses a 3px coral outline plus offset, visible in both themes. Copy buttons acknowledge success in-place. The terminal recording has explicit Play/Pause and Restart controls, keyboard labels, and a live but non-interruptive status.

## Motion

On entry, geometry resolves from scattered nodes to aligned rows over 600ms. The terminal recording advances at reading speed only after the user presses Play; nothing autoplays or loops. UI feedback uses 160–220ms transform/opacity transitions. Under `prefers-reduced-motion: reduce`, transforms and progressive reveals are removed and the final states appear immediately.

## Asset plan and provenance

- `site/public/assets/contract-loom.webp`: generated specifically for this product with `/opt/fleet/lib/gen-image.sh` using the factory `factory-image` deployment, then converted locally to WebP at ≤300 KB. Prompt: “Editorial generative geometry for an API developer tool: on warm ivory drafting paper, irregular coral JSON-like data tiles and small circular nodes travel left-to-right along thin navy connector lines, pass through a precise indigo rectangular schema gate at center, and emerge as orderly mint aligned rows; flat screenprint/plotter aesthetic, subtle paper grain, crisp geometric shapes, wide landscape composition, strong negative space, no words, no letters, no logos, no UI screenshot, no gradients, no watermark.” License: original project asset under this repository’s MIT license.
- `site/public/assets/contract-loom-social.png`: a 1200×630 local crop of the original Contract Loom art for link previews. No new generative model was used.
- `site/public/apple-touch-icon.png`: a hand-drawn raster version of the repository SVG gate mark, using the documented palette.
- Grid, arrow, and status marks are hand-authored CSS/SVG primitives in the site source and inherit the repository MIT license.

The generated image is meaningful; its alt text explains the input → gate → aligned-output concept. Decorative micro-geometry is hidden from assistive technology.
