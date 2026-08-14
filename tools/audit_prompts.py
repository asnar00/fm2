#!/usr/bin/env python3
"""Reverse index: transcript prompt -> feature node(s).

Feature nodes cite their originating prompt as `transcripts/<file>.md#pN`
(the provenance blockquote). Inverting those citations over the whole tree
gives, for every transcript prompt, the nodes it produced — and therefore:

  --gaps      prompts no node cites (missed, coalesced, or conversational)
  --map       every prompt with its citing nodes
  --coalesced prompts cited by several nodes, and nodes citing several prompts
  --orphans   feature .md files that cite no prompt at all (reverse gap)

Superseded prompts (edited-and-resent retries) are excluded from gap counts.
notes.md citations are shown as `(noted)` — recorded, but not a feature node.
"""
import argparse
import re
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
CITE_RE = re.compile(r"transcripts/([\w.-]+\.md)#(p\d+[a-z]?)\b")
ANCHOR_RE = re.compile(r"^### (p\d+[a-z]?)$")


SESSION_RE = re.compile(r"\*session `([0-9a-f-]+)`")


def parse_transcripts():
    """-> ({(file, anchor): {"text", "superseded"}}, {file: canonical file}).

    Transcript files are point-in-time snapshots of a session; a later export
    of the same session supersets an earlier one with identical anchors. Files
    are grouped by the session id in their header and the fullest snapshot is
    canonical — citations of any snapshot count toward the same anchor."""
    per_file, session_of = {}, {}
    for path in sorted((REPO_ROOT / "transcripts").glob("*.md")):
        text = path.read_text()
        m = SESSION_RE.search(text)
        session_of[path.name] = m.group(1) if m else path.name
        anchor, state, prompts = None, None, {}
        for line in text.splitlines():
            m = ANCHOR_RE.match(line)
            if m:
                anchor = m.group(1)
                state = prompts[anchor] = {"text": "", "superseded": False}
                continue
            if not anchor:
                continue
            if "superseded by the next prompt" in line:
                state["superseded"] = True
            elif line.startswith("> ") and not state["text"]:
                state["text"] = line[2:].strip()
        per_file[path.name] = prompts

    canon = {}  # session -> fullest snapshot
    for name, prompts in per_file.items():
        s = session_of[name]
        if s not in canon or len(prompts) > len(per_file[canon[s]]):
            canon[s] = name
    alias = {name: canon[session_of[name]] for name in per_file}
    merged = {(name, a): p for name in canon.values() for a, p in per_file[name].items()
              if not p["text"].startswith("<task-notification")}
    return merged, alias


def parse_citations(alias):
    """-> {(canonical file, anchor): [citing paths]}; nodes by dir, notes.md as '(noted)'."""
    cites = {}
    sources = sorted((REPO_ROOT / "features").rglob("*.md")) + [REPO_ROOT / "notes.md"]
    for path in sources:
        rel = path.relative_to(REPO_ROOT)
        label = "(noted)" if rel.name == "notes.md" else str(rel.parent)
        for m in CITE_RE.finditer(path.read_text()):
            f = alias.get(m.group(1), m.group(1))
            cites.setdefault((f, m.group(2)), []).append(label)
    return cites


def node_cites(paths):
    return [p for p in paths if p != "(noted)"]


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--map", action="store_true", help="full prompt -> nodes listing")
    ap.add_argument("--coalesced", action="store_true", help="many-to-one views both ways")
    ap.add_argument("--orphans", action="store_true", help="feature .md files citing no prompt")
    args = ap.parse_args()

    prompts, alias = parse_transcripts()
    cites = parse_citations(alias)

    if args.map:
        for (f, a), p in prompts.items():
            mark = " [superseded]" if p["superseded"] else ""
            targets = ", ".join(cites.get((f, a), [])) or "—"
            print(f"{f}#{a}{mark}: {targets}")
            print(f"    > {p['text'][:100]}")
        return

    if args.coalesced:
        print("== prompts cited by more than one node ==")
        for (f, a), paths in cites.items():
            nodes = node_cites(paths)
            if len(nodes) > 1:
                print(f"{f}#{a} -> {len(nodes)} nodes: {', '.join(nodes)}")
        print("\n== nodes citing more than one prompt ==")
        by_node = {}
        for (f, a), paths in cites.items():
            for p in node_cites(paths):
                by_node.setdefault(p, []).append(f"{f}#{a}")
        for node, anchors in sorted(by_node.items()):
            if len(anchors) > 1:
                print(f"{node} <- {', '.join(anchors)}")
        return

    if args.orphans:
        for path in sorted((REPO_ROOT / "features").rglob("*.md")):
            if path.name != "order.md" and not CITE_RE.search(path.read_text()):
                print(path.relative_to(REPO_ROOT))
        return

    # default: the gap audit
    dangling = [(f, a) for (f, a) in cites if (f, a) not in prompts]
    uncited = [((f, a), p) for (f, a), p in prompts.items()
               if not p["superseded"] and not node_cites(cites.get((f, a), []))]
    n_live = sum(1 for p in prompts.values() if not p["superseded"])
    print(f"{n_live} live prompts across {len(set(f for f, _ in prompts))} transcript(s); "
          f"{n_live - len(uncited)} reached a feature node, {len(uncited)} did not:\n")
    for (f, a), p in uncited:
        noted = " (noted)" if "(noted)" in cites.get((f, a), []) else ""
        print(f"{f}#{a}{noted}: {p['text'][:100]}")
    if dangling:
        print("\n== citations pointing at anchors that do not exist ==")
        for f, a in dangling:
            print(f"{f}#{a} <- {', '.join(cites[(f, a)])}")


if __name__ == "__main__":
    main()
