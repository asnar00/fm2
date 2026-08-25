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

# ---- fragment gates: the page-side twin of the enabled gates ----------------
# A composed node carrying this token in a page fragment asks the linker to
# make every OTHER node's index fragments obey the tick map, the same way
# GATE_HOOK makes the Rust chains obey it: a fragment's chain links fall
# through to the function they replaced, its load-time furniture is marked
# with its owner, and its stylesheet can be switched off. The runtime that
# reads the map is the hook-bearing node's own fragment; everything here is
# the wiring beneath it. See features/miso/loop/context/changing/enabled/obey.
FRAGMENT_GATE_HOOK = "fm:fragment-gate"
# an assignment to a method of an object: `feature_Review.releases = …`. The
# JS chain link — the page's `existing.fn()`.
JS_PATCH_RE = re.compile(r"^[ \t]*(feature_\w+)\.(\w+)\s*=(?!=)", re.M)
JS_DEFINE_RE = re.compile(r"^[ \t]*(?:const|let|var)\s+(feature_\w+)\s*=", re.M)
# elements that never have children, so they never open a nesting level
HTML_VOID = {"area", "base", "br", "col", "embed", "hr", "img", "input",
             "link", "meta", "source", "track", "wbr"}

# ---- gate coverage: what a node's tickbox actually reaches -------------------
# Every composed node gets an `enabled` var and a tickbox in the chooser, but a
# node whose functions carry no loop state and whose fragments patch nothing has
# nothing for either gate to hold: unticking it changes nothing until the next
# link. The linker knows this at emission time, so it says so — here, as a table
# (--coverage) and as a per-node record the export stamps into the tree.
COVERAGE = {}


def coverage_note(node: str, kind: str, n: int):
    """Record what got emitted for one node, keyed by its tree-global path (a
    product-local override and the node it replaces are one node). Rust gates
    are counted per PLACE
    and merged by max: the same node composes into every place, so its gates
    are the same set seen twice, not twice as many."""
    c = COVERAGE.setdefault(node, {"rust": 0, "fragment": 0, "style": 0, "body": 0})
    c[kind] = max(c[kind], n) if kind == "rust" else c[kind] + n


def coverage_of(node: str) -> dict:
    return COVERAGE.get(node, {"rust": 0, "fragment": 0, "style": 0, "body": 0})


def coverage_total(node: str) -> int:
    return sum(coverage_of(node).values())


def coverage_table(features: list):
    """The report: one line per composed node, loudest case last."""
    print("gate coverage — what each node's tickbox reaches at runtime:")
    silent = []
    for f in features:
        node = node_path(f.rel)
        c = coverage_of(node)
        if not coverage_total(node):
            silent.append(node)
            continue
        parts = [f"{c['rust']} rust" if c["rust"] else "",
                 f"{c['fragment']} fragment" if c["fragment"] else "",
                 f"{c['style']} style" if c["style"] else "",
                 f"{c['body']} body" if c["body"] else ""]
        print(f"  {node:<52} {', '.join(p for p in parts if p)}")
    for node in silent:
        print(f"  {node:<52} NOTHING — this tickbox is compose-time only")
    print(f"  {len(features) - len(silent)} of {len(features)} nodes gate "
          f"something at runtime; {len(silent)} are compose-time only")

# ---- context slots: the sidecar declaration file ----------------------------
# a node may carry <name>.vars, one declaration per line:
#   name: Type = default (scope, merge, inherit)
# the linker collects them from every composed node and emits a `Context`
# struct whose fields are typed by the var family in the context node's
# verbatim library. scaffolding per the standing arrangement: the linker holds
# the mechanism, features/miso/loop/context owns the design and the types.
VAR_DECL_RE = re.compile(
    r"^(\w+)\s*:\s*(.+?)\s*=\s*(.+?)\s*"
    r"\(\s*([\w-]+)\s*,\s*([\w-]+)\s*,\s*([\w-]+)\s*\)"
    # optional fourth column: the legacy state key this var is republished at,
    # so a page fragment that reads `s.<key>` keeps working after the value has
    # moved into the Context. See features/miso/loop/context/changing/converge/payload.
    r"(?:\s+js:([A-Za-z_][A-Za-z0-9_]*))?\s*$")
VAR_SCOPE = {"global": "ScopeGlobal", "group": "ScopeGroup",
              "user": "ScopeUser", "device": "ScopeDevice"}
VAR_MERGE = {"last-write": "MergeLastWrite", "crdt-sum": "MergeCrdtSum",
              "better": "MergeBetter", "none": "MergeNone",
              "counter": "MergeCounter"}
VAR_INHERIT = {"inherit": "Inherit", "own": "Own"}
# scopes the runtime cannot yet honour. A context is held per user (ladder rung
# 5); a global or group scoped var would be stored per user and quietly behave
# as if it were user-scoped, which is the kind of silent lie the typed
# declaration exists to prevent. Refused with the rung that earns them back.
# (rung 6 syncs a user's own vars across their instances; what global and group
# still await is the OVERLAY chain — a value living above the user and falling
# through to them — which no rung on the ladder owns yet.)
# (the overlay chain arrived with the ladder's rung 6b, so `global` is real and
# no longer refused. `group` still is: a group layer needs to know who is in a
# group, and no rung owns membership.)
VAR_SCOPE_AWAITS = {
    "group": "scope 'group' awaits a membership model — the overlay chain "
             "resolves straight through the group layer because nothing can "
             "say who is in one; declare user, global or device for now",
}
# a scope whose values live in the shared `_global` layer rather than in any
# user's world. The resolver reads the layer and never the user's own field.
VAR_SCOPE_LAYER = "ScopeGlobal"
# the hook: this token in a composed verbatim library switches slot collection
# and Context emission on. No hook in the composition -> no struct, and the
# emitted source is byte-identical to a build without this mechanism.
VAR_HOOK = "pub struct Var<"
# the second hook: a composed node whose Rust source carries this token is
# asking for Context::snapshot(), the generated walker over every declared var.
# No asker in the composition -> no walker, and no serde::Serialize demand on
# var types. An asker without the var family is a link error, not a rustc one.
SNAPSHOT_HOOK = "fm:context-snapshot"
# the third hook: a composed node whose Rust source carries this token is
# asking for Context::set_from_json() — the generated write path — and the
# Clone impl a turn's frozen view needs. Those are what impose
# serde::Deserialize and Clone on every var type, so a composition that never
# edits a context pays neither. An asker without the var family is a link
# error, exactly as for the snapshot hook.
SET_HOOK = "fm:context-set"
# the fourth hook: a composed node carrying this token asks for the enabled
# machinery — an implicit `enabled` var on EVERY composed node, a per-node
# `<node>_on()` conjunction down the composition tree, and a gate at the head
# of every chain-EXTENDING function that carries loop state. No asker -> none of
# it, and the emitted source is byte-identical to a build without this rung.
GATE_HOOK = "fm:context-gate"
# the implicit var every composed node gets while the gate hook is present.
# Same shape as a declared line in a .vars sidecar, so it rides the same
# emission, snapshot and write paths — there is nothing special about `enabled`.
GATE_VAR = {"name": "enabled", "type": "bool", "default": "true",
            "scope": "ScopeUser", "merge": "MergeLastWrite",
            "inherit": "Inherit"}
# a gated function is one whose FIRST parameter is `state: String` — the loop
# state travelling through an Elm chain. Chain-STARTING definitions are never
# gated (they are the seam the chain hangs from), and a function that does not
# carry loop state (a route, a helper, a startup hook) is not gated at all.
GATE_FIRST_PARAM = ("state", "String")
# the fifth hook: a composed node carrying this token asks for the two halves of
# the merge discipline — Context::edit_op(), which mutates a var through the
# write method its DECLARED merge earned and queues the resulting op, and
# Context::apply_op(), which applies an op that arrived over the wire after
# checking that its verb is the one the declaration speaks. No asker -> neither,
# and the emitted source is byte-identical to a build without this rung.
OP_HOOK = "fm:context-op"
# which write method a declared merge earns. A merge that is not in this table
# has no write API yet, and edit_op says so by name rather than guessing.
MERGE_WRITE = {"MergeLastWrite": ("set", "set_at"),
               "MergeCrdtSum": ("add", "add_at"),
               # the counter's DEFAULT verb; it also speaks `set`, which is
               # emitted separately because it is the only two-verb kind.
               "MergeCounter": ("add", "add_at")}
