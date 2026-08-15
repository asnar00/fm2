#!/usr/bin/env python3
"""Fetch the pinned potion-base-8M static-embedding artifacts and convert
them into muon's own on-device format (see loop/compute/semantic-find):

  vocab.json   token strings in row order (WordPiece, ## continuations)
  table.bin    int8 token->vector table, row-major [29528 x 256]
  scales.bin   one f32 per row: dequantise as int8 * scale

The recipe lives in git; the binaries do not (the stt pattern). Re-running
is deterministic: same pin, same bytes out. The catalog embedder
(tools/potion_embed.py) and the device (semantic-find.index.js) both read
THIS table, so deploy-side and device-side embeddings agree by
construction."""

import hashlib
import json
import struct
import sys
import urllib.request
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
DEST = REPO / "features/muon/loop/compute/semantic-find/assets/find"
HF = "https://huggingface.co/minishlab/potion-base-8M/resolve/main/"


def fetch(name):
    print(f"fetching {name}…")
    data = urllib.request.urlopen(HF + name).read()
    return data


def main():
    DEST.mkdir(parents=True, exist_ok=True)
    st = fetch("model.safetensors")
    tk = fetch("tokenizer.json")

    hlen = struct.unpack("<Q", st[:8])[0]
    hdr = json.loads(st[8:8 + hlen])
    info = hdr["embeddings"]
    assert info["dtype"] == "F32", info
    rows, dims = info["shape"]
    off = 8 + hlen + info["data_offsets"][0]
    import array
    emb = array.array("f")
    emb.frombytes(st[off:off + rows * dims * 4])

    tok = json.loads(tk)
    assert tok["model"]["type"] == "WordPiece"
    vocab = [None] * rows
    for t, i in tok["model"]["vocab"].items():
        vocab[i] = t

    # int8 per-row quantisation: scale = max|row| / 127
    table = bytearray(rows * dims)
    scales = array.array("f", [0.0] * rows)
    for r in range(rows):
        base = r * dims
        m = max(abs(v) for v in emb[base:base + dims]) or 1.0
        s = m / 127.0
        scales[r] = s
        for d in range(dims):
            q = int(round(emb[base + d] / s))
            table[base + d] = (q + 256) % 256  # two's complement byte

    (DEST / "vocab.json").write_text(json.dumps(vocab))
    (DEST / "table.bin").write_bytes(bytes(table))
    (DEST / "scales.bin").write_bytes(scales.tobytes())
    (DEST / "meta.json").write_text(json.dumps(
        {"rows": rows, "dims": dims, "unk": tok["model"]["unk_token"],
         "prefix": tok["model"]["continuing_subword_prefix"]}))
    (DEST / "PINNED").write_text(
        "minishlab/potion-base-8M\n"
        f"model.safetensors sha256:{hashlib.sha256(st).hexdigest()}\n"
        f"tokenizer.json sha256:{hashlib.sha256(tk).hexdigest()}\n"
        f"rows {rows} dims {dims} int8 per-row scales\n")
    print(f"wrote {DEST.relative_to(REPO)}: vocab.json, table.bin "
          f"({rows}x{dims} int8), scales.bin, meta.json, PINNED")


if __name__ == "__main__":
    sys.exit(main())
