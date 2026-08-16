#!/usr/bin/env python3
"""fm linker v0 — composes a product from the feature tree.

Pipeline:
  1. walk the product tree (symlinks into features/ or local overrides),
     include/exclude from each node's order.md checklist, then linearise by
     PROVENANCE TIME: each node's position is the timestamp of the prompt its
     spec cites (tree = grouping + selection only; regrouping never rewires)
  2. parse each feature's .rs files: feature_X impls (functions, with full
     signatures) and plain structs (fields)
  3. chain functions by (name, all parameter types) — full-signature keying,
     i.e. multiple dispatch; rewrite existing.fn() to the previous definition
     in the enclosing function's chain
  4. flat-merge same-named structs; duplicate field = link error
  5. emit dispatchers: a plain delegate for unique names, a generated trait +
     generic fn for overloaded names (rustc's type system does the dispatch),
     and std::ops operator glue for names like add/sub/mul (`col + col`)
  6. emit a cargo project under products/<name>/build/ and run cargo build,
     mapping rustc diagnostics back to feature-source file:line

Usage: fmlink.py [product] [--run]      (product defaults to "demo")
"""

import argparse
import datetime
import json
import re
import shutil
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
SKIP_DIRS = {"build", "target", "assets"}

# ---- asset fragments: implementation files in page languages -----------------
# a feature may carry css/js/html fragment files beside its spec; the linker
# assembles them into the page-owning feature's asset at slot markers, in
# linearisation order, each wrapped in a provenance comment. filename infix
# names the target page (bare = index); 'page' = every html page.
FRAGMENT_PAGE = {"index": ["index.html"], "login": ["login.html"],
                 "install": ["install.html"], "sw": ["sw.js"],
                 "page": ["index.html", "login.html", "install.html"]}
EXT_SLOT = {"css": "style", "js": "script", "html": "body"}
# pages whose script/style fragments are emitted as per-feature FILES under
# site/f/ (referenced at the slot marker) instead of inlined — so a release
# invalidates only the fragments it touched (minimal updates, fm-spec #p6).
# Classic scripts and stylesheets apply in document order: composition
# semantics are identical to inlining.
SPLIT_PAGES = {"index.html"}
SLOT_MARKER = {"head": "<!-- fm:head -->", "style": "/* fm:style */",
               "body": "<!-- fm:body -->", "script": "// fm:script"}
SLOT_COMMENT = {"head": "<!-- fm: {} -->", "style": "/* fm: {} */",
                "body": "<!-- fm: {} -->", "script": "// fm: {}"}

# fn names that additionally get std::ops glue, with required arity
OP_TRAITS = {"add": ("Add", 2), "sub": ("Sub", 2), "mul": ("Mul", 2),
             "div": ("Div", 2), "rem": ("Rem", 2), "neg": ("Neg", 1)}

ARG_LETTERS = "abcdefgh"


def fail(msg: str):
    sys.exit(f"fm link error: {msg}")


# ------------------------------------------------------------------ tree walk

def read_places(product_dir: Path):
    """Parse places.md -> [{name, kind, entry}], or None if absent (legacy:
    single native place with entry main). Placement lives in the product:
    the same placeless feature code is built once per place."""
    f = product_dir / "places.md"
    if not f.exists():
        return None
    places = []
    for line in f.read_text().splitlines():
        m = re.match(r"-\s*\[( |x)\]\s*(\w+)\s*:\s*(native|wasm)\s*,\s*entry\s*=\s*(\w+)"
                     r"(?:\s*,\s*event\s*=\s*(\w+))?",
                     line.strip())
        if m and m.group(1) == "x":
            places.append({"name": m.group(2), "kind": m.group(3),
                           "entry": m.group(4), "event": m.group(5)})
    if not places:
        fail(f"{f.relative_to(REPO)} defines no included places")
    return places


def read_order(directory: Path):
    """Parse order.md checklist -> list of (name, included), or None if absent."""
    f = directory / "order.md"
    if not f.exists():
        return None
    entries = []
    for line in f.read_text().splitlines():
        m = re.match(r"-\s*\[( |x)\]\s*(\S+)", line.strip())
        if m:
            entries.append((m.group(2), m.group(1) == "x"))
    return entries


def linearise(directory: Path, out: list, excluded: list, root: Path):
    """DFS pre-order over feature dirs (symlinks followed transparently);
    sibling order/inclusion from order.md."""
    subs = {p.name for p in directory.iterdir()
            if p.is_dir() and p.name not in SKIP_DIRS}
    order = read_order(directory)
    rel = directory.relative_to(root)
    if order is None:
        if subs:
            fail(f"{rel} has subfeatures but no order.md")
        return
    listed = {name for name, _ in order}
    for extra in sorted(subs - listed):
        fail(f"{rel}/order.md does not list subfeature folder '{extra}'")
    for name, included in order:
        if not included:
            # unticked: excluded — the folder need not exist locally, which is
            # how a product override subtracts a shared subfeature
            excluded.append(str(rel / name))
            continue
        if name not in subs:
            fail(f"{rel}/order.md includes '{name}' but the folder does not exist")
        out.append(directory / name)
        linearise(directory / name, out, excluded, root)


