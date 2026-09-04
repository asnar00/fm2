#!/usr/bin/env python3
"""Speechmatics batch transcription — this node's own copy of the pipeline
that ran in the field for fieldnote (its transcribe.py, provider
"speechmatics"), with its settings unchanged: the **enhanced** operating
point, speaker diarization at **sensitivity 0.75**, and `additional_vocab`
built from the caller's phrase list.

    transcribe_api.py <clip> [phrase,phrase,...]

Stdout is one JSON object: {text, raw_text, segments, diarized, provider}.
Anything that goes wrong is {error: "..."} and a non-zero exit; nothing is
ever printed that could carry the key.

The key comes from the environment (SPEECHMATICS_API_KEY) and never from
argv — the same rule /off-argv wrote for the SMS credentials, because argv is
readable by any local `ps` and the environment of another user's process is
not.

Only the standard library is used, so this runs under whatever python3 the
mini has and needs no virtual environment: a field day is a bad time to
discover an interpreter has moved.
"""
import json
import os
import subprocess
import sys
import tempfile
import time
import urllib.error
import urllib.request
import uuid


def ffmpeg_path():
    """ffmpeg by path, not by PATH: the server runs under launchd with
    /usr/bin:/bin only, and ffmpeg is Homebrew's (found live 2026-09-04 —
    ash's 15:30 clip came back "no words" because `ffmpeg` was not found).
    FFMPEG in the environment wins; then the usual homes; then PATH."""
    import shutil
    cands = [os.environ.get("FFMPEG", ""), "/opt/homebrew/bin/ffmpeg",
             "/usr/local/bin/ffmpeg", shutil.which("ffmpeg") or ""]
    for c in cands:
        if c and os.path.exists(c):
            return c
    return "ffmpeg"

API_BASE = "https://asr.api.speechmatics.com/v2"


def die(msg):
    print(json.dumps({"error": msg}))
    sys.exit(1)


def load_key():
    key = os.environ.get("SPEECHMATICS_API_KEY", "").strip()
    if not key:
        die("SPEECHMATICS_API_KEY is not in the environment")
    return key


def multipart(fields, files):
    """One multipart/form-data body, built by hand so the standard library is
    all this needs. `fields` is name -> (value, content_type); `files` is
    name -> (filename, bytes, content_type)."""
    boundary = "----miso" + uuid.uuid4().hex
    out = bytearray()
    for name, (value, ctype) in fields.items():
        out += f"--{boundary}\r\n".encode()
        out += f'Content-Disposition: form-data; name="{name}"\r\n'.encode()
        if ctype:
            out += f"Content-Type: {ctype}\r\n".encode()
        out += b"\r\n" + value.encode() + b"\r\n"
    for name, (filename, blob, ctype) in files.items():
        out += f"--{boundary}\r\n".encode()
        out += (f'Content-Disposition: form-data; name="{name}"; '
                f'filename="{filename}"\r\n').encode()
        out += f"Content-Type: {ctype}\r\n\r\n".encode()
        out += blob + b"\r\n"
    out += f"--{boundary}--\r\n".encode()
    return bytes(out), f"multipart/form-data; boundary={boundary}"


def call(key, method, path, body=None, ctype=None, timeout=60):
    req = urllib.request.Request(f"{API_BASE}{path}", data=body, method=method)
    req.add_header("Authorization", f"Bearer {key}")
    if ctype:
        req.add_header("Content-Type", ctype)
    with urllib.request.urlopen(req, timeout=timeout) as r:
        raw = r.read().decode("utf-8")
    return json.loads(raw) if raw.strip() else {}


def submit_job(key, audio_path, language="en", vocab=None):
    """fieldnote's config, field for field."""
    transcription_config = {
        "language": language,
        "operating_point": "enhanced",
        "diarization": "speaker",
        "speaker_diarization_config": {
            # fieldnote's value. Default is 0.5; higher splits speakers harder.
            "speaker_sensitivity": 0.75,
        },
    }
    if vocab:
        transcription_config["additional_vocab"] = [{"content": v} for v in vocab]
    config = {"type": "transcription", "transcription_config": transcription_config}
    with open(audio_path, "rb") as f:
        blob = f.read()
    body, ctype = multipart(
        {"config": (json.dumps(config), "application/json")},
        {"data_file": (os.path.basename(audio_path), blob, "audio/wav")},
    )
    return call(key, "POST", "/jobs", body, ctype, timeout=300)["id"]


