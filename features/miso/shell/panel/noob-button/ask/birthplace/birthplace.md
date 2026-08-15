# birthplace
*an ask carries where it was born: the open tool travels with the wish*

> (transcripts/2026-08-15-fm-spec.md#p27)
> The system should be smart enough to know 1) that I'm in the tap tool right now, so my request probably pertains to it
> *(fired at #p30: "OK let's do it")*

## spec

A wish made inside a tool is probably about that tool. This node makes
the context part of the record: a filed ask now carries the tool that
was open when it was asked (`tool`) and that tool's registering
feature path (`at`) — so the builder knows the probable parent node
before reading a word, and the wish arrives with its provenance
already placed in the tree. Asked from the launcher, an ask carries
nothing extra — absence stays honest.

## user

When you ask for something while using a tool, miso remembers which
tool you were in — so what you asked for lands with the builder
already knowing what it's probably about.

## glossary

- **birthplace**: the open tool (and its feature path) at the moment
  an ask was filed.

## code description

`birthplace.index.js` owns `context()` — the open tool from loop state
and, when `/chooser`'s catalog is loaded, the path of the node whose
stamped `tool` matches — and redefines `feature_Ask.file` to send the
`Ask` event with those fields alongside the text (a later node may
redefine `file` again and reuse `context()`; that is the seam).

`birthplace.rs` extends the update chain after `/ask`: an `Ask` event
carrying `tool`/`at` finds the entry just appended to the user-scoped
`asks` list (matched by its `t`) and stamps the fields into it.