# ------------------------------------------------------------- chronology
# proposal 9 (notes.md): composition order is provenance order. Each node's
# spec cites the prompt that caused it; the prompt's timestamp in transcripts/
# is the node's position. The tree carries grouping and selection only —
# regrouping can never rewire behaviour. "Newest is outermost", globally.

CITE_RE = re.compile(r"transcripts/([\w.-]+\.md)#p(\d+)([a-z]?)")

# a field ask is a human prompt too, and a better provenance record than a
# chat message quoting one: it carries its own OK and its own stable id (the
# millisecond it was filed). Asks reach the builder through the ask store,
# not the session log, so they cite `asks#<t>` — the id IS the timestamp, so
# the node's position needs no lookup. (notes.md, the flywheel's provenance
# ruling; first used by shell/logo/dots/aligned-grid.)
ASK_CITE_RE = re.compile(r"\basks#(\d{13})\b")


def read_anchor_times() -> dict:
    """(transcript filename, prompt number, rider) -> 'YYYY-MM-DD HH:MM'."""
    times = {}
    for t in sorted((REPO / "transcripts").glob("*.md")):
        for m in re.finditer(r"^### p(\d+)([a-z]?)\n\*([0-9: -]+)\*",
                             t.read_text(), re.M):
            times[(t.name, int(m.group(1)), m.group(2))] = m.group(3)
    return times


def node_key(directory: Path, times: dict):
    """(timestamp, transcript, prompt number, rider) from the first provenance
    citation in the node's spec; None if the spec cites nothing."""
    real = directory.resolve()
    spec = real / f"{real.name}.md"
    if not spec.exists():
        return None
    text = spec.read_text()
    m = CITE_RE.search(text)
    ask = ASK_CITE_RE.search(text)
    # whichever provenance the spec cites FIRST is the node's position
    if ask and (not m or ask.start() < m.start()):
        ms = int(ask.group(1))
        stamp = datetime.datetime.fromtimestamp(ms / 1000)
        if not 2020 <= stamp.year <= 2100:
            fail(f"{real.relative_to(REPO)}: asks#{ms} is not a plausible "
                 f"filing time ({stamp.year})")
        return (stamp.strftime("%Y-%m-%d %H:%M"), "asks", ms, "")
    if not m:
        return None
    key = (m.group(1), int(m.group(2)), m.group(3))
    if key not in times:
        fail(f"{real.relative_to(REPO)}: spec cites {m.group(1)}#p{m.group(2)}"
             f"{m.group(3)} but no such anchor exists in transcripts/")
    return (times[key],) + key


def contributes(directory: Path) -> bool:
    """Does this node add composition material (code, fragments, assets,
    deps)? Pure grouping nodes don't, and are ordered by their subtree."""
    real = directory.resolve()
    if (real / "assets").is_dir() or (real / "deps.toml").exists():
        return True
    return any(f.is_file() and (f.suffix == ".rs" or f.suffix[1:] in EXT_SLOT)
               for f in real.iterdir())


def chronologise(feature_dirs: list, root: Path) -> list:
    """Sort the included nodes by provenance time. Ties (one prompt, several
    nodes) resolve by containment (parent first) then path. A code-free
    grouping node takes the earliest key in its subtree, so a late regroup
    never displaces old children. A contributing node must cite an anchor;
    a child citing an earlier prompt than its parent is a link error."""
    times = read_anchor_times()
    own = {}
    for d in feature_dirs:
        own[d] = node_key(d, times)
        if own[d] is None and contributes(d):
            fail(f"{d.resolve().relative_to(REPO)}: contributes code but its "
                 f"spec cites no transcript anchor — chronological "
                 f"linearisation needs provenance")
    key = dict(own)
    for d in sorted(feature_dirs, key=lambda p: -len(p.parts)):  # deepest first
        if contributes(d) and own[d]:
            continue
        child_keys = [key[c] for c in feature_dirs
                      if c.parent == d and key[c] is not None]
        cands = [k for k in [own[d]] + child_keys if k is not None]
        if not cands:
            fail(f"{d.resolve().relative_to(REPO)}: no provenance anywhere in "
                 f"its subtree")
        key[d] = min(cands)
    depth = {d: len(d.relative_to(root).parts) for d in feature_dirs}
    ordered = sorted(feature_dirs,
                     key=lambda d: (key[d], depth[d], str(d)))
    pos = {d: i for i, d in enumerate(ordered)}
    for d in feature_dirs:
        if d.parent in pos and pos[d.parent] > pos[d]:
            fail(f"{d.resolve().relative_to(REPO)} linearises before its own "
                 f"parent — child provenance predates the parent's; fix the "
                 f"citations")
    return ordered


# ------------------------------------------------------------------ rust parse

def match_brace(text: str, open_idx: int) -> int:
    depth = 0
    for i in range(open_idx, len(text)):
        if text[i] == "{":
            depth += 1
        elif text[i] == "}":
            depth -= 1
            if depth == 0:
                return i
    fail(f"unbalanced braces at char {open_idx}")


def line_of(text: str, idx: int) -> int:
    return text.count("\n", 0, idx) + 1


