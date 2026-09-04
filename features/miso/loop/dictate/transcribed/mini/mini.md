# mini
*the local rung, grade 2: one warm whisper on the mini, one clip at a time*

> (transcripts/2026-09-04-field-walk.md#p10)
> yeah that sounds like a decent start. Parity with fieldnote would be good since I know that works OK in the field.

## user

Nothing to see. When the mini cannot reach Speechmatics — no key, no line
out, a field day in a dead spot — your notes are transcribed by the mini
itself, seeded with the same street names, and they arrive the same way. A
note with nothing said in it comes back with nothing said in it.

## spec

The road when the network is not there, which on a canvassing day is often.
**mlx whisper large-v3-turbo**, fieldnote's local settings
(`transcribe_local.py`): ffmpeg to 16 kHz mono, `word_timestamps`, an
`initial_prompt` from `/vocabulary`, and `condition_on_previous_text=False` —
without which one invented phrase is carried through the rest of a clip.
Diarization is fieldnote's too and is deliberately **not** in this cut: it is
a second model and a second resident set, and a later rung.

**A resident worker, not a subprocess per clip.** Loading the weights costs
about half a minute; a note is often shorter than that. `tools/transcriber.py`
(this node's own asset, so it ships in `site/` and leaves with the node) loads
the model once, warms it on a second of silence so the first real clip does
not pay, and then watches a directory: one job file in, one answer file out,
one clip at a time. It re-stamps a heartbeat every five seconds with its pid,
its model and its resident set.

**Reachable means beating.** This rung raises the ladder to 2 only while that
heartbeat is fresh and says `warm`. Not "the model is on disk", not "the
script exists" — a clip queued for a worker that is not running is how a
phone comes to say "transcribing…" for ever. If the worker dies mid-clip the
rung waits fifteen minutes, takes its job file back, and answers nothing; the
clip is retried on the next drain and dropped after five, with a line in the
log.

**Nothing is landed for silence.** Whisper writes "Thank you." and subtitle
credits over a hiss. The audio is silence-trimmed before the model sees it and
a clip with under 0.6 s of sound left is answered as *silent* — which
`/transcribed` treats as an answer, so the post keeps its own words and the
clip is never handed to a worse rung that would guess.

**Its own HOME, its own worker.** The directory it watches is under the blob
root, so a rig started with a scratch HOME talks to its own transcriber and
can never be handed the live server's clips.

**Measured, on the mini it runs on.** Warm with large-v3-turbo the worker
holds **1,543 MB in MLX** and peaked at **1,332 MB** resident while loading;
`ps` shows about 37 MB, because on Apple silicon the weights live in unified
memory through Metal and are not counted as resident — so `ps` is the wrong
instrument here and the heartbeat carries both numbers instead. Call it
**1.5 GB**, against the mini's 8.6 GB shared with the live server, the tunnel
and the cached Qwen models. That is why one clip at a time is a rule and not a
preference, and why diarization waits: pyannote is a second model.

**And it is quick.** Ten seconds of speech took 17.1 s on the first clip after
the warm-up and 4.8 s on the next — well over real time, which is what makes
"the words a moment later" true rather than hopeful.

**The launchd job is written and not loaded.** `tools/com.noob.transcriber.plist`
is a reference plist beside `com.noob.miso.plist`; loading it against the live
server is triage's decision, not a builder's.

## glossary

- **resident worker**: a process holding a loaded model, taking work from a
  directory, so the model's loading cost is paid once.
- **heartbeat**: the worker's stamp saying it is alive and warm; the only
  thing that makes this rung reachable.

## code description

`mini.rs` redefines two links of `/transcribed`'s. `transcribe_best_grade`
raises the ladder to 2 when `mini_ready` sees a fresh, warm heartbeat.
`transcribe_rung` answers only for grade 2: it writes one job file named
`<id>.<nonce>.json`, waits for the answer file of the same name, and returns
`{text, rung, grade}` — or `{silent}`, or nothing at all, which passes to
`existing`. The nonce is what stops a second run of a clip (a retry, an
upgrade) reading the first run's answer.

`mini_prompt` builds "Canvassing in `<constituency>`. Nearby: `<the rest>`."
from `/vocabulary`'s list and cuts it at seven hundred characters on a comma,
so the model is never given half a street name and never more prompt than its
window holds.

`assets/tools/transcriber.py` is the worker. `--once CLIP [PROMPT]` runs one
clip and prints the JSON, for a bench test without the daemon.