def wait_for_job(key, job_id, poll=3, timeout=900):
    deadline = time.time() + timeout
    while time.time() < deadline:
        status = call(key, "GET", f"/jobs/{job_id}", timeout=30)["job"]["status"]
        if status == "done":
            return
        if status in ("rejected", "deleted", "expired"):
            raise RuntimeError(f"Speechmatics job ended in {status}")
        time.sleep(poll)
    raise TimeoutError(f"Speechmatics job {job_id} not done after {timeout}s")


def fetch_transcript(key, job_id):
    return call(key, "GET", f"/jobs/{job_id}/transcript?format=json-v2", timeout=60)


def delete_job(key, job_id):
    """Best effort, and it matters: these are the team's own notes and there
    is no reason for a copy of them to sit on someone else's disk."""
    try:
        call(key, "DELETE", f"/jobs/{job_id}?force=true", timeout=30)
    except Exception as e:
        print(f"  warning: delete failed: {type(e).__name__}", file=sys.stderr)


def relabel(speaker_id, seen):
    if speaker_id not in seen:
        seen[speaker_id] = chr(ord("A") + len(seen))
    return seen[speaker_id]


def parse_results(payload):
    """json-v2 tokens -> grouped speaker segments + plain text. fieldnote's
    parse, unchanged: punctuation glues to the word before it, and a run of
    tokens by one speaker is one segment."""
    seen = {}
    grouped = []
    raw_words = []
    for tok in payload.get("results", []):
        alts = tok.get("alternatives") or []
        if not alts:
            continue
        text = alts[0].get("content", "")
        if not text:
            continue
        speaker = relabel(alts[0].get("speaker", "UU"), seen)
        is_punct = tok.get("type") == "punctuation"
        attaches_to = tok.get("attaches_to", "previous")
        start = tok.get("start_time", 0.0)
        end = tok.get("end_time", start)
        if is_punct and attaches_to == "previous" and raw_words:
            raw_words[-1] += text
        else:
            raw_words.append(text)
        if grouped and grouped[-1]["speaker"] == speaker:
            sep = "" if (is_punct and attaches_to == "previous") else " "
            grouped[-1]["text"] += sep + text
            grouped[-1]["end"] = end
        else:
            grouped.append({"speaker": speaker, "start": start, "end": end, "text": text})
    for g in grouped:
        g["text"] = g["text"].strip()
        g["start"] = round(g["start"], 2)
        g["end"] = round(g["end"], 2)
    return grouped, " ".join(raw_words).strip()


def to_wav(clip):
    """Speechmatics refused fieldnote's raw browser webm ("invalid audio") and
    takes wav reliably, so the clip is re-encoded once. 44.1 kHz mono, which
    is fieldnote's choice: diarization does better with the extra spectrum and
    Speechmatics handles it natively."""
    wav = tempfile.NamedTemporaryFile(suffix=".wav", delete=False)
    wav.close()
    r = subprocess.run(
        [ffmpeg_path(), "-y", "-i", clip, "-ac", "1", "-ar", "44100", "-vn", wav.name],
        capture_output=True)
    if r.returncode != 0 or os.path.getsize(wav.name) == 0:
        os.unlink(wav.name)
        die("ffmpeg could not take the audio out of that clip")
    return wav.name


def main():
    if len(sys.argv) < 2:
        die("usage: transcribe_api.py <clip> [phrase,phrase,...]")
    clip = sys.argv[1]
    if not os.path.exists(clip):
        die(f"no clip at {clip}")
    vocab = [v.strip() for v in (sys.argv[2] if len(sys.argv) > 2 else "").split(",")
             if v.strip()]
    key = load_key()
    wav = to_wav(clip)
    try:
        job_id = submit_job(key, wav, vocab=vocab or None)
    except urllib.error.HTTPError as e:
        os.unlink(wav)
        die(f"Speechmatics refused the job ({e.code})")
    except Exception as e:
        os.unlink(wav)
        die(f"could not reach Speechmatics ({type(e).__name__})")
    os.unlink(wav)
    try:
        wait_for_job(key, job_id)
        payload = fetch_transcript(key, job_id)
    except Exception as e:
        delete_job(key, job_id)
        die(f"Speechmatics job {job_id} failed ({type(e).__name__})")
    delete_job(key, job_id)
    segments, raw_text = parse_results(payload)
    print(json.dumps({
        "text": "\n\n".join(f"{s['speaker']}: {s['text']}" for s in segments),
        "raw_text": raw_text,
        "segments": segments,
        "diarized": True,
        "provider": "speechmatics",
    }))


if __name__ == "__main__":
    main()