def parse_signature(header: str):
    """('name', [param types], return type or '') from 'fn name(a: T, b: U) -> R'."""
    m = re.search(r"fn\s+(\w+)\s*\(([^)]*)\)\s*(?:->\s*(.+?))?\s*$", header, re.S)
    if not m:
        fail(f"cannot parse fn signature: {header.strip()!r}")
    params = [p.split(":", 1)[1].strip()
              for p in m.group(2).split(",") if ":" in p]
    return m.group(1), params, (m.group(3) or "").strip()


class FeatureCode:
    """Everything one feature contributes: functions and struct fields."""

    def __init__(self, feature_dir: Path):
        self.dir = feature_dir
        # resolve symlinks so diagnostics point at the real source location
        self.rel = str(feature_dir.resolve().relative_to(REPO))
        self.name = None          # e.g. "Hello" from struct feature_Hello
        self.fns = []             # dicts: name, params, ret, src, first, lines
        self.structs = []         # (struct_name, [(field, type, src_file, line)])
        assets = feature_dir / "assets"
        # (file, path relative to assets/) pairs — subdirectories are preserved
        # into site/, so a node may own a whole asset tree (e.g. STT models)
        self.assets = sorted(((p, p.relative_to(assets))
                              for p in assets.rglob("*") if p.is_file()),
                             key=lambda t: str(t[1])) if assets.is_dir() else []
        self.deps = {}            # cargo deps: name -> full spec line (verbatim)
        deps_file = feature_dir / "deps.toml"
        if deps_file.exists():
            for line in deps_file.read_text().splitlines():
                m = re.match(r'([\w-]+)\s*=\s*(.+)$', line.strip())
                if m:
                    self.deps[m.group(1)] = f"{m.group(1)} = {m.group(2)}"
        self.fragments = []       # page fragments: {file, slot, text, src}
        for frag in sorted(feature_dir.glob("*.*")):
            ext = frag.suffix[1:]
            if not frag.is_file() or ext not in EXT_SLOT:
                continue
            slot = EXT_SLOT[ext]
            target = "index"
            for mid in frag.name.split(".")[1:-1]:
                if mid == "head" and ext == "html":
                    slot = "head"
                elif mid in FRAGMENT_PAGE:
                    target = mid
                else:
                    fail(f"{self.rel}/{frag.name}: unknown fragment target '{mid}'")
            # 'page' fragments decorate whichever pages this composition has;
            # an explicitly named page is a hard requirement
            for page in FRAGMENT_PAGE[target]:
                self.fragments.append({"file": page, "slot": slot,
                                       "text": frag.read_text(),
                                       "name": frag.name,
                                       "required": target != "page",
                                       "src": self.rel.replace("features/", "", 1)})
        # verbatim library files: full Rust (generics, traits, helpers) the
        # composition machinery doesn't touch — emitted as-is, per node
        self.libs = []            # (src_rel, text)
        for lib in sorted(feature_dir.glob("*.lib.rs")):
            self.libs.append((str(lib.resolve().relative_to(REPO)),
                              lib.read_text()))
        for rs in sorted(feature_dir.glob("*.rs")):
            if rs.name.endswith(".lib.rs"):
                continue
            self._parse(rs)

    def _parse(self, rs: Path):
        text = rs.read_text()
        src = str(rs.resolve().relative_to(REPO))

        m = re.search(r"struct\s+feature_(\w+)\s*;", text)
        if m:
            self.name = m.group(1)

        for im in re.finditer(r"impl\s+feature_(\w+)", text):
            self.name = im.group(1)
            body_open = text.index("{", im.end())
            body_close = match_brace(text, body_open)
            self._parse_fns(text, src, body_open + 1, body_close)

        for sm in re.finditer(r"(?:pub\s+)?struct\s+(\w+)\s*\{", text):
            if sm.group(1).startswith("feature_"):
                continue
            close = match_brace(text, sm.end() - 1)
            fields = []
            for fm in re.finditer(r"(?:pub\s+)?(\w+)\s*:\s*([^,\n}]+)",
                                  text[sm.end():close]):
                fields.append((fm.group(1), fm.group(2).strip(), src,
                               line_of(text, sm.end() + fm.start())))
            self.structs.append((sm.group(1), fields))

    def _parse_fns(self, text: str, src: str, start: int, end: int):
        lines = text.splitlines()
        pos = start
        while True:
            fm = re.search(r"(?:pub\s+)?fn\s+(\w+)", text[pos:end])
            if not fm:
                break
            fn_start = pos + fm.start()
            body_open = text.index("{", pos + fm.end())
            body_close = match_brace(text, body_open)
            name, params, ret = parse_signature(text[fn_start:body_open])
            first, last = line_of(text, fn_start), line_of(text, body_close)
            self.fns.append({"name": name, "params": params, "ret": ret,
                             "src": src, "first": first,
                             "lines": lines[first - 1:last]})
            pos = body_close + 1


# ------------------------------------------------------------------ compose

class Emitter:
    """Accumulates generated lines with a per-line map back to feature source."""

    def __init__(self):
        self.lines = []
        self.map = []

    def emit(self, text: str, src=None, line=None):
        self.lines.append(text)
        self.map.append((src, line) if src else None)


def sig_str(name: str, params: list) -> str:
    return f"{name}({', '.join(params)})"


