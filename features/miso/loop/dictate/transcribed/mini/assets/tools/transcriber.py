#!/usr/bin/env python3
"""The mini's resident transcriber: one warm whisper large-v3-turbo, one clip
at a time, watching a directory.

    transcriber.py [--root DIR] [--once CLIP [PROMPT]]

Resident (no --once) it watches `<root>/whisper/in` for job files, writes
answers to `<root>/whisper/out`, and re-stamps `<root>/whisper/alive` every
few seconds with its pid, its model and its resident set in megabytes. The
server's mini rung reads that stamp to decide whether this rung is reachable
at all: no heartbeat, no grade 2, and nothing is queued that nothing can do.

`<root>` defaults to $HOME/.miso-blobs, so a rig with its own HOME gets its
own transcriber and cannot be handed the live server's clips.

Settings are fieldnote's transcribe_local.py, minus diarization (a later
rung): ffmpeg to 16 kHz mono, `word_timestamps`, an `initial_prompt` from the
caller, and `condition_on_previous_text=False` — whisper otherwise carries a
hallucination forward through a whole clip. Silence is trimmed before the
model sees it, and a clip that is *only* silence lands nothing at all rather
than the subtitle credits whisper invents over a hiss.

A job file is {"clip": path, "prompt": text}; the answer is {"text": ...} or
{"error": ...}. The name is <id>.<nonce>.json at both ends, so a clip
transcribed twice — an upgrade, a retry — can never read the earlier answer.

Run it by hand while working; the launchd job is tools/com.noob.transcriber.plist,
which is a REFERENCE plist and is not loaded by anything here.
"""
import argparse
import json
import os
import pathlib
import resource
import subprocess
import sys
import tempfile
import time
import warnings

warnings.filterwarnings("ignore")

MODEL = "mlx-community/whisper-large-v3-turbo"
HEARTBEAT = 5.0
# a clip with less than this much sound in it, after the silence is cut, is
# silence: whisper writes "Thank you." and "Subtitles by ..." over a hiss, and
# a note that says that is worse than a note that says nothing.
MIN_SOUND = 0.6


def root_dir(arg):
    if arg:
        return pathlib.Path(arg)
    return pathlib.Path(os.environ.get("HOME", ".")) / ".miso-blobs"


def rss_mb():
    """Peak resident set in MB. macOS reports ru_maxrss in bytes and Linux in
    kilobytes, and guessing from the magnitude gets it wrong at exactly the
    sizes that matter here, so the platform is asked."""
    v = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    return round(v / (1024 * 1024) if sys.platform == "darwin" else v / 1024, 1)


def mlx_mb():
    """What MLX itself is holding. On Apple silicon the weights live in
    unified memory through Metal, which `ps` does not count as resident — so
    the honest number for "how much of the mini is this using" is this one
    plus the process's own. Zero if MLX cannot say."""
    try:
        import mlx.core as mx
        return round(mx.get_active_memory() / (1024 * 1024), 1)
    except Exception:
        return 0.0


def to_wav(clip):
    """16 kHz mono, silence trimmed from both ends and any gap over a second
    squeezed to one. Returns (path, seconds_of_sound) or (None, 0)."""
    wav = tempfile.NamedTemporaryFile(suffix=".wav", delete=False)
    wav.close()
    trim = ("silenceremove=start_periods=1:start_duration=0.1:start_threshold=-45dB:"
            "stop_periods=-1:stop_duration=1.0:stop_threshold=-45dB")
    r = subprocess.run(
        ["ffmpeg", "-y", "-i", clip, "-ac", "1", "-ar", "16000", "-vn",
         "-af", trim, wav.name],
        capture_output=True)
    if r.returncode != 0 or not os.path.exists(wav.name):
        return None, 0.0
    size = os.path.getsize(wav.name)
    if size <= 44:
        return wav.name, 0.0
    # 16 kHz, 16-bit, mono: two bytes a sample, and a 44-byte wav header.
    return wav.name, (size - 44) / 32000.0


def transcribe(model_holder, clip, prompt):
    wav, sound = to_wav(clip)
    if wav is None:
        return {"error": "ffmpeg could not take the audio out of that clip"}
    try:
        if sound < MIN_SOUND:
            return {"text": "", "silent": True, "sound": round(sound, 2)}
        started = time.time()
        result = model_holder.transcribe(
            wav,
            path_or_hf_repo=MODEL,
            initial_prompt=prompt or None,
            word_timestamps=True,
            condition_on_previous_text=False,
        )
        return {
            "text": (result.get("text") or "").strip(),
            "sound": round(sound, 2),
            "took": round(time.time() - started, 1),
            "rss_mb": rss_mb(),
            "mlx_mb": mlx_mb(),
        }
    except Exception as e:
        return {"error": f"{type(e).__name__}: {e}"}
    finally:
        try:
            os.unlink(wav)
        except OSError:
            pass