# the merge kind whose ops carry an epoch, and whose apply drops a stale add.
MERGE_EPOCH = "MergeCounter"
# the sixth hook: a composed node carrying this token persists and replays
# contexts. It asks for NOTHING to be emitted — the op log replays through
# Context::apply_op, which the fifth hook already provides — but declaring it
# lets a composition missing that door fail by name here rather than as a rustc
# error inside a verbatim library.
REMEMBER_HOOK = "fm:context-remember"
# the seventh hook: a composed node carrying this token asks for the overlay
# chain — a per-var presence record, a resolved read per var that falls from the
# user's own value through the shared layer to the declared default, the `clear`
# verb that returns a var to inheriting, and the scope lookup that routes a
# global var's ops to the layer. No asker -> none of it, no presence record, and
# every read stays the raw `.value` it was before this rung.
OVERLAY_HOOK = "fm:context-overlay"
# the eighth hook: a composed node carrying this token republishes bridged vars
# into the loop's state at their legacy keys, so a page fragment that reads
# `s.<key>` keeps working after the value has moved into the Context. No asker
# -> no republish, and a `js:` column without one is a link error naming both.
BRIDGE_HOOK = "fm:context-bridge"
# the ninth hook, and the only one that asks for no emission at all: a composed
# node carrying TURN_END_NEEDS has moved work that must happen after every link
# of a turn onto the turn-end phase, and is silently wrong without it — an op it
# minted would sit in the outbox instead of leaving. Nothing rustc can see would
# break, so the linker says it instead: a composition that needs the phase and
# does not compose it fails by name.
TURN_END_HOOK = "fm:turn-end-phase"
TURN_END_NEEDS = "fm:turn-end-required"

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
# a field ask's anchor: asks#<ms timestamp> — the filing time IS the position,
# read straight from the id, no lookup (agents.md "field asks are provenance
# too"; rebuilt 2026-08-21, hybrid #p68, second workaround = mechanism time)
ASK_CITE_RE = re.compile(r"asks#(\d{13})")


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
    if not m:
        a = ASK_CITE_RE.search(text)
        if not a:
            return None
        import datetime
        ms = int(a.group(1))
        stamp = datetime.datetime.fromtimestamp(ms / 1000).strftime(
            "%Y-%m-%d %H:%M")
        return (stamp, "asks", ms, "")
    key = (m.group(1), int(m.group(2)), m.group(3))
    if key not in times:
        fail(f"{real.relative_to(REPO)}: spec cites {m.group(1)}#p{m.group(2)}"
             f"{m.group(3)} but no such anchor exists in transcripts/")
    return (times[key],) + key


def contributes(directory: Path) -> bool:
    """Does this node add composition material (code, fragments, assets,
    deps, slots)? Pure grouping nodes don't, and are ordered by their subtree."""
    real = directory.resolve()
    if (real / "assets").is_dir() or (real / "deps.toml").exists():
        return True
    return any(f.is_file() and (f.suffix in (".rs", ".vars")
                                or f.suffix[1:] in EXT_SLOT
                                or f.name.endswith(".agent.md"))
               for f in real.iterdir())


def tie_break(directory: Path, root: Path) -> tuple:
    """The tie-break for two nodes citing the SAME prompt: (how deep in
    CONTRIBUTING nodes, then the path with code-free grouping ancestors
    removed).

    Ties used to break by (depth, path) counting every node, which meant a
    regroup — inserting a grouping node above tied siblings — pushed them a
    level down and past nodes elsewhere in the tree that shared their prompt.
    That rewires chains, which agents.md forbids a regroup from doing (found
    2026-08-25: the holding/changing regroup moved `dictate/mirror/adopt`
    through `converge`'s `handle_msg` links, all of them citing hybrid #p32).
    A grouping node contributes nothing to the composition, so it is not
    counted and not named here, and inserting or dissolving one is
    order-neutral by construction.

    A grouping node itself sorts immediately before its own subtree: it is one
    level shallower than the children it does not count, and its own last
    component is `\\x00`-prefixed for the nested case where two grouping nodes
    land at the same depth.
    """
    parts = directory.relative_to(root).parts
    cur, out = root, []
    for i, name in enumerate(parts):
        cur = cur / name
        if i == len(parts) - 1:
            out.append(name if contributes(cur) else "\x00" + name)
        elif contributes(cur):
            out.append(name)
    return (len(out), tuple(out))


def chronologise(feature_dirs: list, root: Path) -> list:
    """Sort the included nodes by provenance time. Ties (one prompt, several
    nodes) resolve by `tie_path` — containment first, tree position never. A code-free
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
    ordered = sorted(feature_dirs, key=lambda d: (key[d], tie_break(d, root)))
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
    """('name', [param names], [param types], return type or '') from
    'fn name(a: T, b: U) -> R'. The names are what a generated gate needs in
    order to hand the arguments on to the previous link of the chain."""
    m = re.search(r"fn\s+(\w+)\s*\(([^)]*)\)\s*(?:->\s*(.+?))?\s*$", header, re.S)
    if not m:
        fail(f"cannot parse fn signature: {header.strip()!r}")
    parts = [p for p in m.group(2).split(",") if ":" in p]
    names = [re.sub(r"^mut\s+", "", p.split(":", 1)[0].strip()) for p in parts]
    params = [p.split(":", 1)[1].strip() for p in parts]
    return m.group(1), names, params, (m.group(3) or "").strip()


class FeatureCode:
    """Everything one feature contributes: functions and struct fields."""

    def __init__(self, feature_dir: Path):
        self.dir = feature_dir
        # resolve symlinks so diagnostics point at the real source location
        self.rel = str(feature_dir.resolve().relative_to(REPO))
        self.name = None          # e.g. "Hello" from struct feature_Hello
        self.fns = []             # dicts: name, params, ret, src, first, lines
        self.structs = []         # (struct_name, [(field, type, src_file, line)])
        self.sources = []         # (src_rel, full text) of this node's chain .rs
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
                                       "src": self.rel.replace("features/", "", 1),
                                       # the tree-global address, which a
                                       # product-local override shares with the
                                       # shared node it replaces
                                       "node": node_path(self.rel)})
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
        # agent instruction fragments: the tree's third language (#p29 of
        # 2026-08-23-plans). Markdown the composition machinery never parses —
        # assembled verbatim, provenance-ordered, into the product's skillset;
        # toggleable with the node like any other implementation file.
        self.agent = []          # (src_rel, text)
        for af in sorted(feature_dir.glob("*.agent.md")):
            self.agent.append((str(af.resolve().relative_to(REPO)),
                               af.read_text()))
        # context var declarations: sidecar files the chain parser never sees
        self.vars = []           # dicts: name, type, default, scope/merge/inherit
        for sf in sorted(feature_dir.glob("*.vars")):
            self._parse_slots(sf)

    def _parse_slots(self, sf: Path):
        src = str(sf.resolve().relative_to(REPO))
        seen = {}
        for lineno, raw in enumerate(sf.read_text().splitlines(), 1):
            line = raw.split("#", 1)[0].strip()
            if not line:
                continue
            m = VAR_DECL_RE.match(line)
            if not m:
                fail(f"{src}:{lineno}: cannot parse var declaration "
                     f"{line!r} — expected "
                     f"'name: Type = default (scope, merge, inherit)'")
            name, ty, default, scope, merge, inherit, js = m.groups()
            for word, table, what in ((scope, VAR_SCOPE, "scope"),
                                      (merge, VAR_MERGE, "merge"),
                                      (inherit, VAR_INHERIT, "inherit")):
                if word not in table:
                    fail(f"{src}:{lineno}: unknown {what} '{word}' — "
                         f"expected one of {' | '.join(sorted(table))}")
            if scope in VAR_SCOPE_AWAITS:
                fail(f"{src}:{lineno}: {VAR_SCOPE_AWAITS[scope]}")
            if name in seen:
                fail(f"{src}:{lineno}: var '{name}' already declared on this "
                     f"node at line {seen[name]}")
            seen[name] = lineno
            self.vars.append({"name": name, "type": ty, "default": default,
                               "scope": VAR_SCOPE[scope],
                               "merge": VAR_MERGE[merge],
                               "inherit": VAR_INHERIT[inherit],
                               "js": js,
                               "src": src, "line": lineno})

    def _parse(self, rs: Path):
        text = rs.read_text()
        src = str(rs.resolve().relative_to(REPO))
        self.sources.append((src, text))

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
            name, pnames, params, ret = parse_signature(text[fn_start:body_open])
            first, last = line_of(text, fn_start), line_of(text, body_close)
            self.fns.append({"name": name, "pnames": pnames, "params": params,
                             "ret": ret, "src": src, "first": first,
                             # offset within `lines` of the line the body's
                             # opening brace sits on: where a gate is injected
                             "open_off": line_of(text, body_open) - first,
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
    """A node's tree-global address: its path with the features/ (or a product
    tree's products/<name>/) root stripped."""
    if rel.startswith("features/"):
        return rel[len("features/"):]
    m = re.match(r"products/[^/]+/(.*)$", rel)
    return m.group(1) if m else rel