def merge_structs(features: list, out: Emitter):
    merged = {}
    for feature in features:
        for sname, fields in feature.structs:
            existing_fields = {f[0]: f for f in merged.setdefault(sname, [])}
            for field in fields:
                if field[0] in existing_fields:
                    prev = existing_fields[field[0]]
                    fail(f"struct '{sname}' field '{field[0]}' defined twice: "
                         f"{prev[2]}:{prev[3]} and {field[2]}:{field[3]}")
                merged[sname].append(field)
    for sname, fields in merged.items():
        out.emit("#[derive(Default, Debug, Clone)]")
        out.emit(f"pub struct {sname} {{")
        for fname, ftype, src, line in fields:
            out.emit(f"    pub {fname}: {ftype},", src, line)
        out.emit("}")
        out.emit("")


def rewrite_existing(text: str, fn: dict, key: tuple, heads: dict, feature) -> str:
    """Rewrite existing.fn( -> feature_Prev::fn(. `existing` may only refer to
    the enclosing function's own chain (name + signature)."""
    def sub(m):
        called = m.group(1)
        if called != fn["name"]:
            fail(f"{feature.rel}: existing.{called}() inside fn {fn['name']} — "
                 f"`existing` may only call the enclosing function's own chain")
        if key not in heads:
            fail(f"{feature.rel}: existing.{called}() but no earlier feature "
                 f"defines {sig_str(fn['name'], fn['params'])}")
        return f"feature_{heads[key]}::{called}("
    return re.sub(r"existing\s*\.\s*(\w+)\s*\(", sub, text)


def node_path(rel: str) -> str:
    """A node's tree-global path: the shared-tree address a per-user map keys
    on. Strips the features/ root, or a product tree's products/<name>/ root
    (materialised override dirs)."""
    if rel.startswith("features/"):
        return rel[len("features/"):]
    m = re.match(r"products/[^/]+/(.*)$", rel)
    return m.group(1) if m else rel


def log_paths(text: str, path: str) -> str:
    """`fm_log(…)` -> `fm_log_at("<node path>", …)`. The author writes what
    happened; the linker says who said it, so a log line can never claim the
    wrong feature and never drifts when a node is regrouped
    (features/miso/diag/logging)."""
    return re.sub(r"\bfm_log\s*\(", 'fm_log_at("' + path + '", ', text)


def tick_gate(fn, feature, heads, key, trusted):
    """The context manager's runtime gate line, or None. Chain-EXTENDING fns
    whose first parameter is `state: String` fall through to the previous
    link when the owning node's path crosses an explicit per-user untick —
    the same skip compose-time unticking performs, at runtime (see
    features/miso/loop/context-manager). Chain starters are the seams
    themselves and stay ungated; so do fns that don't carry loop state."""
    if key not in heads:
        return None
    header = " ".join(fn["lines"])
    m = re.search(r"fn\s+" + re.escape(fn["name"]) + r"\s*\(([^)]*)\)", header)
    if not m:
        return None
    names = [p.split(":", 1)[0].strip()
             for p in m.group(1).split(",") if ":" in p]
    if not names or names[0] != "state" or fn["params"][0] != "String":
        return None
    path = node_path(feature.rel)
    # the hook node's trusted base (trusted.md): subtrees that deliver the
    # ticks var itself stay ungated — gating them would freeze the map
    if any(path == t or path.startswith(t + "/") for t in trusted):
        return None
    args = ", ".join(names)
    return (f'        if fm_unticked(&state, "{path}") '
            f'{{ return feature_{heads[key]}::{fn["name"]}({args}); }}')


def compose_features(features: list, out: Emitter, gated: bool = False,
                     trusted: list = []) -> dict:
    """Emit feature impl blocks; return chains keyed by (name, param types)."""
    chains = {}   # key -> {"head": feature struct name, "params": [...], "ret": str}
    for feature in features:
        if not feature.fns:
            continue
        if not feature.name:
            fail(f"{feature.rel} defines functions but no feature_ struct")
        out.emit(f"// ---- feature: {feature.rel}")
        out.emit(f"struct feature_{feature.name};")
        out.emit(f"impl feature_{feature.name} {{")
        for fn in feature.fns:
            key = (fn["name"], tuple(fn["params"]))
            if key in chains and chains[key]["ret"] != fn["ret"]:
                fail(f"{sig_str(fn['name'], fn['params'])}: return type changed "
                     f"from '{chains[key]['ret']}' to '{fn['ret']}' in {feature.rel}"
                     f" — all links of a chain must agree")
            heads = {k: v["head"] for k, v in chains.items()}
            gate = tick_gate(fn, feature, heads, key, trusted) if gated else None
            fpath = node_path(feature.rel)
            for offset, text in enumerate(fn["lines"]):
                out.emit(log_paths(
                             rewrite_existing(text, fn, key, heads, feature),
                             fpath),
                         fn["src"], fn["first"] + offset)
                if gate and "{" in text:
                    out.emit(gate)
                    gate = None
        out.emit("}")
        out.emit("")
        for fn in feature.fns:
            key = (fn["name"], tuple(fn["params"]))
            members = chains[key]["members"] if key in chains else []
            chains[key] = {"head": feature.name, "params": fn["params"],
                           "ret": fn["ret"],
                           "members": members + [feature.rel]}
    return chains


