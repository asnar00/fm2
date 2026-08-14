#!/usr/bin/env python3
"""Fetch the on-device transcription artifacts for /loop/dictate/phone.

The repo carries the recipe, not the binaries: this script populates
features/muon/loop/dictate/phone/assets/stt/ (gitignored except engine.js)
with pinned versions of
  - transformers.js (from the npm registry tarball, exact version), and its
    onnxruntime wasm/mjs backends;
  - the whisper-tiny.en ONNX model (q4) from huggingface, with the
    resolved revision recorded in stt/PINNED.

Re-running is idempotent: present files are kept unless --force.
"""
import io
import json
import sys
import tarfile
import urllib.request
from pathlib import Path

TRANSFORMERS_VERSION = "4.2.0"
# transformers' dist vendors the ort .mjs glue but not the .wasm binaries;
# they come from the exact onnxruntime-web build it depends on
ORT_WEB_VERSION = "1.26.0-dev.20260416-b7804b056c"
MODEL_REPO = "onnx-community/whisper-tiny.en"
# q4 pair: the q8 ("quantized") decoder trips a MatMulNBits bug in this ort
# build ("Missing required scale"); q4 loads and transcribes correctly
MODEL_ONNX = ["onnx/encoder_model_q4.onnx",
              "onnx/decoder_model_merged_q4.onnx"]

REPO = Path(__file__).resolve().parent.parent
STT = REPO / "features/muon/loop/dictate/phone/assets/stt"


def fetch(url: str) -> bytes:
    print(f"  fetching {url}")
    with urllib.request.urlopen(url) as r:
        return r.read()


def main():
    force = "--force" in sys.argv
    STT.mkdir(parents=True, exist_ok=True)

    # --- transformers.js + ort backends, from the pinned npm tarball --------
    marker = STT / "transformers.min.js"
    if force or not marker.exists():
        tgz = fetch("https://registry.npmjs.org/@huggingface/transformers/-/"
                    f"transformers-{TRANSFORMERS_VERSION}.tgz")
        n = 0
        with tarfile.open(fileobj=io.BytesIO(tgz)) as tar:
            for m in tar.getmembers():
                name = Path(m.name).name
                if not m.name.startswith("package/dist/"):
                    continue
                if name == "transformers.min.js" or name.endswith(".wasm") \
                        or (name.startswith("ort-") and name.endswith(".mjs")):
                    data = tar.extractfile(m).read()
                    (STT / name).write_bytes(data)
                    n += 1
        print(f"transformers {TRANSFORMERS_VERSION}: {n} files")
    else:
        print("transformers: present, skipping (use --force to refetch)")

    if force or not list(STT.glob("*.wasm")):
        tgz = fetch("https://registry.npmjs.org/onnxruntime-web/-/"
                    f"onnxruntime-web-{ORT_WEB_VERSION}.tgz")
        n = 0
        with tarfile.open(fileobj=io.BytesIO(tgz)) as tar:
            for m in tar.getmembers():
                name = Path(m.name).name
                # only the plain + jsep pairs and the webgpu bundle: the
                # asyncify/jspi variants are never requested (verified by
                # request-logging the engine end to end)
                keep = ("ort-wasm-simd-threaded.wasm", "ort-wasm-simd-threaded.mjs",
                        "ort-wasm-simd-threaded.jsep.wasm", "ort-wasm-simd-threaded.jsep.mjs",
                        "ort.webgpu.bundle.min.mjs")
                if m.name.startswith("package/dist/") and name in keep:
                    (STT / name).write_bytes(tar.extractfile(m).read())
                    n += 1
        print(f"onnxruntime-web {ORT_WEB_VERSION}: {n} wasm/mjs files")

    # --- the model, revision-pinned on first fetch --------------------------
    info = json.loads(fetch(f"https://huggingface.co/api/models/{MODEL_REPO}"))
    sha = info.get("sha", "main")
    model_dir = STT / "models" / "whisper-tiny.en"
    model_dir.mkdir(parents=True, exist_ok=True)
    small = [s["rfilename"] for s in info["siblings"]
             if "/" not in s["rfilename"] and (
                 s["rfilename"].endswith(".json")
                 or s["rfilename"] in ("merges.txt", "vocab.txt"))]
    for rel in small + MODEL_ONNX:
        dest = model_dir / rel
        if dest.exists() and not force:
            continue
        dest.parent.mkdir(parents=True, exist_ok=True)
        dest.write_bytes(fetch(
            f"https://huggingface.co/{MODEL_REPO}/resolve/{sha}/{rel}"))
    (STT / "PINNED").write_text(
        f"transformers.js {TRANSFORMERS_VERSION}\n{MODEL_REPO} @ {sha}\n")
    total = sum(f.stat().st_size for f in STT.rglob("*") if f.is_file())
    print(f"stt assets ready: {total / 1e6:.1f} MB in {STT.relative_to(REPO)}")


if __name__ == "__main__":
    main()
