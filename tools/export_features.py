#!/usr/bin/env python3
"""Static export of the feature browser into the muon site.

Renders every feature node with the explorer's own server-side renderer into
site/features/<path>/index.html (tree | spec + code | transcript, no client
JS), so the deployed site serves the exact feature tree that built it.
Run by deploy.sh after fmlink; served publicly at muon.nøøb.org/features/.
"""

import json
import re
import shutil
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
import explorer

OUT = explorer.REPO / "products" / "muon" / "build" / "site" / "features"


def relink(html: str) -> str:
    """/feature/<path>[#f|?q] links -> /features/<path>/[#f|?q] (static layout);
    the fm.md doc link points at the tree root, which renders it."""
    html = re.sub(r'href="/feature/([^"#?]+)([^"]*)"',
                  lambda m: f'href="/features/{m.group(1)}/{m.group(2)}"', html)
    return html.replace('href="/view/fm.md"', 'href="/features/"')


def all_paths(children, acc):
    for feature in children:
        acc.append(feature.path)
        all_paths(feature.children, acc)
    return acc


def purpose_of(feature) -> str:
    """The spec's one-line italic purpose (line 2 by convention)."""
    if not feature.spec.exists():
        return ""
    for line in feature.spec.read_text().splitlines()[:4]:
        m = re.match(r"^\*(.+)\*\s*$", line.strip())
        if m:
            return m.group(1)
    return ""


def tree_json(children) -> list:
    """The tree as data, for the in-app chooser (features/muon/shell/panel/
    noob-button/chooser): name, path, purpose, children — order.md order."""
    return [{
        "name": f.name,
        "path": f.path,
        "purpose": purpose_of(f),
        "children": tree_json(f.children),
    } for f in children]


def main():
    children = explorer.load_children(explorer.FEATURES)
    paths = all_paths(children, [])
    if OUT.exists():
        shutil.rmtree(OUT)
    OUT.mkdir(parents=True, exist_ok=True)
    (OUT / "tree.json").write_text(json.dumps(tree_json(children)))
    for path in paths:
        page = relink(explorer.render_feature_page(path, ""))
        page_dir = OUT / path
        page_dir.mkdir(parents=True, exist_ok=True)
        (page_dir / "index.html").write_text(page)
    # /features/ itself lands on fm.md — the founding document, as orientation
    (OUT / "index.html").write_text(relink(explorer.render_file_page("fm.md", "")))
    print(f"features exported: {len(paths)} nodes -> site/features/")


if __name__ == "__main__":
    main()