def emit_dispatchers(chains: dict, out: Emitter):
    """Top-level callables: plain delegates for unique names, generated trait +
    generic dispatcher for overloaded names, std::ops glue for operator names."""
    by_name = {}
    for (name, ptypes), info in chains.items():
        by_name.setdefault(name, []).append(info)

    for name, entries in sorted(by_name.items()):
        if name == "main":
            continue
        arities = {len(e["params"]) for e in entries}
        if len(arities) > 1:
            fail(f"'{name}' is defined with different arities {sorted(arities)} — "
                 f"overloads of one name must take the same number of arguments")
        n = arities.pop()

        if len(entries) == 1:
            e = entries[0]
            sig = ", ".join(f"{l}: {t}" for l, t in zip(ARG_LETTERS, e["params"]))
            args = ", ".join(ARG_LETTERS[:n])
            ret = f" -> {e['ret']}" if e["ret"] else ""
            out.emit(f"fn {name}({sig}){ret} {{ feature_{e['head']}::{name}({args}) }}")
        else:
            emit_overload_trait(name, n, entries, out)

        for e in entries:
            emit_op_glue(name, n, e, out)
        out.emit("")


def emit_overload_trait(name: str, n: int, entries: list, out: Emitter):
    """Generated trait with one type param per argument slot after the first;
    rustc's type system picks the impl — multiple dispatch, zero linker inference."""
    tparams = [f"P{i}" for i in range(1, n)]
    gen = f"<{', '.join(tparams)}>" if tparams else ""
    callsig = "a: Self" + "".join(f", {l}: {p}"
                                  for l, p in zip(ARG_LETTERS[1:], tparams))
    out.emit("#[allow(non_camel_case_types)]")
    out.emit(f"trait fm_{name}{gen}: Sized {{ type Out; "
             f"fn call({callsig}) -> Self::Out; }}")
    args = ", ".join(ARG_LETTERS[:n])
    for e in entries:
        ptypes = e["params"]
        implgen = f"<{', '.join(ptypes[1:])}>" if n > 1 else ""
        argsig = ", ".join(f"{l}: {t}" for l, t in zip(ARG_LETTERS, ptypes))
        ret = e["ret"] or "()"
        out.emit(f"impl fm_{name}{implgen} for {ptypes[0]} {{ type Out = {ret}; "
                 f"fn call({argsig}) -> Self::Out {{ "
                 f"feature_{e['head']}::{name}({args}) }} }}")
    bound = f"A: fm_{name}" + (f"<{', '.join(tparams)}>" if tparams else "")
    generics = ", ".join([bound] + tparams)
    dsig = "a: A" + "".join(f", {l}: {p}"
                            for l, p in zip(ARG_LETTERS[1:], tparams))
    out.emit(f"fn {name}<{generics}>({dsig}) -> A::Out {{ A::call({args}) }}")


def emit_op_glue(name: str, n: int, e: dict, out: Emitter):
    """std::ops impls so `col + col` / `col + vec` work alongside add(col, col)."""
    if name not in OP_TRAITS or OP_TRAITS[name][1] != n or not e["ret"]:
        return
    trait = OP_TRAITS[name][0]
    ptypes, ret, head = e["params"], e["ret"], e["head"]
    if n == 2:
        out.emit(f"impl std::ops::{trait}<{ptypes[1]}> for {ptypes[0]} {{ "
                 f"type Output = {ret}; fn {name}(self, b: {ptypes[1]}) -> {ret} "
                 f"{{ feature_{head}::{name}(self, b) }} }}")
    else:
        out.emit(f"impl std::ops::{trait} for {ptypes[0]} {{ "
                 f"type Output = {ret}; fn {name}(self) -> {ret} "
                 f"{{ feature_{head}::{name}(self) }} }}")


def compose(features: list):
    """Compose the placeless body once; entry glue is appended per place."""
    out = Emitter()
    out.emit("// generated by fm linker v0 — do not edit; edit features/ instead")
    out.emit("#![allow(non_camel_case_types, dead_code, non_snake_case, unused)]")
    out.emit("")
    merge_structs(features, out)
    for feature in features:
        for src, text in feature.libs:
            out.emit(f"// ---- library: {src}")
            for offset, line in enumerate(text.splitlines()):
                out.emit(line, src, offset + 1)
            out.emit("")
    # the context manager's hook: its presence in a composed verbatim lib
    # switches on runtime tick gates; without it the output is unchanged.
    # the hook node may declare a trusted base (trusted.md) of subtrees
    # that stay ungated — the machinery that delivers the ticks var
    gated = False
    trusted = []
    for f in features:
        if any("fn fm_unticked" in text for _, text in f.libs):
            gated = True
            tm = f.dir / "trusted.md"
            if tm.exists():
                trusted += [m.group(1) for m in
                            re.finditer(r"^-\s*(\S+)", tm.read_text(), re.M)]
    chains = compose_features(features, out, gated, trusted)
    out.emit("// ---- dispatchers (plain delegate / generated-trait multiple dispatch)")
    emit_dispatchers(chains, out)
    return out, chains


