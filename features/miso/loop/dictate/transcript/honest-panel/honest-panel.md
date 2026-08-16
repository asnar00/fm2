# honest-panel
*the panel never shows nothing: waiting, transcribing, or failed — with the reason*

> (transcripts/2026-08-14-fm-spec-3.md#p61)
> ok, so I'm hearing the audio but not seeing the transcript pop up

## user

Tap a note and the panel always tells you where its transcript is: the words themselves, "transcribing…", "waiting to transcribe (2 ahead)" — first-ever use also downloads the speech model during this wait, so give it a few minutes on wifi — or "transcription failed" with the reason, which is worth screenshotting at us.

## spec

The reported silence was `/transcript` lying by omission: a playing note with no transcript showed no panel unless it happened to be the scheduler's current target. Now every playing note gets a panel telling the truth about its transcript: **transcribing…** when it's the active one (as before); **waiting to transcribe (N ahead)** when other notes are queued in front (the scheduler is oldest-first, and the first note is also where the speech model downloads — so a fresh install legitimately waits); **transcription failed** with the engine's actual error when the attempt was stamped failed. The error was already sent by `/phone` and dropped by `/dictate`'s stamping — this node keeps it (`t_err`), which makes every phone its own diagnostic readout: the failure reason appears exactly where the audio is.

## rule

A playing note's panel always states the transcript's true condition — present (the text), in progress, queued (with position), or failed (with reason). Absence of a panel means absence of a recording playing, nothing else.

## glossary

(no new terms)

## code description

`honest.rs` extends two chains. `update`, after existing: a `Transcribed` event with `failed: true` also stamps its `err` onto the file as `t_err` — `/dictate`'s stamping keeps text/rung/grade; this keeps the reason. `render`, after existing: for a playing note with an empty transcript that the parent's panel didn't cover, it appends the same `.transcript-panel` markup with the honest state — `t_err` present → "transcription failed" + the error (escaped); otherwise "waiting to transcribe (N ahead)", N counting `here` files earlier in the grid still needing this grade. The parent handles the text and the active-target "transcribing…"; the conditions are disjoint, so exactly one panel ever renders.
