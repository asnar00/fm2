# ask
*the nøøb surface's first brick: say what you want — muon finds the tool, or files the wish*

> (transcripts/2026-08-15-fm-spec.md#p2)
> 4) agent hookup ("do x" -> find + introduce/use tool, or build tool for next update)

## spec

The nøøb button's destiny is "how do I use this?" / "do xyz" / "I need
xyz" (#p53, #p70); this node is the ladder's first rung, in two moves.

**Find.** The system panel grows an ask box. What you type is matched
two ways: against the toolbar's tools (label words) — a hit offers
**open**, which puts you straight in the tool — and against the feature
tree's names, purposes and intros — hits appear as ordinary chooser
rows, so the introduction is the chooser's own machinery: tap to read
the feature's paragraph, drill to its page. Surfacing what exists is
grade one of the derivation ladder (surface → compose → build).

**File.** Every ask can be sent to the builder — the button under the
results; when nothing matched, the ask files itself and says so. A
filed ask is appended to the user-scoped `asks` var
(`{t, text, status: "asked"}`), travels like all state, and persists
on the server as `/tmp/muon-vars/user.<name>.asks.json` — where the
dev loop reads it at session start, each wish arriving with its
provenance already born. Deploy prints the unaddressed asks it finds
there, beside its nodeless-release warning: shipping without answering
becomes visible at the moment of shipping.

The agent behind the box is the dev-session agent on a delay (the
doctrine's starting point); the proposed/in-progress lifecycle states
on the feature list are the named next rungs, not this node.

## user

Open the system panel and ask for what you want — "count my taps",
"record a note", "I need a timer". If a tool can already do it, muon
shows you the tool and opens it for you; if a feature page explains it,
it's a tap away. If muon doesn't have it, your ask is filed for the
builder — with your name and words on it — and lands in front of the
agent that grows muon.

## glossary

- **ask**: a user's wish, in their words — the unit the nøøb surface
  answers, by surfacing what exists or by becoming provenance for what
  doesn't.

## code description

`ask.rs` claims the `Ask` event: it appends
`{t, text, status: "asked"}` (timestamp and text from the event data)
to the user-scoped `asks` var — a JSON list in a string var, the
`feature_ticks` pattern — and `/scope` does the rest: sync to the
user's instances, persistence on the server.

`ask.index.js` inserts the ask row (input + **ask** button) into the
panel above the feature list. Submit runs `find()`: tool hits come
from the composed catalog (`catalog()` reads the `tools_catalog` state
var `/tools` stamps at init — the toolbar's DOM renders only the open
tool in open mode, so it falls back to the DOM only when state offers
nothing) and render as open-chips that send the tool's own event and
dismiss the panel; tree hits come from
`feature_Chooser.load()`'s flat list (typeof-guarded — without the
chooser there are simply no tree results), scored by word overlap
against name, purpose and intro, top three rendered via
`feature_Chooser.row()` with `byPath` registered so the chooser's tap
machinery works unmodified. Below the results: **send to the builder**
files the ask (an `Ask` event carrying `Date.now()` and the text); with
no results at all the ask files immediately and the box says so.

`ask.index.css` styles the row, the results strip and the open-chips.

Deploy's warning lives in deploy.sh (scaffolding): after shipping, it
reads `/tmp/muon-vars/user.*.asks.json` on the mini and prints every
entry still `"asked"`.