def with_entry(base: Emitter, chains: dict, place: dict) -> Emitter:
    """Copy the composed body and append this place's entry glue."""
    out = Emitter()
    out.lines = list(base.lines)
    out.map = list(base.map)
    entry = place["entry"]
    key = (entry, ())
    if key not in chains:
        fail(f"place '{place['name']}': no feature defines {entry}() (zero-arg)")
    head = chains[key]["head"]
    if place["kind"] == "native":
        out.emit(f"// ---- entry point for place '{place['name']}'")
        out.emit(f"fn main() {{ feature_{head}::{entry}(); }}")
    else:  # wasm: export the entry chain's String result as a packed ptr/len
        if chains[key]["ret"] != "String":
            fail(f"place '{place['name']}': wasm entry {entry}() must return "
                 f"String (found '{chains[key]['ret'] or 'nothing'}')")
        out.emit(f"// ---- wasm exports for place '{place['name']}'")
        out.emit("fn fm_pack(s: String) -> u64 {")
        out.emit("    let b = s.into_bytes();")
        out.emit("    let packed = ((b.as_ptr() as u64) << 32) | (b.len() as u64);")
        out.emit("    std::mem::forget(b);")
        out.emit("    packed")
        out.emit("}")
        out.emit("#[no_mangle]")
        out.emit("pub extern \"C\" fn fm_alloc(len: u32) -> u32 {")
        out.emit("    let mut buf = vec![0u8; len as usize];")
        out.emit("    let ptr = buf.as_mut_ptr() as u32;")
        out.emit("    std::mem::forget(buf);")
        out.emit("    ptr")
        out.emit("}")
        out.emit("#[no_mangle]")
        out.emit("pub extern \"C\" fn fm_entry() -> u64 {")
        out.emit(f"    fm_pack(feature_{head}::{entry}())")
        out.emit("}")
        event = place.get("event")
        if event:
            ekey = (event, ("String",))
            if ekey not in chains or chains[ekey]["ret"] != "String":
                fail(f"place '{place['name']}': event chain {event}(String) -> "
                     f"String is not defined")
            ehead = chains[ekey]["head"]
            out.emit("#[no_mangle]")
            out.emit("pub extern \"C\" fn fm_event(ptr: u32, len: u32) -> u64 {")
            out.emit("    let input = unsafe {")
            out.emit("        String::from_raw_parts(ptr as *mut u8,")
            out.emit("                               len as usize, len as usize)")
            out.emit("    };")
            out.emit(f"    fm_pack(feature_{ehead}::{event}(input))")
            out.emit("}")
    return out


def print_chains(chains: dict, features: list):
    """Dump composition topology: each Rust chain, then each page slot's
    fragment order — both in linearisation order (innermost/first-injected
    first). Stable, sorted output — diff it before/after a tree reorganisation
    to prove the regroup did or didn't rewire behaviour. Fragment order is
    behaviour too: CSS cascade and script order follow it."""
    for (name, ptypes), info in sorted(chains.items()):
        ret = f" -> {info['ret']}" if info["ret"] else ""
        members = [m.replace("features/", "", 1) for m in info["members"]]
        print(f"{sig_str(name, list(ptypes))}{ret}:")
        print(f"  {' → '.join(members)}")
    slots = {}
    for feature in features:
        for fr in feature.fragments:
            slots.setdefault((fr["file"], fr["slot"]), []).append(fr["src"])
    for (page, slot), srcs in sorted(slots.items()):
        print(f"fragment {page} [{slot}]:")
        print(f"  {' → '.join(srcs)}")
    # lib/chain ratio: verbatim .lib.rs code sits outside the composition
    # machinery — a steadily climbing share means typed code is escaping the
    # chain model and the parser needs to grow
    chain_lines = sum(len(fn["lines"]) for f in features for fn in f.fns)
    lib_lines = sum(len(text.splitlines()) for f in features for _, text in f.libs)
    total = chain_lines + lib_lines
    if total:
        print(f"rust lines: {chain_lines} chain, {lib_lines} verbatim lib "
              f"({100 * lib_lines // total}% lib)")


# ------------------------------------------------------------------ build

CARGO_BIN = """[package]
name = "{name}"
version = "0.0.1"
edition = "2021"

[dependencies]
{deps}
[[bin]]
name = "{name}"
path = "src/main.rs"
"""

CARGO_WASM = """[package]
name = "{name}"
version = "0.0.1"
edition = "2021"

[dependencies]
{deps}
[lib]
crate-type = ["cdylib"]
path = "src/lib.rs"

[profile.release]
lto = true
opt-level = "z"
"""


def merged_deps(features: list) -> str:
    """Union of every feature's deps.toml; conflicting specs = link error."""
    merged = {}
    for feature in features:
        for name, spec in feature.deps.items():
            if name in merged and merged[name] != spec:
                fail(f"cargo dep '{name}' wanted as both '{merged[name]}' and "
                     f"'{spec}' — align the features' deps.toml files")
            merged[name] = spec
    return "".join(f"{spec}\n" for _, spec in sorted(merged.items()))


def cargo_build(crate_dir: Path, emitter: Emitter, wasm: bool, label: str):
    cmd = ["cargo", "build", "--release", "--message-format=json"]
    if wasm:
        cmd += ["--target", "wasm32-unknown-unknown"]
    result = subprocess.run(cmd, cwd=crate_dir, capture_output=True, text=True)
    report_diagnostics(result.stdout, emitter, label)
    if result.returncode != 0:
        sys.exit(f"build FAILED ({label})")