def field_ident(node: str) -> str:
    """A node name as a Rust identifier fragment. Node names may carry hyphens
    (`reset-taps`); field and method names may not."""
    return re.sub(r"[^0-9A-Za-z_]", "_", node)


def gate_plan(features: list) -> dict:
    """rel -> {ident, path, parent rel} for every composed node, when the
    enabled machinery is switched on. The parent is looked up by node path, so
    a product-local override (products/miso/miso/loop) and a shared node
    (features/miso/loop/tap) sit in the same tree, which is what lets the
    ancestor conjunction be resolved at compile time rather than walked at
    runtime."""
    by_path = {node_path(f.rel): f.rel for f in features}
    plan, idents = {}, {}
    for f in features:
        path = node_path(f.rel)
        node = Path(f.rel).name
        ident = field_ident(node)
        if ident in idents and idents[ident] != f.rel:
            fail(f"nodes {idents[ident]} and {f.rel} both become the context "
                 f"identifier '{ident}' — rename one (node names are "
                 f"tree-global, fm.md)")
        idents[ident] = f.rel
        parent = path.rsplit("/", 1)[0] if "/" in path else None
        plan[f.rel] = {"ident": ident, "path": path,
                       "parent": by_path.get(parent) if parent else None}
    return plan


def emit_context_presence(fields: list, out: Emitter):
    """The presence record: one bool per var, mirroring the Context's fields.

    Presence belongs to a var, and the tidy home for it would be a field on
    `Var` — but that is rung 1's verbatim library, and putting it there would
    leave the bit in every composition whether or not it wanted overlays. A
    parallel record generated under the same hook keeps the property exactly as
    toggleable as the feature that needs it, and costs one bool per var.

    A var declared `own` starts present: it has no layer to fall to, so it is
    always its own answer. A var declared `inherit` starts ABSENT, holding its
    declared default, which is what makes 'never touched' expressible."""
    out.emit("// ---- context: presence, one bool per var (fm:context-overlay)")
    out.emit("#[derive(Clone)]")
    out.emit("pub struct Present {")
    for fname, path, s in fields:
        out.emit(f"    pub {fname}: bool,", s["src"], s["line"])
    out.emit("}")
    out.emit("")
    out.emit("impl Present {")
    out.emit("    pub fn fresh() -> Present {")
    out.emit("        Present {")
    for fname, _, s in fields:
        starts = "true" if s["inherit"] == "Own" else "false"
        out.emit(f"            {fname}: {starts},", s["src"], s["line"])
    out.emit("        }")
    out.emit("    }")
    out.emit("}")
    out.emit("")


def emit_context_resolve(fields: list, out: Emitter):
    """THE resolved read: one `<field>_get()` per var, and the scope lookup that
    routes an op to the world that owns it.

    Resolution falls through the overlay chain — own value if present, then the
    group layer, then the `_global` layer, then the declared default. The group
    step is written as a comment rather than as code because nothing can say who
    is in a group; when membership exists it lands between these two lines.

    A `global`-scoped var never consults the user's own field at all: its
    authority is the layer, and the field every user carries for it is unread
    ballast that keeps `Context` one shape."""
    out.emit("// ---- context: the resolved read (fm:context-overlay)")
    out.emit("impl Context {")
    for fname, path, s in fields:
        addr = f"{path}/{s['name']}"
        layered = (s["scope"] == VAR_SCOPE_LAYER)
        out.emit(f"    /// {addr} ({'global — the layer is the authority'
                                    if layered else s['inherit'].lower()})")
        out.emit(f"    pub fn {fname}_get(&self) -> {s['type']} {{",
                 s["src"], s["line"])
        if not layered and s["inherit"] != "Own":
            out.emit(f"        if self.present.{fname} {{")
            out.emit(f"            return self.{fname}.value.clone();",
                     s["src"], s["line"])
            out.emit("        }")
        elif not layered:
            # `own`: always present, never falls through
            out.emit(f"        return self.{fname}.value.clone();",
                     s["src"], s["line"])
            out.emit("    }")
            continue
        # ... the group layer would be consulted here ...
        out.emit(f"        if let Some(v) = context_layer(|g| if g.present.{fname} "
                 f"{{ Some(g.{fname}.value.clone()) }} else {{ None }}) {{",
                 s["src"], s["line"])
        out.emit("            return v;")
        out.emit("        }")
        out.emit(f"        {s['default']}", s["src"], s["line"])
        out.emit("    }")
    out.emit("")
    out.emit("    // which world owns a var's authority: the shared layer for a")
    out.emit("    // global-scoped one, the requester's own for everything else.")
    out.emit("    pub fn scope_of(path: &str, name: &str) -> Option<&'static str> {")
    out.emit("        match (path, name) {")
    for fname, path, s in fields:
        tag = next(k for k, v in VAR_SCOPE.items() if v == s["scope"])
        out.emit(f"            ({json.dumps(path)}, {json.dumps(s['name'])}) "
                 f"=> Some({json.dumps(tag)}),", s["src"], s["line"])
    out.emit("            _ => None,")
    out.emit("        }")
    out.emit("    }")
    out.emit("")
    out.emit("    // whether a var may be cleared: an `own` var has no layer to")
    out.emit("    // fall back to, so returning it to inheriting is meaningless.")
    out.emit("    pub fn inherits(path: &str, name: &str) -> bool {")
    out.emit("        match (path, name) {")
    for fname, path, s in fields:
        out.emit(f"            ({json.dumps(path)}, {json.dumps(s['name'])}) "
                 f"=> {'false' if s['inherit'] == 'Own' else 'true'},",
                 s["src"], s["line"])
    out.emit("            _ => false,")
    out.emit("        }")
    out.emit("    }")
    out.emit("}")
    out.emit("")


def emit_context_republish(fields: list, out: Emitter):
    """Carry every bridged var's RESOLVED value back into the loop's state, at
    the legacy key a page fragment still reads.

    This is the one direction the world-object did not have: rungs 1-6b moved
    values out of the JSON state, and a JavaScript fragment cannot call
    with_context. Rather than edit every fragment (six read `open_tool` alone),
    a var may name the key it used to live at and the bridge writes it back
    before the paint. The values are the resolved ones, so what a fragment sees
    is what a gate would see."""
    out.emit("// ---- context: bridged vars, back into the payload"
             " (fm:context-bridge)")
    out.emit("impl Context {")
    out.emit("    pub fn republish(&self, state: &mut serde_json::Value) {")
    for fname, path, s in fields:
        key = s.get("js")
        if not key:
            continue
        out.emit(f"        // {path}/{s['name']}")
        out.emit(f"        state[{json.dumps(key)}] = "
                 f"serde_json::to_value(&self.{fname}_get())",
                 s["src"], s["line"])
        out.emit("            .unwrap_or(serde_json::Value::Null);")
    out.emit("    }")
    out.emit("}")
    out.emit("")


