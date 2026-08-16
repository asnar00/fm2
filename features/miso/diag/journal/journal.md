# journal
*features say what they are doing — but only the ones you have asked to hear*

> (transcripts/2026-08-16-fm-spec.md#p23)
> We should move to a system where logging statements are pervasive, but enabled at runtime on a per feature basis. So if we're working on transcription, we enable logging for transcription and have at it, then silence it once we're done (except for basic stuff)

> (transcripts/2026-08-16-fm-spec.md#p22, the complaint that prompted it)
> I feel like we need more pervasive logging as part of the black box. A lot of these questions devolve to "what actually ran and what did it return"

## user

Switch on the parts of miso you want to hear from, and they narrate
themselves — into the console, into the flight recorder, and home to the
server. Switch them off and they go quiet again. Turning on a feature
turns on everything inside it, so asking to hear "dictate" gets you the
recorder, the mirror and the speech engine together.

## spec

A whole day of questions reduced to one question — *what actually ran and
what did it return* — and each answer cost an excavation: file timestamps
to infer a device, a DOM-by-DOM reproduction to find a vanished picker,
arithmetic after the fact to explain an out-of-memory. `/engine-receipts`
was built that morning as one feature's private telemetry and its spec
names the general form as deliberately not built. This is the second ask,
so the promotion rule (notes.md #p18) fires and the general form is built.

**The call site carries no bookkeeping.** A feature writes `fm_log(…)`
and nothing else; **the linker supplies the node path**, exactly as it
supplies node paths for `/context-manager`'s gates. A log line therefore
cannot claim the wrong feature, and none of them drift when a node is
regrouped — which matters in a tree where regrouping is a routine event.
The same rewrite serves both languages: chain code and page fragments.

**The switch is the context manager wearing a different hat.** A
user-scoped `feature_log` var maps node path → on, with the prefix
semantics ticks already use: switch on `miso/loop/dictate` and everything
beneath it speaks. **Absent means off**, the mirror image of
`feature_ticks` where absent means on. Per #p24 there is one level for
now — on is verbose — and named levels (verbose / minimal / off) come
when one is missed rather than in anticipation.

**The transport already existed.** Lines ride `/blackbox`, which batches,
bounds by age and count, survives offline, ships on visibility, reconnect
and page-hide, and is ingested into a rotating log on the mini. So log
lines land beside the event deltas they explain, and **replay with them**:
the record of what ran is reconstructable after the fact, not merely
watchable live. Nothing new was built to deliver anything.

Because the switch is a per-user var, it can be thrown from outside the
device: `tools/set_log.py` writes the var and publishes it, and the
existing long-poll reaches an open instance in about half a second.
Turning on transcription logging on a phone from a terminal, watching it,
and turning it off again is the capability this whole day lacked.

Per #p24, log lines may carry content — it is one user's own data on
their own devices. That decision is worth revisiting the day miso has a
second user, and the receipt precedent (shape, never words) is the
fallback if it ever needs one.

**Open question, deliberately not answered here:** whether a feature may
gain `fm_log` lines by editing its file in place, or whether logging is a
behaviour change requiring a subfeature. The letter of the law says the
latter, which would make logging unusable; the sense of it says a log
line is closer to a comment than a capability. Ruling wanted before
logging spreads through the tree.

## glossary

- **journal**: what a feature says about its own working, when switched on.
- **switched on**: this node's path, or an ancestor of it, marked true in
  the user's `feature_log`.

## code description

`journal.lib.rs` (verbatim library) holds two thread-locals: the paths
switched on for this user, and the lines gathered so far. `fm_journal_arm`
refreshes the first from state (parsed properly, since it runs once per
turn rather than once per call); `fm_journal_on` does the prefix walk;
`fm_log_at(path, msg)` — what `fm_log(msg)` is rewritten into — appends
when on and returns immediately when not; `fm_journal_drain` empties the
buffer.

`journal.rs` extends `on_event`, the whole-turn chain: arm the switches
from the incoming state, let the chain run and log as it goes, then append
whatever was gathered to the outgoing state as `_log` — the route `_send`
already uses for outbound messages, which keeps `client.wasm`'s
zero-import law untouched and needs no signature change anywhere in the
tree.

`journal.js` drains `_log` on every apply (deleting it, so state never
grows), prints to the console, and pushes each line into `/blackbox`'s
entries with the `/instance` id attached. It also defines the page-side
`fm_log_at`, which does its own on-check against `feature_log` in state.
Page fragments load in composition order, so `fm_log` is for use inside
functions, not at fragment load time.

The linker half (`tools/fmlink.py`, scaffolding as the tick gates are):
`node_path()` gives a feature's tree-global address, and `log_paths()`
rewrites `fm_log(` to `fm_log_at("<path>", ` in both chain code and page
fragments. Without this node in the composition nothing calls `fm_log_at`,
and the rewrite is inert.