def build_legacy(product: str, emitter: Emitter, chains: dict, run: bool):
    """Old single-place layout: build/ is one native crate, entry = main."""
    emitter = with_entry(emitter, chains,
                         {"name": "main", "kind": "native", "entry": "main"})
    build_dir = REPO / "products" / product / "build"
    (build_dir / "src").mkdir(parents=True, exist_ok=True)
    (build_dir / "Cargo.toml").write_text(
        CARGO_BIN.format(name=product, deps=""))
    (build_dir / "src" / "main.rs").write_text("\n".join(emitter.lines) + "\n")
    print(f"emitted {build_dir.relative_to(REPO)}/src/main.rs "
          f"({len(emitter.lines)} lines)")
    cargo_build(build_dir, emitter, wasm=False, label=product)
    print("build OK")
    if run:
        run_binary(build_dir / "target" / "release" / product, build_dir)


def build_places(product: str, places: list, base: Emitter, chains: dict,
                 features: list, run: bool):
    """One crate per place from the same composed body; wasm artifacts and
    feature assets/ files are assembled into build/site/ for serving."""
    build_dir = REPO / "products" / product / "build"
    site = build_dir / "site"
    deps = merged_deps(features)
    native_binaries = []
    for place in places:
        emitter = with_entry(base, chains, place)
        crate = f"{product}_{place['name']}"
        crate_dir = build_dir / place["name"]
        (crate_dir / "src").mkdir(parents=True, exist_ok=True)
        wasm = place["kind"] == "wasm"
        toml = (CARGO_WASM if wasm else CARGO_BIN).format(name=crate, deps=deps)
        (crate_dir / "Cargo.toml").write_text(toml)
        src = crate_dir / "src" / ("lib.rs" if wasm else "main.rs")
        src.write_text("\n".join(emitter.lines) + "\n")
        print(f"emitted {src.relative_to(REPO)} ({len(emitter.lines)} lines) "
              f"[{place['kind']}]")
        cargo_build(crate_dir, emitter, wasm, label=place["name"])
        if wasm:
            artifact = (crate_dir / "target" / "wasm32-unknown-unknown"
                        / "release" / f"{crate}.wasm")
            site.mkdir(parents=True, exist_ok=True)
            (site / "client.wasm").write_bytes(artifact.read_bytes())
            print(f"  site/client.wasm ({artifact.stat().st_size} bytes)")
        else:
            native_binaries.append(crate_dir / "target" / "release" / crate)

    # feature assets land in site/, linearisation order (later overwrites earlier)
    asset_files = [a for f in features for a in f.assets]
    if asset_files:
        site.mkdir(parents=True, exist_ok=True)
        for a, rel in asset_files:
            dest = site / rel
            dest.parent.mkdir(parents=True, exist_ok=True)
            dest.write_bytes(a.read_bytes())
        top = [str(rel) for _, rel in asset_files if len(rel.parts) == 1]
        trees = sorted({rel.parts[0] + "/" for _, rel in asset_files
                        if len(rel.parts) > 1})
        print(f"  site/ assets: {', '.join(top + trees)}")
    # sweep assets a previous build copied whose owner is now excluded —
    # only files this linker itself placed are ever deleted (notes.md
    # hygiene #9: stale asset trees lingered after an untick)
    manifest = build_dir / "asset-manifest.json"
    current = {str(rel) for _, rel in asset_files}
    try:
        previous = set(json.loads(manifest.read_text()))
    except (OSError, ValueError):
        previous = set()
    for rel in sorted(previous - current):
        stale = site / rel
        if stale.exists():
            stale.unlink()
            print(f"  removed stale site/{rel} (asset owner not in this composition)")
    manifest.write_text(json.dumps(sorted(current)))
    remove_stale_pages(site, current)
    compose_assets(site, features)

    print("build OK")
    if run and native_binaries:
        run_binary(native_binaries[0], build_dir)


def remove_stale_pages(site: Path, copied: set):
    """Delete composition-target pages left over from a previous build whose
    owning feature is now excluded — their slot markers are already consumed
    and their presence isn't this composition's choice."""
    targets = {p for pages in FRAGMENT_PAGE.values() for p in pages}
    for page in sorted(targets - copied):
        if (site / page).exists():
            (site / page).unlink()
            print(f"  removed stale site/{page} (owner not in this composition)")


