#!/usr/bin/env python3
"""Static export of the feature browser into the miso site.

Renders every feature node with the explorer's own server-side renderer into
site/features/<path>/index.html (tree | spec + code | transcript, no client
JS), so the deployed site serves the exact feature tree that built it.
Run by deploy.sh after fmlink; served publicly at miso.nøøb.org/features/.
"""

import json
import re
import shutil
import subprocess
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
import explorer

OUT = explorer.REPO / "products" / "miso" / "build" / "site" / "features"


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


def build_numbers() -> dict:
    """commit hash -> build number (deploy's convention: build = commit count)."""
    order = subprocess.run(("git", "rev-list", "--reverse", "HEAD"),
                           cwd=explorer.REPO, capture_output=True,
                           text=True).stdout.split()
    return {h: i + 1 for i, h in enumerate(order)}


DIARY_PREFIXES = ("notes:", "handover:", "idea:", "ideas:", "format:")


def latest_build(feature, build) -> int:
    """The most-recent release that touched this node's OWN files (#p82) —
    its spec, code and assets, excluding child-node subdirectories (children
    carry their own numbers) and excluding order.md (#p41: gaining a child
    edits the parent's order.md but changes nothing about the parent —
    grouping bookkeeping must not bump builds, or every new leaf drags its
    ancestor into the awaiting-update list). Diary-class commits — subjects
    prefixed notes:/handover:/idea:/format: — are bookkeeping by declaration
    and skipped for the same reason (a format pass across every spec must
    not age the whole tree; 2026-08-21-hybrid #p17). The chooser and the
    release list speak the same numbers; a feature's number moves forward
    as it evolves."""
    own = [str(p.relative_to(explorer.REPO))
           for p in feature.dir.iterdir()
           if p.is_file() and p.name != "order.md"]
    assets = feature.dir / "assets"
    if assets.is_dir():
        own.append(str(assets.relative_to(explorer.REPO)))
    if not own:
        return 0
    log = subprocess.run(("git", "log", "--format=%H%x09%s", "--") + tuple(own),
                         cwd=explorer.REPO, capture_output=True,
                         text=True).stdout
    for line in log.splitlines():
        h, _, subject = line.partition("\t")
        if not subject.lstrip().lower().startswith(DIARY_PREFIXES):
            return build.get(h, 0)
    return 0


def intro_of(feature) -> str:
    """The '## user' paragraph — the chooser's show-me-more teaser is read by
    exactly the person that section is written for (#p73). Falls back to the
    spec's first paragraph for nodes without one."""
    if not feature.spec.exists():
        return ""
    text = feature.spec.read_text()
    for heading in ("user", "spec"):
        m = re.search(r"^## " + heading + r"\s*\n+(.+?)(?:\n\s*\n|\Z)",
                      text, re.M | re.S)
        if m:
            return " ".join(m.group(1).split())[:400]
    return ""


def purpose_of(feature) -> str:
    """The spec's one-line italic purpose (line 2 by convention)."""
    if not feature.spec.exists():
        return ""
    for line in feature.spec.read_text().splitlines()[:4]:
        m = re.match(r"^\*(.+)\*\s*$", line.strip())
        if m:
            return m.group(1)
    return ""


def tool_of(feature) -> str:
    """The toolbar tool this node registers, if any: a tools_list extension
    pushing {"id": "X", ...} in one of the node's .rs files. Ground truth for
    feature->tool (the #p54 mapping): the ask surface uses it to turn found
    features into open-the-tool buttons."""
    import re as _re
    for rs in sorted(feature.dir.glob("*.rs")):
        text = rs.read_text()
        if "fn tools_list" not in text:
            continue
        m = _re.search(r'"id"\s*:\s*"([^"]+)"', text)
        if m:
            return m.group(1)
    return ""


def subtools_of(feature) -> list:
    """The toolbar controls this node registers, if any: a tool_controls
    extension appending buttons whose data-ev is not a tool_ open event.
    Ground truth for control->feature (the sub-tool twin of the #p54
    mapping): long-press resolves a held control button to the node whose
    documentation describes it. Formatted ids (containing {}) are dynamic
    per-item buttons, not controls, and are skipped."""
    import re as _re
    evs = []
    for rs in sorted(feature.dir.glob("*.rs")):
        text = rs.read_text()
        if "fn tool_controls" not in text:
            continue
        for m in _re.finditer(r'data-ev=\\"([^"\\]+)\\"', text):
            ev = m.group(1)
            if ev.startswith("tool_") or "{" in ev:
                continue
            if ev not in evs:
                evs.append(ev)
    return evs


def tree_json(children, times, builds) -> list:
    """The tree as data, for the in-app chooser (features/miso/shell/panel/
    noob-button/chooser): name, path, purpose, intro, provenance timestamp,
    children — order.md order. ts uses fmlink's provenance rule: a node's
    time is its cited prompt's; a citation-less grouping node inherits its
    earliest child's."""
    import fmlink
    out = []
    for f in children:
        kids = tree_json(f.children, times, builds)
        key = fmlink.node_key(f.dir, times)
        ts = key[0] if key else min((k["ts"] for k in kids if k["ts"]), default="")
        node = {
            "name": f.name,
            "path": f.path,
            "purpose": purpose_of(f),
            "intro": intro_of(f),
            "ts": ts,
            "build": latest_build(f, builds),
            "children": kids,
        }
        tool = tool_of(f)
        if tool:
            node["tool"] = tool
        subtools = subtools_of(f)
        if subtools:
            node["subtools"] = subtools
        out.append(node)
    return out


def main():
    # deploy calls this on every ship; when no feature source changed since
    # the last bake, the bake is identical (per-feature builds move only when
    # a feature's own files do) — skip the ~4.5s. --force overrides.
    stamp = OUT / "stamp"
    if stamp.exists() and "--force" not in sys.argv:
        watched = [explorer.FEATURES, explorer.REPO / "transcripts"]
        newest = max((f.stat().st_mtime for root in watched
                      for f in root.rglob("*") if f.is_file()), default=0.0)
        if stamp.stat().st_mtime > newest:
            print("features export: sources unchanged since last bake — skipped")
            return
    children = explorer.load_children(explorer.FEATURES)
    paths = all_paths(children, [])
    if OUT.exists():
        shutil.rmtree(OUT)
    OUT.mkdir(parents=True, exist_ok=True)
    import fmlink
    (OUT / "tree.json").write_text(
        json.dumps(tree_json(children, fmlink.read_anchor_times(),
                             build_numbers())))
    for path in paths:
        page = relink(explorer.render_feature_page(path, ""))
        page_dir = OUT / path
        page_dir.mkdir(parents=True, exist_ok=True)
        (page_dir / "index.html").write_text(page)
    # /features/ itself lands on fm.md — the founding document, as orientation
    (OUT / "index.html").write_text(relink(explorer.render_file_page("fm.md", "")))
    # the bake's content fingerprint: held catalogs revalidate against this
    # (serve/features/auto-export), so it moves only when the words do
    import hashlib
    (OUT / "stamp").write_text(
        hashlib.sha256((OUT / "tree.json").read_bytes()).hexdigest()[:16])
    print(f"features exported: {len(paths)} nodes -> site/features/")


if __name__ == "__main__":
    main()
