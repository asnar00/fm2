# tools
*muon as an operating system: tools on a launcher screen*

> (transcripts/2026-08-14-fm-spec-2.md#p41)
> I like the idea that the "muon" we've built so far is like an operating system - it runs things like "tools" or "apps" (I prefer "tools"), which are organised into "pages" or "toolsets". So in this case, we'd have a tool called "transcribe" which is a button on the main muon screen (a bit like the iphone's app launcher). So I think that's what we should build first.

## spec

muon's main screen becomes a launcher: a grid of `/tool` buttons (the iPhone
home-screen idea). Tapping a tool opens it full-screen with a home chip to
return. Which tool is open is per-instance state (`Var::<String>::local
("open_tool")` — your phone's navigation never moves your laptop's). A tool
registers by extending the `tools_list` chain with `{id, label}` — any
feature newer than this node can register itself from its own code;
provenance ordering means older features (like `/tap`) register via a new
subfeature instead. Toolsets/pages arrive when one screen of tools overflows.

## user

The muon screen shows your tools as buttons. Tap one to open it; tap the
`‹ tools` chip to come home. Adding or removing a feature from your product
adds or removes its button.

## glossary

- **tool**: a capability with a button on the launcher and a full-screen
  surface when open.
- **launcher**: the home screen — the grid of tool buttons.
- **toolset**: a named page of tools (future: when one screen overflows).

## code description

`tools.rs` owns four things. `tools_list(state)` is the registry chain — the
base returns `[]`; each tool redefines it to append its entry. `init` marks
launcher-mode by setting `open_tool` to empty (the key's *absence* means the
launcher feature itself is toggled off — tools then render unconditionally,
preserving pre-launcher behaviour). `update` handles `tool_<id>` clicks (open)
and `tools_home` (back). `render` extends the chain: launcher mode appends
the grid built from `tools_list`; open mode appends the home chip — the open
tool's own render contributes its surface.

`tools.css` styles the grid and the home chip.