def compose_assets(site: Path, features: list):
    """Inject every included feature's page fragments at the slot markers of
    the page-owning assets, in linearisation order, provenance-commented.
    Toggling a feature off in order.md genuinely removes its fragments."""
    by_page = {}
    for feature in features:
        fpath = node_path(feature.rel)
        for fr in feature.fragments:
            # page-side logging gets its node path the same way chain code
            # does — from the linker, never from the author
            fr["text"] = log_paths(fr["text"], fpath)
            by_page.setdefault(fr["file"], []).append(fr)
    # f/ is wholly linker-owned: sweep it so unticked features leave no
    # stale fragment files behind
    shutil.rmtree(site / "f", ignore_errors=True)
    for page, items in sorted(by_page.items()):
        path = site / page
        if not path.exists():
            needed = [i for i in items if i["required"]]
            if not needed:
                by_page[page] = []   # only 'page'-target decorators — page absent
                continue
            fail(f"fragments target '{page}' but no such site asset exists "
                 f"(required by {needed[0]['src']})")
        text = path.read_text()
        for slot in ("head", "style", "body", "script"):
            slot_items = [i for i in items if i["slot"] == slot]
            marker = SLOT_MARKER[slot]
            if slot_items and marker not in text:
                fail(f"{page} has no '{marker}' slot (needed by {slot_items[0]['src']})")
            if marker in text:
                if page in SPLIT_PAGES and slot in ("script", "style") and slot_items:
                    # per-feature files: close the enclosing block, reference
                    # each fragment in composition order, reopen the block
                    fdir = site / "f"
                    fdir.mkdir(exist_ok=True)
                    tags = []
                    for i in slot_items:
                        (fdir / i["name"]).write_text(
                            SLOT_COMMENT[slot].format(i["src"]) + "\n"
                            + i["text"].rstrip() + "\n")
                        tags.append(
                            f'<script src="f/{i["name"]}"></script>'
                            if slot == "script"
                            else f'<link rel="stylesheet" href="f/{i["name"]}">')
                    close, reopen = (("</script>", "<script>") if slot == "script"
                                     else ("</style>", "<style>"))
                    text = text.replace(
                        marker, close + "\n" + "\n".join(tags) + "\n" + reopen)
                else:
                    blocks = "\n".join(
                        SLOT_COMMENT[slot].format(i["src"]) + "\n" + i["text"].rstrip()
                        for i in slot_items)
                    text = text.replace(marker, blocks)
        path.write_text(text)
    composed = {p: i for p, i in by_page.items() if i}
    if composed:
        print("  fragments composed: " + ", ".join(
            f"{p} ({len(i)})" for p, i in sorted(composed.items())))


def run_binary(binary: Path, cwd: Path):
    print(f"running {binary.relative_to(REPO)}:")
    run_result = subprocess.run([binary], capture_output=True, text=True, cwd=cwd)
    print(run_result.stdout, end="")
    if run_result.returncode != 0:
        sys.exit(f"run FAILED ({run_result.returncode}): {run_result.stderr}")


def translate(message: str) -> str:
    """Rewrite rustc phrasing that leaks generated machinery into fm terms."""
    m = re.match(r"the trait bound `(.+?): fm_(\w+)(?:<(.+?)>)?` is not satisfied",
                 message)
    if m:
        first, name, rest = m.group(1), m.group(2), m.group(3)
        params = first + (f", {rest}" if rest else "")
        return f"no definition of {name}({params}) in any linked feature"
    return message


def report_diagnostics(cargo_json: str, emitter: Emitter, label: str = ""):
    for raw in cargo_json.splitlines():
        try:
            msg = json.loads(raw)
        except json.JSONDecodeError:
            continue
        if msg.get("reason") != "compiler-message":
            continue
        m = msg["message"]
        if m.get("level") not in ("error", "warning"):
            continue
        where = "generated code"
        for span in m.get("spans", []):
            if span.get("is_primary"):
                gen_line = span["line_start"]
                mapped = (emitter.map[gen_line - 1]
                          if gen_line <= len(emitter.map) else None)
                where = (f"{mapped[0]}:{mapped[1]}" if mapped
                         else f"generated:{gen_line}")
                break
        tag = f"[{label}] " if label else ""
        print(f"  {tag}{m['level']}: {where}: {translate(m['message'])}")


# ------------------------------------------------------------------ main

def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("product", nargs="?", default="demo")
    ap.add_argument("--run", action="store_true", help="run the binary after building")
    ap.add_argument("--chains", action="store_true",
                    help="print chain topology and exit (no build)")
    args = ap.parse_args()

    product_dir = REPO / "products" / args.product
    if not product_dir.is_dir():
        fail(f"no such product: products/{args.product}")
    if not (product_dir / "order.md").exists():
        fail(f"products/{args.product} has no order.md — a product is a feature "
             f"tree (symlinks into features/, or local overrides) plus order.md")

    feature_dirs, excluded = [], []
    linearise(product_dir, feature_dirs, excluded, product_dir)
    # tree-global names (fm.md): a node's name must be unique across the
    # composed tree — implementation namespaces (structs, page consts) are
    # flat, and a name should not need its path to mean something
    seen_names = {}
    for d in feature_dirs:
        if d.name in seen_names:
            fail(f"node name '{d.name}' used by both {seen_names[d.name]} and "
                 f"{d.relative_to(product_dir)} — names are tree-global (fm.md)")
        seen_names[d.name] = d.relative_to(product_dir)
    feature_dirs = chronologise(feature_dirs, product_dir)
    print("linearisation (provenance order):",
          " → ".join(str(d.relative_to(product_dir)) for d in feature_dirs))
    for ex in excluded:
        print(f"  excluded (order.md unticked): {ex}")

    features = [FeatureCode(d) for d in feature_dirs]
    base, chains = compose(features)
    if args.chains:
        print_chains(chains, features)
        return
    places = read_places(product_dir)
    if places is None:
        build_legacy(args.product, base, chains, args.run)
    else:
        build_places(args.product, places, base, chains, features, args.run)


if __name__ == "__main__":
    main()
