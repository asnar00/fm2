# self-heal
*a failed attempt never destroys words the network already has*

> (transcripts/2026-08-16-fm-spec.md#p21a)
> also - the first recording is showing up on the phone as an error, but correct on the laptop. perhaps a couple of tweaks to the mirroring algorithm would help - the correct version exists in the device network, and it should automatically "heal" a missing file on the phone

## user

If one of your devices can't transcribe a note — no speech engine, no
memory, a broken GPU driver — it no longer wipes the words another device
already worked out. The good version arrives from your other instances and
stays, and the tile shows what was said instead of an error.

## spec

`/mirrored-words` shipped with a known accepted gap, recorded in its own
spec: *"`/dictate`'s own `Transcribed` handler stamps unconditionally, so
a slower local result can momentarily overwrite an equal-grade mirrored
one."* The field found it the first time a device genuinely could not
transcribe: the phone's engine failed, stamped an empty result, and
overwrote the correct transcript the laptop had already shared — leaving
an error where words had been.

The healing was in fact already happening and being undone. Boot order on
a failing device is: local metadata reseeds without stamps, the scheduler
queues a local attempt, `RecIndexed` arrives from the exchange and the
good words are adopted — and *then*, half a second later, the local
attempt fails and blanks them. The mirror was doing its job; the failure
raced it and won.

So the rule this node adds is narrow and absolute: **an empty result may
never replace a non-empty transcript.** A failed attempt reports a
failure, not an erasure. Silence is not information about what was said.

Mechanically the node compares the state before the event with the state
after: when a `Transcribed` carrying empty text has blanked a transcript
that was there a moment ago, the previous text, rung and grade are put
back and the failure stamp (`t_err`) is cleared with them — the note is
not in a failed state, it simply wasn't this device that worked it out.
The scheduler is then re-run, which finds the note already at the best
reachable grade and stops asking for it: the failing device stops
retrying forever, which is the second half of the healing.

Everything else stays as `/mirrored-words` built it. A device with no
words at all still adopts them from `RecIndexed` or a live
`TranscriptShared`; a genuinely better transcript still replaces a rougher
one everywhere. This node only refuses the one write that destroys
information.

## glossary

(no new terms)

## code description

`self-heal.rs` extends the `update` chain, and being the newest node it
composes outermost — so `existing.update` has already run `/dictate`'s
stamping and `/honest-panel`'s error recording by the time it looks.

It claims nothing and passes everything through except a `Transcribed`
event whose `text` is empty. For those it reads the *incoming* state (the
extension's own `state` argument, before the chain touched it) for that
id's transcript. If there was one, the after-state's entry gets its
`transcript`, `t_rung` and `t_grade` restored and `t_err` removed, and
the result goes through `transcribe()` so the scheduler re-evaluates
against the restored grade and drops the intent.

No transcript before the event means there is nothing to protect and the
chain's result stands untouched — which is exactly the path a first-ever
failure on a device with no mirrored words takes, so honest failure
reporting is preserved.
