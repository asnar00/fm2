#!/usr/bin/env python3
"""Static export of the feature browser into the muon site.

Renders every feature node with the explorer's own server-side renderer into
site/features/<path>/index.html (tree | spec + code | transcript, no client
JS), so the deployed site serves the exact feature tree that built it.
Run by deploy.sh after fmlink; served publicly at muon.nøøb.org/features/.
"""

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


def main():
    paths = all_paths(explorer.load_children(explorer.FEATURES), [])
    if OUT.exists():
        shutil.rmtree(OUT)
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
