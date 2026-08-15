#!/usr/bin/env python3
"""Embed the exported feature catalog for on-device semantic find.

Reads site/features/tree.json (written by export_features.py), embeds each
node's name + purpose + intro with muon's own potion table
(tools/potion_embed.py — the same table the device reads), and writes
site/features/vectors.json: {"dims": N, "vecs": {path: [floats…]}}.
Deploy runs this after the tree export; the device embeds only queries."""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import potion_embed  # noqa: E402

REPO = Path(__file__).resolve().parent.parent
SITE = REPO / "products/muon/build/site"


def main():
    tree = json.loads((SITE / "features/tree.json").read_text())
    vecs = {}

    def walk(nodes):
        for n in nodes:
            text = " ".join(x for x in
                            [n.get("name"), n.get("purpose"), n.get("intro")]
                            if x)
            vecs[n["path"]] = [round(x, 4) for x in potion_embed.embed(text)]
            walk(n.get("children", []))

    walk(tree)
    out = {"dims": 256, "vecs": vecs}
    (SITE / "features/vectors.json").write_text(json.dumps(out))
    print(f"embedded {len(vecs)} catalog entries -> site/features/vectors.json")


if __name__ == "__main__":
    sys.exit(main())
