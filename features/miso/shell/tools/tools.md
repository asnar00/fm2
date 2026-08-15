# tools
*miso as an operating system: a toolbar of tools along the bottom edge*

> (transcripts/2026-08-14-fm-spec-2.md#p41)
> I like the idea that the "muon" we've built so far is like an operating system - it runs things like "tools" or "apps" (I prefer "tools"), which are organised into "pages" or "toolsets". So in this case, we'd have a tool called "transcribe" which is a button on the main muon screen (a bit like the iphone's app launcher).

> (transcripts/2026-08-14-fm-spec-2.md#p42, draft-phase revision)
> make tools quite small buttons with icons on them, and put them in a horizontal row (a "panel") down next to the little logo-button, across the bottom of the screen. That gives us the whole rest of the screen above it to hold display information. The "back" button can be a little "<" at the left of the tools panel.

> (transcripts/2026-08-14-fm-spec-2.md#p43, draft-phase revision)
> let's introduce some colour discipline for this first run - let's make it white on dark grey background (no outline), and black on light grey when selected?

> (transcripts/2026-08-14-fm-spec-3.md#p88, revision)
> ah - we don't need a separate "<" button to the left of the tool button we selected; just tap the tool button return.

> (transcripts/2026-08-14-fm-spec-2.md#p48, draft-phase revision)
> when we select dictate, the other tool icons (=taps) slide off to the left, the dictate icon slides to leftmost, and the rec/stop buttons are the new tools to the right of dictate - they don't sit above the toolbar.

## spec

miso runs **tools**: each is a small icon button in a **toolbar** — a
horizontal row across the bottom of the screen beside the corner build
stamp — leaving the whole screen above as the display surface. Tapping a
tool opens it in the display area; tapping the open tool's own button
closes it again (#p88 — the `‹` back button is gone). Which tool is open is per-instance state (`Var::<String>::local
("open_tool")`) — navigation never syncs between devices. A tool registers
`{id, label, icon}` (emoji icons for now) on the `tools_list` chain from its
own node; features older than this chain register via a subfeature
(provenance ordering: you can only extend what existed when you were
written). Toolsets (pages of tools) arrive when the row overflows. (The row
is called a toolbar, not a panel — `/panel` is the system panel.)

## user

Your tools sit as small icon buttons along the bottom of the screen. Tap one
to open it above; tap it again to close. Adding or removing a feature from your
product adds or removes its button.

## glossary

- **tool**: a capability with a toolbar button and use of the display
  surface when open.
- **toolbar**: the row of tool buttons along the bottom edge.
- **display surface**: the screen above the toolbar, owned by the open tool.
- **toolset**: a named page of tools (future: when the toolbar overflows).

## code description

`tools.rs` owns four things. `tools_list(state)` is the registry chain — the
base returns `[]`; each tool redefines it to append its entry. `init` marks
launcher-mode by setting `open_tool` to empty (the key's *absence* means this
feature is toggled off — tools then render unconditionally, preserving
pre-toolbar behaviour). `update` handles `tool_<id>` clicks (open; clicking
the open tool's button is a no-op) and `tools_home` (the `‹`, close).
`render` appends the toolbar. Closed: one button per registered tool. Open:
the toolbar becomes the tool's control surface — `‹`, the open tool's icon
(leftmost, `sel`), then whatever the `tool_controls` chain contributes (base:
nothing; the open tool redefines it to add its own buttons). Renders are
whole-DOM swaps, so the "slide" is a mount animation on mode change.

`tool_colour(id)` is the colour seam (base: empty — the monochrome
discipline): a styling feature redefines it per tool id, and `render_toolbar`
emits the colour as a `--tool-colour` custom property with a `tinted` class.

`tools.css` styles the toolbar (safe-area aware, clear of the corner stamp)
and its buttons: white glyph on dark grey, black on light grey when selected,
no outline. Emoji ignore CSS `color`, so icons are forced monochrome with a
grayscale/brightness/invert filter on the icon span.