def emit_gate_predicates(features: list, plan: dict, out: Emitter, src, line,
                         resolved: bool = False):
    """`<node>_on()` per composed node: own enabled AND the parent's answer.
    The tree is known at link time, so an ancestor's untick silences its whole
    subtree through a conjunction rustc inlines — no path string is ever
    compared at runtime, which is the old design's `':'` bug made
    inexpressible."""
    out.emit("// ---- context: effective enablement (fm:context-gate)")
    out.emit("//   <node>_on() = this node's enabled var AND its parent's")
    out.emit("//   answer. A root node answers from its own field alone.")
    out.emit("impl Context {")
    for f in sorted(features, key=lambda f: plan[f.rel]["path"]):
        me = plan[f.rel]
        parent = plan.get(me["parent"]) if me["parent"] else None
        conj = (f" && self.{parent['ident']}_on()" if parent else "")
        # with the overlay composed a gate reads the RESOLVED enablement, which
        # is what lets a value set once on the shared layer reach every user who
        # never overrode it.
        own = (f"self.{me['ident']}_enabled_get()" if resolved
               else f"self.{me['ident']}_enabled.value")
        out.emit(f"    /// {me['path']}")
        out.emit(f"    pub fn {me['ident']}_on(&self) -> bool "
                 f"{{ {own}{conj} }}", src, line)
    out.emit("")
    # the tick map the chooser's page half has always read: one entry per node
    # whose OWN enablement resolves off, absent meaning on. Ancestor shading is
    # the page's own prefix walk and stays that way, so the map means exactly
    # what it meant when a user's explicit choices were stored in it.
    #
    # It is emitted here rather than behind a hook of its own because it IS the
    # gate machinery's view of itself — the same per-node fields, in path
    # order — and because a walk of the snapshot would serialise every var's
    # value on every event to answer a question about bools.
    out.emit("    // the chooser's derived tick map: explicit false for a node")
    out.emit("    // whose own enablement resolves off; absent means on.")
    out.emit("    pub fn enabled_off_map(&self) -> serde_json::Value {")
    out.emit("        let mut m = serde_json::Map::new();")
    for f in sorted(features, key=lambda f: plan[f.rel]["path"]):
        me = plan[f.rel]
        own = (f"self.{me['ident']}_enabled_get()" if resolved
               else f"self.{me['ident']}_enabled.value")
        out.emit(f"        if !{own} {{", src, line)
        out.emit(f"            m.insert({json.dumps(me['path'])}.to_string(),")
        out.emit("                     serde_json::Value::Bool(false));")
        out.emit("        }")
    out.emit("        serde_json::Value::Object(m)")
    out.emit("    }")
    out.emit("}")
    out.emit("")


def emit_context(features: list, out: Emitter):
    """Collect every composed node's .vars declarations and emit the Context
    struct plus Context::fresh(). Scaffolding: the mechanism lives here, the
    design and the slot types live in features/miso/loop/context.

    A field is named <node name>_<slot name>. Node names are tree-global
    (fm.md), so that disambiguates two nodes declaring the same slot name —
    and unlike a flattened path it survives a regroup, which the tree's own
    law says must never change behaviour. The full path rides in a comment.

    Context::snapshot() rides on a second hook (SNAPSHOT_HOOK): it is emitted
    only when a composed node asks for it, because it is what imposes
    serde::Serialize on every declared var type. Context::set_from_json() and
    the Clone impl ride on a third (SET_HOOK), for the same reason with
    Deserialize and Clone."""
    asks_snapshot = [f.rel for f in features
                     for _, text in f.sources if SNAPSHOT_HOOK in text]
    asks_set = [f.rel for f in features
                for _, text in f.sources if SET_HOOK in text]
    asks_gate = [(f.rel, src) for f in features
                 for src, text in f.sources + f.libs if GATE_HOOK in text]
    asks_op = [f.rel for f in features
               for _, text in f.sources + f.libs if OP_HOOK in text]
    asks_remember = [f.rel for f in features
                     for _, text in f.sources + f.libs if REMEMBER_HOOK in text]
    asks_overlay = [f.rel for f in features
                    for _, text in f.sources + f.libs if OVERLAY_HOOK in text]
    asks_bridge = [f.rel for f in features
                   for _, text in f.sources + f.libs if BRIDGE_HOOK in text]
    if not any(VAR_HOOK in text for f in features for _, text in f.libs):
        for asker, what, token in ((asks_snapshot, "Context::snapshot()", SNAPSHOT_HOOK),
                                   (asks_set, "Context::set_from_json()", SET_HOOK),
                                   ([a[0] for a in asks_gate],
                                    "the enabled gates", GATE_HOOK),
                                   (asks_op, "the var op methods", OP_HOOK),
                                   (asks_remember, "context persistence",
                                    REMEMBER_HOOK),
                                   (asks_overlay, "the overlay chain",
                                    OVERLAY_HOOK),
                                   (asks_bridge, "the payload bridge",
                                    BRIDGE_HOOK)):
            if asker:
                fail(f"{asker[0]} asks for {what} "
                     f"('{token}') but no composed node provides the var "
                     f"family — tick loop/context, or untick the asking node")
        return
    # the gates read the turn's frozen view, which is loop/context/changing/edit's
    # machinery: no frozen read, no gate. Loudly, because a silently ungated
    # build is one whose tickboxes do nothing.
    if asks_gate and not asks_set:
        fail(f"{asks_gate[0][0]} asks for the enabled gates ('{GATE_HOOK}') but "
             f"no composed node provides the frozen-read machinery "
             f"('{SET_HOOK}') — tick loop/context/changing/edit, or untick the gates")
    # an arriving CtxUpdate is applied through set_from_json, and a local edit
    # is made under edit_context: both are loop/context/changing/edit's.
    if asks_op and not asks_set:
        fail(f"{asks_op[0]} asks for the var op methods ('{OP_HOOK}') but no "
             f"composed node provides the write path ('{SET_HOOK}') — tick "
             f"loop/context/changing/edit, or untick the op methods")
    # a log replays through apply_op and through nothing else: no op methods,
    # no recovery, and a persisted world that could not be rebuilt would be
    # worse than one that was never written.
    if asks_remember and not asks_op:
        fail(f"{asks_remember[0]} persists contexts ('{REMEMBER_HOOK}') but no "
             f"composed node provides the op methods ('{OP_HOOK}') that a log "
             f"replays through — tick loop/context/changing/converge, or untick it")
    # the overlay resolves a var by falling from the user's own value through
    # the shared layer, and both halves of that are op machinery: `clear` is an
    # op verb, and a global var's authority is reached by routing its ops.
    if asks_overlay and not asks_op:
        fail(f"{asks_overlay[0]} asks for the overlay chain ('{OVERLAY_HOOK}') "
             f"but no composed node provides the op methods ('{OP_HOOK}') it "
             f"resolves and routes through — tick loop/context/changing/converge, or "
             f"untick the overlay")
    if not asks_overlay:
        # global scope has no layer to live in without the overlay composed, so
        # the refusal that rung 6b lifted comes back for this composition.
        for feature in features:
            for s in feature.vars:
                if s["scope"] == VAR_SCOPE_LAYER:
                    fail(f"{s['src']}:{s['line']}: scope 'global' needs the "
                         f"overlay chain ('{OVERLAY_HOOK}') — tick "
                         f"loop/context/changing/converge/overlay, or declare user")
    # a `js:` column is a promise to a page fragment, and a promise nothing
    # keeps is worse than one nobody made: a build whose declarations claim
    # legacy keys but whose bridge is absent would render blank rather than
    # fail, so it fails.
    # the bridge republishes RESOLVED values, which is the overlay's read.
    if asks_bridge and not asks_overlay:
        fail(f"{asks_bridge[0]} republishes bridged vars ('{BRIDGE_HOOK}') but "
             f"no composed node provides the resolved read ('{OVERLAY_HOOK}') "
             f"it republishes — tick loop/context/changing/converge/overlay, or untick "
             f"the bridge")
    bridged = {}
    for feature in features:
        for s in feature.vars:
            key = s.get("js")
            if not key:
                continue
            if not asks_bridge:
                fail(f"{s['src']}:{s['line']}: '{s['name']}' claims the page "
                     f"key '{key}' (js:), but no composed node provides the "
                     f"payload bridge ('{BRIDGE_HOOK}') — tick "
                     f"loop/context/changing/converge/payload, or drop the js: column")
            if key in bridged:
                prev = bridged[key]
                fail(f"{s['src']}:{s['line']}: page key '{key}' is already "
                     f"claimed by {prev['src']}:{prev['line']} — two vars "
                     f"cannot republish to one key")
            bridged[key] = s
    plan = gate_plan(features) if asks_gate else None
    gate_src, gate_line = (asks_gate[0][1], 1) if asks_gate else (None, None)
    fields = []
    for feature in features:
        node = field_ident(Path(feature.rel).name)
        if plan:
            # the implicit var: every composed node has an `enabled`, emitted
            # exactly like a declared one, so it rides the snapshot and the
            # write path with no special case anywhere downstream.
            fields.append((f"{node}_enabled", node_path(feature.rel),
                           dict(GATE_VAR, src=gate_src, line=gate_line)))
        for s in feature.vars:
            if plan and s["name"] == GATE_VAR["name"]:
                fail(f"{s['src']}:{s['line']}: node "
                     f"'{Path(feature.rel).name}' declares its own 'enabled' "
                     f"var, but loop/context/changing/enabled gives every composed node "
                     f"one — remove the declaration, or untick the gates")
            fields.append((f"{node}_{s['name']}", node_path(feature.rel), s))
    out.emit("// ---- context: vars declared by composed nodes (<name>.vars)")
    out.emit("pub struct Context {")
    for fname, path, s in fields:
        out.emit(f"    // {path}/{s['name']} ({s['src']}:{s['line']})")
        out.emit(f"    pub {fname}: Var<{s['type']}, {s['scope']}, "
                 f"{s['merge']}, {s['inherit']}>,", s["src"], s["line"])
    if asks_overlay:
        out.emit("    // fm:context-overlay — which vars have been written. A")
        out.emit("    // var that has not is what makes `inherit` expressible.")
        out.emit("    pub present: Present,")
    out.emit("}")
    out.emit("")
    if asks_overlay:
        emit_context_presence(fields, out)
    out.emit("impl Context {")
    out.emit("    pub fn fresh() -> Context {")
    out.emit("        Context {")
    for fname, _, s in fields:
        out.emit(f"            {fname}: Var::new({s['default']}),",
                 s["src"], s["line"])
    if asks_overlay:
        out.emit("            present: Present::fresh(),")
    out.emit("        }")
    out.emit("    }")
    out.emit("}")
    out.emit("")
    if asks_overlay:
        emit_context_resolve(fields, out)
    if asks_bridge:
        emit_context_republish(fields, out)
    if plan:
        emit_gate_predicates(features, plan, out, gate_src, gate_line,
                             bool(asks_overlay))
    if not asks_snapshot:
        emit_context_set(fields, asks_set, out, bool(asks_overlay))
        emit_context_ops(fields, asks_op, out, bool(asks_overlay))
        return plan
    out.emit("impl Context {")
    out.emit("    // every declared var as JSON — node path, name, current")
    out.emit("    // value, and the three attributes read from the markers'")
    out.emit("    // TAGs. serde_json::to_value is what demands Serialize of a")
    out.emit("    // var's type; a type that hasn't got it fails on the line")
    out.emit("    // below, which the line map points back at the .vars file.")
    out.emit("    pub fn snapshot(&self) -> serde_json::Value {")
    out.emit("        let mut vars: Vec<serde_json::Value> = Vec::new();")
    for fname, path, s in fields:
        out.emit(f"        let a = self.{fname}.attrs();", s["src"], s["line"])
        out.emit("        vars.push(serde_json::json!({")
        out.emit(f"            \"path\": {json.dumps(path)},")
        out.emit(f"            \"name\": {json.dumps(s['name'])},")
        out.emit(f"            \"value\": serde_json::to_value(&self.{fname}.value)",
                 s["src"], s["line"])
        out.emit("                .unwrap_or(serde_json::Value::Null),")
        out.emit("            \"scope\": a.0, \"merge\": a.1, \"inherit\": a.2,")
        if asks_overlay:
            # additive, so rung 2's readers keep working: `value` is still this
            # world's own field, `resolved` is what a reader would actually get.
            out.emit(f"            \"present\": self.present.{fname},",
                     s["src"], s["line"])
            out.emit(f"            \"resolved\": serde_json::to_value(&self.{fname}_get())",
                     s["src"], s["line"])
            out.emit("                .unwrap_or(serde_json::Value::Null),")
        out.emit("        }));")
    out.emit("        serde_json::Value::Array(vars)")
    out.emit("    }")
    out.emit("}")
    out.emit("")
    emit_context_set(fields, asks_set, out, bool(asks_overlay))
    emit_context_ops(fields, asks_op, out, bool(asks_overlay))
    return plan


