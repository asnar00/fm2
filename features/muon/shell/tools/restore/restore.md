# restore
*an instance reopens where you left it: the open tool survives a restart*

> (transcripts/2026-08-14-fm-spec-3.md#p46a)
> also, let's make each instance remember its state (which tool we're in, basically) so that if I restart the instance, I come back to where I was when I shut it down

## spec

Which tool is open is per-instance state (`/tools`); this feature makes it survive the instance restarting. Every change to the open tool is remembered on the device; at boot, if a tool was open when the instance last ran, it is reopened — by sending the same event a tap would send, so the tool's own opening behaviour (and any other feature watching) runs identically. Restore is honest about absence: a remembered tool that is no longer in the composition (its feature unticked, its button gone) is not reopened — the instance boots to the launcher rather than into a ghost. Remembering the launcher ("no tool open") is state too.

## user

Restart the app and it opens where you were — in dictate if you were in dictate, the launcher if you'd gone home. Nothing to set up.

## glossary

(no new terms; /instance/ is defined at `/loop/scope`)

## code description

`restore.js` does both halves. It wraps `feature_Loop.apply`: whenever the rendered state's `open_tool` differs from what's stored (`localStorage.muon_open_tool`), it stores the new value — including the empty launcher value. At boot (the poll-until-`feature_Loop`-ready pattern), if the stored id is non-empty, the state's `open_tool` is empty, and a matching `[data-ev="tool_<id>"]` button exists in the rendered launcher, it sends `{type: 'click', ev: 'tool_<id>'}` through the normal update chain. The DOM check is the composition check: no button, no restore.
