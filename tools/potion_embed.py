#!/usr/bin/env python3
"""Embed text with muon's own potion table (see tools/fetch_find.py) —
the deploy-side twin of semantic-find.index.js's embedder. Both read the
same int8 table and implement the same BERT WordPiece walk, so catalog
vectors (embedded here at deploy) and query vectors (embedded on the
device) live in the same space by construction.

Usage: embed_texts([...]) -> list of L2-normalised 256-dim vectors.
CLI: potion_embed.py "some text" prints the vector (for parity tests)."""

import array
import json
import sys
import unicodedata
from pathlib import Path

FIND = (Path(__file__).resolve().parent.parent
        / "features/muon/loop/compute/semantic-find/assets/find")

_state = None


def _load():
    global _state
    if _state:
        return _state
    meta = json.loads((FIND / "meta.json").read_text())
    vocab = json.loads((FIND / "vocab.json").read_text())
    table = (FIND / "table.bin").read_bytes()
    scales = array.array("f")
    scales.frombytes((FIND / "scales.bin").read_bytes())
    _state = (meta, {t: i for i, t in enumerate(vocab)}, table, scales)
    return _state


def _basic_tokens(text):
    """BERT basic tokenisation: lowercase, strip accents/control, split on
    whitespace and punctuation (punctuation is its own token)."""
    out, word = [], []
    text = unicodedata.normalize("NFD", text.lower())
    for ch in text:
        cat = unicodedata.category(ch)
        if cat == "Mn" or cat in ("Cc", "Cf"):
            continue
        if ch.isspace():
            if word:
                out.append("".join(word))
                word = []
        elif cat.startswith("P") or (cat.startswith("S") and not ch.isalnum()):
            if word:
                out.append("".join(word))
                word = []
            out.append(ch)
        else:
            word.append(ch)
    if word:
        out.append("".join(word))
    return out


def _wordpiece(word, vocab_idx, prefix, max_chars=100):
    if len(word) > max_chars:
        return None
    pieces, start = [], 0
    while start < len(word):
        end = len(word)
        cur = None
        while start < end:
            sub = word[start:end]
            if start > 0:
                sub = prefix + sub
            if sub in vocab_idx:
                cur = sub
                break
            end -= 1
        if cur is None:
            return None  # whole word becomes [UNK]
        pieces.append(cur)
        start = end
    return pieces


def tokenize(text):
    meta, vocab_idx, _, _ = _load()
    ids = []
    for word in _basic_tokens(text):
        pieces = _wordpiece(word, vocab_idx, meta["prefix"])
        if pieces is None:
            u = vocab_idx.get(meta["unk"])
            if u is not None:
                ids.append(u)
        else:
            ids.extend(vocab_idx[p] for p in pieces)
    return ids


def embed(text):
    meta, _, table, scales = _load()
    dims = meta["dims"]
    ids = tokenize(text)
    v = [0.0] * dims
    if not ids:
        return v
    for i in ids:
        base = i * dims
        s = scales[i]
        for d in range(dims):
            b = table[base + d]
            v[d] += (b - 256 if b >= 128 else b) * s
    n = len(ids)
    v = [x / n for x in v]
    norm = sum(x * x for x in v) ** 0.5 or 1.0
    return [x / norm for x in v]


def embed_texts(texts):
    return [embed(t) for t in texts]


if __name__ == "__main__":
    print(json.dumps(embed(" ".join(sys.argv[1:]))))