def emit_counter_apply(fname: str, addr: str, s: dict, out: Emitter,
                       overlay: bool):
    """The arriving half of the `counter` merge — the only kind that speaks two
    verbs. A `set` opens a new epoch and assigns; an `add` sums only if it was
    minted under the epoch the var is in now. An add from before a reset is
    DROPPED, and says so on stderr: reset wins, and the loss is deliberate
    rather than the silent one SyncVar had (converge.md argues the direction)."""
    if overlay:
        out.emit('                if op == "clear" {')
        if s["inherit"] == "Own":
            refuse = json.dumps(
                f"{addr}: declared 'own', so it has nothing beneath it to fall "
                f"back to — a clear is meaningless", ensure_ascii=False)
            out.emit(f"                    return Err({refuse}.to_string());",
                     s["src"], s["line"])
        else:
            out.emit(f"                    self.present.{fname} = false;",
                     s["src"], s["line"])
            out.emit(f"                    self.{fname}.value = {s['default']};",
                     s["src"], s["line"])
            out.emit("                    return Ok(serde_json::to_value("
                     f"&self.{fname}_get())", s["src"], s["line"])
            out.emit("                        .unwrap_or(serde_json::Value::Null));")
        out.emit("                }")
    out.emit(f"                let c: Counter = serde_json::from_value(value)",
             s["src"], s["line"])
    out.emit(f"                    .map_err(|e| format!("
             f"{json.dumps(addr + ': {}')}, e))?;")
    out.emit('                if op == "set" {')
    stale_set = json.dumps(
        f"miso: context: {addr}: reset at epoch {{}} is older than the current "
        f"epoch {{}} — dropped", ensure_ascii=False)
    out.emit(f"                    if c.epoch < self.{fname}.value.epoch {{",
             s["src"], s["line"])
    out.emit(f"                        eprintln!({stale_set}, c.epoch, self.{fname}.value.epoch);",
             s["src"], s["line"])
    out.emit("                    } else {")
    out.emit(f"                        self.{fname}.value = c;", s["src"], s["line"])
    if overlay:
        out.emit(f"                        self.present.{fname} = true;",
                 s["src"], s["line"])
    out.emit("                    }")
    out.emit(f"                    return Ok(serde_json::to_value(&self.{fname}.value)",
             s["src"], s["line"])
    out.emit("                        .unwrap_or(serde_json::Value::Null));")
    out.emit("                }")
    out.emit('                if op == "add" {')
    stale_add = json.dumps(
        f"miso: context: {addr}: add of {{}} minted under epoch {{}} arrived "
        f"after a reset to epoch {{}} — dropped", ensure_ascii=False)
    out.emit(f"                    if c.epoch != self.{fname}.value.epoch {{",
             s["src"], s["line"])
    out.emit(f"                        eprintln!({stale_add}, c.sum, c.epoch, self.{fname}.value.epoch);",
             s["src"], s["line"])
    out.emit("                    } else {")
    out.emit(f"                        self.{fname}.value.sum = self.{fname}.value.sum + c.sum;",
             s["src"], s["line"])
    if overlay:
        out.emit(f"                        self.present.{fname} = true;",
                 s["src"], s["line"])
    out.emit("                    }")
    out.emit(f"                    return Ok(serde_json::to_value(&self.{fname}.value)",
             s["src"], s["line"])
    out.emit("                        .unwrap_or(serde_json::Value::Null));")
    out.emit("                }")
    speaks = json.dumps(f"{addr}: merge 'counter' speaks set and add, not " + "{}")
    out.emit(f"                Err(format!({speaks}, op))")


def emit_edit_reset(fields: list, out: Emitter, overlay: bool):
    """The counter kind's second verb, emitted only when a composition
    declares a counter — so a build with none is what it was before."""
    out.emit("    // a LOCAL edit through the OTHER verb, for the one merge kind")
    out.emit("    // that has two. On a counter this is the reset: it opens a new")
    out.emit("    // epoch, which is what lets every add still in flight from")
    out.emit("    // before it be recognised and dropped on arrival.")
    out.emit("    pub fn edit_reset(&mut self, path: &str, name: &str,"
             " value: serde_json::Value) -> Result<serde_json::Value, String> {")
    out.emit("        match (path, name) {")
    for fname, path, s in fields:
        addr = f"{path}/{s['name']}"
        tag = next(k for k, v in VAR_MERGE.items() if v == s["merge"])
        out.emit(f"            ({json.dumps(path)}, {json.dumps(s['name'])}) => {{",
                 s["src"], s["line"])
        if s["merge"] == MERGE_EPOCH:
            out.emit("                let v: u64 = serde_json::from_value(value)",
                     s["src"], s["line"])
            out.emit(f"                    .map_err(|e| format!("
                     f"{json.dumps(addr + ' (reset): {}')}, e))?;")
            out.emit(f"                self.{fname}.set_at("
                     f"{json.dumps(path)}, {json.dumps(s['name'])}, v);",
                     s["src"], s["line"])
            if overlay:
                out.emit(f"                self.present.{fname} = true;",
                         s["src"], s["line"])
            out.emit(f"                Ok(serde_json::to_value(&self.{fname}.value)",
                     s["src"], s["line"])
            out.emit("                    .unwrap_or(serde_json::Value::Null))")
        else:
            no = json.dumps(f"{addr}: merge {tag!r} has one verb; edit_op is it")
            out.emit("                let _ = value;")
            out.emit(f"                Err({no}.to_string())")
        out.emit("            }")
    out.emit("            _ => Err(context_op_miss(path, name)),")
    out.emit("        }")
    out.emit("    }")
    out.emit("")