def load_model():
    try:
        import mlx_whisper
    except ImportError:
        sys.exit("transcriber: mlx_whisper is not installed for this python — "
                 "run it from a venv that has it "
                 "(~/nøøb/experiments/fieldnote/venv/bin/python3)")
    return mlx_whisper


def beat(root, pid, warm):
    # the directories are remade on every beat, not once at startup: a rig
    # resetting its scratch home takes them out from under a worker that is
    # perfectly alive, and a worker that then watches a directory nobody can
    # write to is worse than one that is plainly dead.
    (root / "whisper" / "in").mkdir(parents=True, exist_ok=True)
    (root / "whisper" / "out").mkdir(parents=True, exist_ok=True)
    (root / "whisper" / "alive").write_text(json.dumps({
        "at": int(time.time() * 1000), "pid": pid, "model": MODEL,
        "warm": warm, "rss_mb": rss_mb(), "mlx_mb": mlx_mb()}))


def sweep(d, older_than=3600):
    """Job and answer files nothing is coming back for. A server that died
    mid-job leaves one of each; an hour is long past any real job."""
    now = time.time()
    for f in d.glob("*.json"):
        try:
            if now - f.stat().st_mtime > older_than:
                f.unlink()
        except OSError:
            pass


def serve(root):
    inbox = root / "whisper" / "in"
    outbox = root / "whisper" / "out"
    inbox.mkdir(parents=True, exist_ok=True)
    outbox.mkdir(parents=True, exist_ok=True)
    sweep(inbox)
    sweep(outbox)
    pid = os.getpid()
    beat(root, pid, False)
    print(f"transcriber: loading {MODEL}", flush=True)
    mw = load_model()
    # one silent warm-up so the FIRST real clip is not the one that pays for
    # loading the weights: the rung's patience is finite and a cold first job
    # is exactly when it would run out.
    warm = tempfile.NamedTemporaryFile(suffix=".wav", delete=False)
    warm.close()
    subprocess.run(["ffmpeg", "-y", "-f", "lavfi", "-i", "anullsrc=r=16000:cl=mono",
                    "-t", "1", warm.name], capture_output=True)
    try:
        mw.transcribe(warm.name, path_or_hf_repo=MODEL)
    except Exception as e:
        print(f"transcriber: warm-up failed ({type(e).__name__}: {e})", flush=True)
    os.unlink(warm.name)
    print(f"transcriber: warm, {rss_mb()} MB peak resident + {mlx_mb()} MB in MLX, "
          f"watching {inbox}", flush=True)
    last = 0.0
    while True:
        now = time.time()
        if now - last > HEARTBEAT:
            beat(root, pid, True)
            last = now
        jobs = sorted(inbox.glob("*.json"), key=lambda p: p.stat().st_mtime)
        if not jobs:
            time.sleep(0.4)
            continue
        job = jobs[0]
        try:
            spec = json.loads(job.read_text())
        except (OSError, ValueError):
            job.unlink(missing_ok=True)
            continue
        print(f"transcriber: {job.name}", flush=True)
        answer = transcribe(mw, spec.get("clip", ""), spec.get("prompt", ""))
        (outbox / job.name).write_text(json.dumps(answer))
        job.unlink(missing_ok=True)
        note = answer.get("error") or (
            "silence" if answer.get("silent") else
            f"{len(answer.get('text', ''))} characters in {answer.get('took')}s, "
            f"{answer.get('rss_mb')} MB peak + {answer.get('mlx_mb')} MB in MLX")
        print(f"transcriber: {job.name} -> {note}", flush=True)
        beat(root, pid, True)


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--root", help="blob root (default $HOME/.miso-blobs)")
    ap.add_argument("--once", nargs="+", metavar=("CLIP", "PROMPT"),
                    help="transcribe one clip and print the JSON, no daemon")
    args = ap.parse_args()
    if args.once:
        mw = load_model()
        clip = args.once[0]
        prompt = args.once[1] if len(args.once) > 1 else ""
        print(json.dumps(transcribe(mw, clip, prompt)))
        return
    serve(root_dir(args.root))


if __name__ == "__main__":
    main()
