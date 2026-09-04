# api
*the Speechmatics rung, grade 3 — fieldnote's field pipeline, setting for
setting*

> (transcripts/2026-09-04-field-walk.md#p10)
> yeah that sounds like a decent start. Parity with fieldnote would be good since I know that works OK in the field.

## user

Nothing to see. When the mini has a key and a working line out, your notes
come back in the best words this system can get — with two people at a
doorstep written as two people. When it does not, they come back from the
mini's own model instead, and you are not told which, because it is not your
problem.

## spec

Parity means the same provider and the same settings, not an equivalent.
Fieldnote's field runs went through **Speechmatics batch**, and this rung is
that pipeline ported into a file this node owns: the **enhanced** operating
point, **speaker diarization at sensitivity 0.75**, `additional_vocab` built
from `/vocabulary`'s phrases, and the same parse of the `json-v2` answer into
speaker-labelled segments.

**Grade 3, tried first and once.** `/transcribed` walks the ladder and names
the grade it is asking for; this rung answers for three and passes everything
else on. A failure — no line out, a rejected job, a timeout — returns nothing,
and the mini's own model is asked next, so a field day with no signal
transcribes anyway.

**Reachable means a key and an interpreter, and the network is not asked
about in advance.** A pre-flight check costs a round trip and is stale by the
time the job is submitted; the honest test is the attempt, and the attempt
already has a fallback beneath it.

**The key is never in the repo.** It lives in `~/.agent-config.json` under
`speechmatics`, beside the SMS and model credentials, and reaches the child in
its environment rather than on argv — `/off-argv`'s rule, because argv is
readable by any local `ps`. `SPEECHMATICS_API_KEY` in the server's own
environment overrides it, for a box that would rather work that way.

**The script needs nothing installed.** It uses the standard library alone —
multipart written by hand rather than `requests` — so it runs under whatever
python3 the mini has, and a virtual environment moving cannot take
transcription down on a field day. `ffmpeg` re-encodes the clip to 44.1 kHz
mono wav once before submission, because Speechmatics refused the raw browser
container for fieldnote too.

**The job is deleted after it is read.** These are the team's own notes and
there is no reason for a copy to stay on somebody else's disk; fieldnote
deleted them for the same reason and this keeps that.

**What lands in the post.** With more than one speaker, the speaker-labelled
text. With one, the plain words — "A: " in front of a lone canvasser's note
says nothing and reads as damage.

## glossary

- **operating point**: Speechmatics' accuracy/cost setting; `enhanced` is the
  better one and the one fieldnote used.
- **additional vocab**: phrases the recogniser is told to expect, the
  equivalent of whisper's seeded prompt.

## code description

`api.rs` redefines two links of `/transcribed`'s. `transcribe_best_grade`
raises the ladder to 3 when the rung is reachable. `transcribe_rung` answers
only when the grade asked for is 3, runs the script with the clip and the
comma-joined phrases, and returns `{text, rung, grade}` — or passes to
`existing`, which is what makes the mini the fallback.

`api_ready` is the reachability test: a key, an interpreter that exists, and
the script in `site/`. `api_python` takes `MISO_PY` if it is set and otherwise
the first of `~/.local/bin/python3`, homebrew's and the system one that
exists, because a server started by launchd has no brew directory on its PATH.

`api_words` picks the speaker-labelled text over the plain one only when
there really were two speakers.

`assets/tools/transcribe_api.py` is the pipeline itself, and the node owns it
outright: the linker puts it in `site/` with the pages, the deploy ships it
there, and unticking this node takes it away again.