def emit_context_ops(fields: list, asks_op: list, out: Emitter,
                     overlay: bool = False):
    """The merge discipline's two generated halves. Scaffolding: mechanism here,
    design in features/miso/loop/context/changing/converge.

    `edit_op` is a LOCAL edit — it reaches for the write method the var's
    DECLARED merge earned (`set_at` on MergeLastWrite, `add_at` on
    MergeCrdtSum), which is what queues the outgoing op. The caller never picks
    the verb, and could not: the method it would have to name does not exist on
    the other marker, so a mis-declared call is a rustc error.

    `apply_op` is the arriving half — it checks the op's verb against the same
    declaration and then assigns directly, without going through the write
    methods, so applying a remote op never queues an echo of itself."""
    if not asks_op:
        return
    declared = ", ".join(f"{path}/{s['name']}" for _, path, s in fields) or "(none)"
    out.emit("// ---- context: var ops by declared merge (fm:context-op)")
    # one listing, shared by both miss arms rather than inlined into each
    miss = json.dumps("no var {}/{} — declared: " + declared, ensure_ascii=False)
    out.emit("fn context_op_miss(path: &str, name: &str) -> String {")
    out.emit(f"    format!({miss}, path, name)")
    out.emit("}")
    out.emit("")
    out.emit("impl Context {")
    out.emit("    // a LOCAL edit: mutate through the declared merge's write")
    out.emit("    // method, which queues the op, and answer with the resolved")
    out.emit("    // value. `value` is the new value for a last-write var and")
    out.emit("    // the DELTA for a crdt-sum one — the declaration says which.")
    out.emit("    pub fn edit_op(&mut self, path: &str, name: &str,"
             " value: serde_json::Value) -> Result<serde_json::Value, String> {")
    out.emit("        match (path, name) {")
    for fname, path, s in fields:
        addr = f"{path}/{s['name']}"
        out.emit(f"            ({json.dumps(path)}, {json.dumps(s['name'])}) => {{",
                 s["src"], s["line"])
        if s["merge"] not in MERGE_WRITE:
            tag = next(k for k, v in VAR_MERGE.items() if v == s["merge"])
            out.emit(f"                let _ = value;")
            out.emit(f"                Err({json.dumps(addr)}.to_string() + "
                     f"{json.dumps(f': merge {tag!r} has no write API yet')})")
        else:
            verb, method = MERGE_WRITE[s["merge"]]
            ty = "u64" if verb == "add" else s["type"]
            what = "delta" if verb == "add" else "value"
            out.emit(f"                let v: {ty} = serde_json::from_value(value)",
                     s["src"], s["line"])
            out.emit(f"                    .map_err(|e| format!("
                     f"{json.dumps(addr + ' (' + what + '): {}')}, e))?;")
            out.emit(f"                self.{fname}.{method}("
                     f"{json.dumps(path)}, {json.dumps(s['name'])}, v);",
                     s["src"], s["line"])
            if overlay:
                out.emit(f"                self.present.{fname} = true;",
                         s["src"], s["line"])
            out.emit(f"                Ok(serde_json::to_value(&self.{fname}.value)",
                     s["src"], s["line"])
            out.emit("                    .unwrap_or(serde_json::Value::Null))")
        out.emit("            }")
    out.emit("            _ => Err(context_op_miss(path, name)),")
    out.emit("        }")
    out.emit("    }")
    out.emit("")
    # the second verb is emitted only when something speaks it, so a
    # composition with no counter declared is unchanged by this rung.
    if any(s["merge"] == MERGE_EPOCH for _, _, s in fields):
        emit_edit_reset(fields, out, overlay)
    out.emit("    // an ARRIVING op. The verb must be the one this var's merge")
    out.emit("    // speaks; a set aimed at a crdt-sum var is a wire error, not")
    out.emit("    // a silent overwrite. Assignment is direct, so applying a")
    out.emit("    // remote op never queues an echo of itself.")
    out.emit("    pub fn apply_op(&mut self, path: &str, name: &str, op: &str,"
             " value: serde_json::Value) -> Result<serde_json::Value, String> {")
    out.emit("        match (path, name) {")
    for fname, path, s in fields:
        addr = f"{path}/{s['name']}"
        tag = next(k for k, v in VAR_MERGE.items() if v == s["merge"])
        out.emit(f"            ({json.dumps(path)}, {json.dumps(s['name'])}) => {{",
                 s["src"], s["line"])
        if s["merge"] not in MERGE_WRITE:
            out.emit("                let _ = value;")
            out.emit(f"                Err(format!({json.dumps(addr + ': merge ' + repr(tag) + ' speaks no op (got {})')}, op))")
        elif s["merge"] == MERGE_EPOCH:
            emit_counter_apply(fname, addr, s, out, overlay)
        else:
            verb, _ = MERGE_WRITE[s["merge"]]
            if overlay:
                # `clear` returns a var to inheriting. An `own` var has no layer
                # beneath it, so clearing one is refused by name.
                out.emit('                if op == "clear" {')
                if s["inherit"] == "Own":
                    # ensure_ascii=False: Rust wants \u{XXXX}, not \uXXXX
                    refuse = json.dumps(
                        f"{addr}: declared 'own', so it has nothing beneath it "
                        f"to fall back to — a clear is meaningless",
                        ensure_ascii=False)
                    out.emit(f"                    return Err({refuse}.to_string());",
                             s["src"], s["line"])
                else:
                    out.emit(f"                    self.present.{fname} = false;",
                             s["src"], s["line"])
                    out.emit(f"                    self.{fname}.value = {s['default']};",
                             s["src"], s["line"])
                    out.emit("                    return Ok(serde_json::to_value("
                             f"&self.{fname}_get())", s["src"], s["line"])
                    out.emit("                        .unwrap_or(serde_json::Value::Null));")
                out.emit("                }")
            wrong = json.dumps(f"{addr}: merge {tag!r} speaks {verb!r}, not " + "{}")
            out.emit(f"                if op != {json.dumps(verb)} {{")
            out.emit(f"                    return Err(format!({wrong}, op));")
            out.emit("                }")
            ty = "u64" if verb == "add" else s["type"]
            out.emit(f"                let v: {ty} = serde_json::from_value(value)",
                     s["src"], s["line"])
            out.emit(f"                    .map_err(|e| format!("
                     f"{json.dumps(addr + ': {}')}, e))?;")
            if verb == "add":
                out.emit(f"                self.{fname}.value = self.{fname}.value + v;",
                         s["src"], s["line"])
            else:
                out.emit(f"                self.{fname}.value = v;", s["src"], s["line"])
            if overlay:
                out.emit(f"                self.present.{fname} = true;",
                         s["src"], s["line"])
            out.emit(f"                Ok(serde_json::to_value(&self.{fname}.value)",
                     s["src"], s["line"])
            out.emit("                    .unwrap_or(serde_json::Value::Null))")
        out.emit("            }")
    out.emit("            _ => Err(context_op_miss(path, name)),")
    out.emit("        }")
    out.emit("    }")
    out.emit("}")
    out.emit("")


