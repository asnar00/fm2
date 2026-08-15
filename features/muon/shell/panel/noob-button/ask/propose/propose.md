# propose
*the ask box turns proposer: a drafted description, your edit, your OK — then it fires*

> (transcripts/2026-08-15-fm-spec.md#p27)
> 2) it should write me a user-description for the tool, let me edit or OK it, and then fire it. And that needs to happen whether we're connected to the server or not
> *(fired at #p30)*

## spec

Filing a wish was fire-and-forget; now it is a small act of
specification. When an ask heads for the builder — the **send to the
builder** button, or nothing having matched at all — the box opens an
editor holding a **drafted user-description**: the prospective node's
`## user` paragraph (#p85's doctrine: the proposal IS that paragraph,
approved before code). The draft is the ask itself, stated plainly —
no "gains a new ability" ceremony (#p33: the description should
concisely say what the tool does, and the birthplace already travels
as data) — because the edit box is the intelligence in the loop:
rewrite it until it says what you mean,
press **propose**, and the ask files with your approved paragraph,
`status: "proposed"`, and its birthplace.

Offline changes nothing: the draft is assembled on the device and the
fire rides the durable outbox — a proposal made in a tunnel lands when
the network returns. The drafter is a seam, not a commitment: the
dev-session agent (online) or a local model on `/compute` (someday)
can replace the template without the flow noticing.

## user

Ask for something muon can't do yet and it answers with a short
description of what it *would* do — in words meant for you. Fix the
words until they're right, hit **propose**, and that exact paragraph
goes to the builder as the contract for what gets built. Works with no
signal; it sends itself when you're back.

## glossary

- **proposal**: the approved user-description an ask carries — the
  prospective feature's `## user` paragraph, agreed before any code
  exists.

## code description

`propose.index.js` redefines `feature_Ask.file` to park the ask text
instead of sending, and wraps `feature_Ask.go`: after the original
renders, a parked text (the nothing-matched path) opens the editor in
place of the stale filed-note, and any **send to the builder** button
is rewired to open the editor with the query it was rendered for.
`editor()` fills the textarea from `draft()` — "The ‹tool› tool gains
a new ability: ‹your words›…", tool from `/birthplace`'s `context()`
(typeof-guarded, tool-less template without it). `fire()` sends the
`Ask` event with text, proposal, and context through the loop — the
outbox makes it durable — and confirms in the box.

`propose.rs` extends the update chain after `/ask` and `/birthplace`:
an `Ask` carrying a `proposal` finds its entry (by `t`) and stamps
`proposal` and `status: "proposed"`.

`propose.index.css` styles the editor.