def emit_context_set(fields: list, asks_set: list, out: Emitter,
                     overlay: bool = False):
    """The Context's generated write path, plus the Clone a turn's frozen view
    needs. Scaffolding: mechanism here, design in features/miso/loop/context/changing/edit.

    set_from_json is a match over every declared var keyed by (node path, var
    name) — the same two strings the snapshot reports — deserialising the given
    JSON into the var's own Rust type. A miss names what it got and what exists;
    a type mismatch returns serde's own message and leaves the var alone,
    because the assignment is downstream of the `?`."""
    if not asks_set:
        return
    out.emit("// ---- context: the write path (fm:context-set)")
    out.emit("impl Clone for Context {")
    out.emit("    // a turn freezes the context by cloning it; that is what")
    out.emit("    // demands Clone of every var type.")
    out.emit("    fn clone(&self) -> Context {")
    out.emit("        Context {")
    for fname, _, s in fields:
        out.emit(f"            {fname}: self.{fname}.clone(),", s["src"], s["line"])
    if overlay:
        out.emit("            present: self.present.clone(),")
    out.emit("        }")
    out.emit("    }")
    out.emit("}")
    out.emit("")
    declared = ", ".join(f"{path}/{s['name']}" for _, path, s in fields) or "(none)"
    out.emit("impl Context {")
    out.emit("    // set one declared var from JSON, addressed by the node path")
    out.emit("    // and var name the snapshot reports. serde_json::from_value is")
    out.emit("    // what demands Deserialize of a var's type; a type that hasn't")
    out.emit("    // got it fails on the line below, which the line map points")
    out.emit("    // back at the .vars file.")
    out.emit("    pub fn set_from_json(&mut self, path: &str, name: &str,"
             " value: serde_json::Value) -> Result<(), String> {")
    out.emit("        match (path, name) {")
    for fname, path, s in fields:
        addr = f"{path}/{s['name']}"
        out.emit(f"            ({json.dumps(path)}, {json.dumps(s['name'])}) => {{",
                 s["src"], s["line"])
        out.emit(f"                self.{fname}.value = serde_json::from_value(value)",
                 s["src"], s["line"])
        out.emit(f"                    .map_err(|e| format!({json.dumps(addr + ': {}')}, e))?;")
        if overlay:
            # a write makes a var present: it stops inheriting from here on.
            out.emit(f"                self.present.{fname} = true;",
                     s["src"], s["line"])
        out.emit("                Ok(())")
        out.emit("            }")
    # non-ASCII stays literal: json.dumps would emit \uXXXX, which is not a
    # Rust escape (Rust wants \u{XXXX}) — and a Rust source file is UTF-8.
    miss = json.dumps("no var {}/{} — declared: " + declared, ensure_ascii=False)
    out.emit(f"            _ => Err(format!({miss}, path, name)),")
    out.emit("        }")
    out.emit("    }")
    out.emit("}")
    out.emit("")


def gate_line(fn: dict, key: tuple, heads: dict, plan_entry: dict) -> str:
    """The gate injected at the head of a chain-extending, state-carrying
    function: if this node is not effectively on in the turn's frozen view,
    hand the previous link's answer back untouched. `gate_open` is
    loop/context/changing/enabled's own read primitive, so the frozen-view rule lives in
    one place; the predicate is a method call rustc resolves statically."""
    args = ", ".join(fn["pnames"])
    return (f"        if !gate_open(|c| c.{plan_entry['ident']}_on()) "
            f"{{ return feature_{heads[key]}::{fn['name']}({args}); }}"
            f"   // fm: gate {plan_entry['path']}")


def gated(fn: dict, key: tuple, chains: dict) -> bool:
    """A function is gated when it EXTENDS an existing chain and carries the
    loop state. A chain-starting definition is the seam the chain hangs from
    and has no previous answer to return; a function that takes no `state` is
    machinery (a route, a helper, a startup hook), not behaviour a user ticks."""
    return (key in chains
            and (fn["pnames"][:1], fn["params"][:1])
            == ([GATE_FIRST_PARAM[0]], [GATE_FIRST_PARAM[1]]))


def compose_features(features: list, out: Emitter, plan=None) -> dict:
    """Emit feature impl blocks; return chains keyed by (name, param types)."""
    chains = {}   # key -> {"head": feature struct name, "params": [...], "ret": str}
    gates = {}    # rel -> how many gates this place injected for that node
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
            inject = (gate_line(fn, key, heads, plan[feature.rel])
                      if plan and gated(fn, key, chains) else None)
            if inject:
                gates[feature.rel] = gates.get(feature.rel, 0) + 1
            for offset, text in enumerate(fn["lines"]):
                out.emit(rewrite_existing(text, fn, key, heads, feature),
                         fn["src"], fn["first"] + offset)
                if inject and offset == fn["open_off"]:
                    if text[text.rfind("{") + 1:].strip():
                        fail(f"{fn['src']}:{fn['first']}: fn {fn['name']} opens "
                             f"its body on a line that already carries code, so "
                             f"the enabled gate has nowhere to go — put the "
                             f"body on its own lines")
                    out.emit(inject, fn["src"], fn["first"])
        out.emit("}")
        out.emit("")
        for fn in feature.fns:
            key = (fn["name"], tuple(fn["params"]))
            members = chains[key]["members"] if key in chains else []
            chains[key] = {"head": feature.name, "params": fn["params"],
                           "ret": fn["ret"],
                           "members": members + [feature.rel]}
    for rel, n in gates.items():
        coverage_note(node_path(rel), "rust", n)
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


def check_turn_end(features: list):
    """A node that has handed work to the turn-end phase must not be composed
    without it. Scaffolding, per the standing arrangement: the mechanism is
    here, the design is in features/miso/loop/context/changing/edit/turn-end.

    This is the one dependency in the family that rustc cannot catch. Every
    other hook removes a generated function, so an asker without its provider
    fails to compile; the phase removes nothing — it moves WHEN something runs.
    Composed without it, /converge would queue ops that nothing drains and the
    app would look fine while nothing synced, which is the worst shape a
    failure can have. So it fails here, by name."""
    needs = [f.rel for f in features
             for _, text in f.sources + f.libs if TURN_END_NEEDS in text]
    has = [f.rel for f in features
           for _, text in f.sources + f.libs if TURN_END_HOOK in text]
    if needs and not has:
        fail(f"{needs[0]} has moved its end-of-turn work onto the turn-end "
             f"phase ('{TURN_END_NEEDS}') but no composed node provides it "
             f"('{TURN_END_HOOK}') — tick loop/context/changing/edit/turn-end. Without "
             f"it a local edit's op is queued and never shipped, which nothing "
             f"in the build would complain about.")


def compose(features: list):
    """Compose the placeless body once; entry glue is appended per place."""
    out = Emitter()
    out.emit("// generated by fm linker v0 — do not edit; edit features/ instead")
    out.emit("#![allow(non_camel_case_types, dead_code, non_snake_case, unused)]")
    out.emit("")
    check_turn_end(features)
    merge_structs(features, out)
    for feature in features:
        for src, text in feature.libs:
            out.emit(f"// ---- library: {src}")
            for offset, line in enumerate(text.splitlines()):
                out.emit(line, src, offset + 1)
            out.emit("")
    plan = emit_context(features, out)
    chains = compose_features(features, out, plan)
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


# --quick builds the debug profile: no lto, no opt — for toggle-proof and
# rig cycles, where seconds matter and artifact size does not. Deploy always
# uses release (deploy.sh calls fmlink without --quick).
BUILD_PROFILE = "release"


def cargo_build(crate_dir: Path, emitter: Emitter, wasm: bool, label: str):
    cmd = ["cargo", "build", "--message-format=json"]
    if BUILD_PROFILE == "release":
        cmd.insert(2, "--release")
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
        run_binary(build_dir / "target" / BUILD_PROFILE / product, build_dir)


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
                        / BUILD_PROFILE / f"{crate}.wasm")
            site.mkdir(parents=True, exist_ok=True)
            (site / "client.wasm").write_bytes(artifact.read_bytes())
            print(f"  site/client.wasm ({artifact.stat().st_size} bytes)")
        else:
            native_binaries.append(crate_dir / "target" / BUILD_PROFILE / crate)

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
    remove_stale_pages(site, {str(rel) for _, rel in asset_files})
    compose_assets(site, features)
    write_coverage(build_dir, features)

    print("build OK")
    if run and native_binaries:
        run_binary(native_binaries[0], build_dir)


def write_skillset(product: str, features: list):
    """Assemble the product's agent instructions — every included node's
    `<name>.agent.md`, verbatim, in the same provenance order the chains
    compose in — into products/<product>/build/skillset.md. The skillset is
    the tree's third language made loadable: untick a node and the builder
    forgets how to serve it; the file regenerates on every link, so editing
    it by hand is editing a build artifact."""
    build_dir = REPO / "products" / product / "build"
    out = build_dir / "skillset.md"
    parts = []
    for f in features:
        for src_rel, text in f.agent:
            parts.append(f"<!-- {node_path(f.rel)} — from {src_rel}; "
                         f"toggles with the node -->\n{text.rstrip()}\n")
    if not parts:
        if out.exists():
            out.unlink()
        return
    build_dir.mkdir(parents=True, exist_ok=True)
    head = (f"# {product} skillset\n"
            f"*assembled by fmlink from the feature tree, provenance order — "
            f"a build artifact: edit the nodes' `.agent.md` files, never "
            f"this file*\n\n")
    out.write_text(head + "\n".join(parts))
    print(f"skillset: {len(parts)} instruction fragment(s) -> "
          f"{out.relative_to(REPO)}")


def write_coverage(build_dir: Path, features: list):
    """The coverage record beside the build, for whoever wants it as data —
    today `tools/export_features.py`, which stamps it onto each node of the
    exported tree so the chooser can one day say what a tickbox reaches."""
    record = {node_path(f.rel): coverage_of(node_path(f.rel)) for f in features}
    (build_dir / "coverage.json").write_text(json.dumps(record, indent=1))


def remove_stale_pages(site: Path, copied: set):
    """Delete composition-target pages left over from a previous build whose
    owning feature is now excluded — their slot markers are already consumed
    and their presence isn't this composition's choice."""
    targets = {p for pages in FRAGMENT_PAGE.values() for p in pages}
    for page in sorted(targets - copied):
        if (site / page).exists():
            (site / page).unlink()
            print(f"  removed stale site/{page} (owner not in this composition)")


def js_patches(text: str) -> list:
    """The (object, method) pairs a fragment installs on ANOTHER node's object,
    in source order, deduped. An assignment to an object the fragment declares
    itself is that object's own definition — a chain's start, not a link — and
    Rust's rule for those is the same: the seam a chain hangs from is not
    gated."""
    own = set(JS_DEFINE_RE.findall(text))
    seen, out = set(), []
    for obj, method in JS_PATCH_RE.findall(text):
        if obj in own or (obj, method) in seen:
            continue
        seen.add((obj, method))
        out.append((obj, method))
    return out


def js_watch_block(path: str, pairs: list) -> str:
    """Emitted ABOVE a fragment: remember what each function it is about to
    replace looks like now, and make sure the observer that attributes new DOM
    to its maker is running. Generated JS names the objects lexically — a
    fragment-composed `const feature_X` is a script binding, not a window
    property — so every read is typeof-guarded against the node being absent
    from this composition."""
    lines = [f"// fm: fragment gate {path} (watch)",
             "if(!self.fmObeyMO){self.fmObeyMO=new MutationObserver(function(){});",
             "self.fmObeyMO.observe(document.documentElement,"
             "{childList:true,subtree:true});}",
             "self.fmObeyPrev=self.fmObeyPrev||{};"]
    for obj, method in pairs:
        key = f"{path}|{obj}.{method}"
        lines.append(f'self.fmObeyPrev[{json.dumps(key)}]='
                     f'(typeof {obj}!=="undefined")?{obj}.{method}:undefined;')
    return "\n".join(lines)


def js_gate_block(path: str, pairs: list) -> str:
    """Emitted BELOW a fragment: wrap each function it actually replaced, and
    claim the DOM it made. The wrapper is the page's `gate_line`: off, the
    previous link answers untouched; on, this link does. `self.fmOn` is the
    runtime's read, late-bound because it is composed last — while it is
    missing (during load) every gate is open, which is what load time is.

    A pair is wrapped only when the fragment REPLACED a function that was
    already there: a fragment that adds a new method to another node's object
    is starting a chain, not extending one."""
    lines = [f"// fm: fragment gate {path}"]
    for obj, method in pairs:
        key = json.dumps(f"{path}|{obj}.{method}")
        lines.append(
            f'if(typeof {obj}!=="undefined"){{const p=self.fmObeyPrev[{key}],'
            f'm={obj}.{method};'
            f'if(typeof p==="function"&&typeof m==="function"&&m!==p)'
            f'{obj}.{method}=function(){{return(self.fmOn&&'
            f'!self.fmOn({json.dumps(path)}))?p.apply(this,arguments)'
            f':m.apply(this,arguments);}};}}')
    # every element this fragment put in the page, including the ones inside
    # what it made: the mark travels with the element, so a later fragment that
    # re-parents somebody's button (and drops the row it came in) cannot take
    # that button out of its owner's reach. An element that is already claimed
    # keeps its first claimant — moving a thing is not making it.
    lines.append(
        'if(self.fmObeyMO){const fmClaim=function(n){'
        'if(n.nodeType!==1||n.getAttribute("data-fm-node"))return;'
        f'n.setAttribute("data-fm-node",{json.dumps(path)});'
        'for(const k of n.querySelectorAll("*"))'
        'if(!k.getAttribute("data-fm-node"))'
        f'k.setAttribute("data-fm-node",{json.dumps(path)});}};'
        'for(const r of self.fmObeyMO.takeRecords())'
        'for(const n of r.addedNodes)fmClaim(n);}')
    return "\n".join(lines)


def html_mark_roots(text: str, path: str, src: str) -> str:
    """Stamp every TOP-LEVEL element of a body fragment with its owning node,
    so the runtime can hide furniture it cannot delete. Nested elements are
    left alone — hiding a root takes its subtree with it, which is what an
    ancestor's untick means everywhere else in this system."""
    out, depth, i, marked = [], 0, 0, 0
    for m in re.finditer(r"<!--.*?-->|<(/?)([a-zA-Z][\w-]*)((?:[^>\"']|\"[^\"]*\"|'[^']*')*?)(/?)>",
                         text, re.S):
        if m.group(0).startswith("<!--"):
            continue
        closing, tag, attrs, selfclose = m.groups()
        if closing:
            depth = max(0, depth - 1)
            continue
        if depth == 0:
            out.append(text[i:m.start()])
            out.append(f'<{tag} data-fm-node="{path}"{attrs}'
                       f'{"/" if selfclose else ""}>')
            i = m.end()
            marked += 1
        if not selfclose and tag.lower() not in HTML_VOID:
            depth += 1
    out.append(text[i:])
    if not marked:
        fail(f"{src}: a body fragment must have at least one element at its "
             f"top level for the fragment gates to mark — this one has none")
    return "".join(out), marked


def compose_assets(site: Path, features: list):
    """Inject every included feature's page fragments at the slot markers of
    the page-owning assets, in linearisation order, provenance-commented.
    Toggling a feature off in order.md genuinely removes its fragments."""
    by_page = {}
    for feature in features:
        for fr in feature.fragments:
            by_page.setdefault(fr["file"], []).append(fr)
    # the fragment gates, if a composed node asked for them. The asking node's
    # own fragments are NOT gated: the runtime that answers "is this node on?"
    # cannot be a thing that stops running when a node is off, the same way the
    # Rust gates' read primitive lives beneath the Context rather than on it.
    gate_owner = next((fr["node"] for f in features for fr in f.fragments
                       if FRAGMENT_GATE_HOOK in fr["text"]), None)
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
                    gating = gate_owner is not None and page == "index.html"
                    for i in slot_items:
                        body = i["text"].rstrip()
                        if gating and slot == "script" and i["node"] != gate_owner:
                            pairs = js_patches(body)
                            coverage_note(i["node"], "fragment", len(pairs))
                            body = (js_watch_block(i["node"], pairs) + "\n"
                                    + body + "\n"
                                    + js_gate_block(i["node"], pairs))
                        (fdir / i["name"]).write_text(
                            SLOT_COMMENT[slot].format(i["src"]) + "\n"
                            + body + "\n")
                        mark = f' data-fm-node="{i["node"]}"' if gating else ""
                        if mark and slot == "style":
                            coverage_note(i["node"], "style", 1)
                        tags.append(
                            f'<script src="f/{i["name"]}"></script>'
                            if slot == "script"
                            else f'<link rel="stylesheet" href="f/{i["name"]}"'
                                 f'{mark}>')
                    close, reopen = (("</script>", "<script>") if slot == "script"
                                     else ("</style>", "<style>"))
                    text = text.replace(
                        marker, close + "\n" + "\n".join(tags) + "\n" + reopen)
                else:
                    parts = []
                    for i in slot_items:
                        body = i["text"].rstrip()
                        if (gate_owner is not None and page == "index.html"
                                and slot == "body"):
                            body, roots = html_mark_roots(body, i["node"],
                                                          i["name"])
                            coverage_note(i["node"], "body", roots)
                        parts.append(SLOT_COMMENT[slot].format(i["src"])
                                     + "\n" + body)
                    text = text.replace(marker, "\n".join(parts))
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
    ap.add_argument("--quick", action="store_true",
                    help="debug-profile build (fast; proof cycles, never deploy)")
    ap.add_argument("--chains", action="store_true",
                    help="print chain topology and exit (no build)")
    ap.add_argument("--coverage", action="store_true",
                    help="after linking, print what each node's tickbox gates "
                         "(pair with --quick for a fast look)")
    args = ap.parse_args()

    if args.quick:
        global BUILD_PROFILE
        BUILD_PROFILE = "debug"
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
    write_skillset(args.product, features)
    places = read_places(product_dir)
    if places is None:
        build_legacy(args.product, base, chains, args.run)
    else:
        build_places(args.product, places, base, chains, features, args.run)
    if args.coverage:
        coverage_table(features)


if __name__ == "__main__":
    main()
