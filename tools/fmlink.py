#!/usr/bin/env python3
"""fm linker v0 — composes a product from the feature tree.

Pipeline:
  1. walk features/, linearising depth-first with sibling order (and static
     include/exclude) taken from each node's order.md checklist
  2. parse each feature's .rs files: feature_X impls (functions) and plain
     structs (fields)
  3. chain same-named functions in linearisation order; rewrite existing.fn()
     to the previous definition in the chain
  4. flat-merge same-named structs; duplicate field = link error
  5. emit a cargo project under products/<name>/build/ and run cargo build,
     mapping rustc diagnostics back to feature-source file:line

Usage: fmlink.py [product] [--run]      (product defaults to "demo")
"""

import argparse
import json
import re
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
FEATURES = REPO / "features"


def fail(msg: str):
    sys.exit(f"fm link error: {msg}")


# ------------------------------------------------------------------ tree walk

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


SKIP_DIRS = {"build", "target"}


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


# ------------------------------------------------------------------ rust parse

def match_brace(text: str, open_idx: int) -> int:
    """Index of the '}' closing the '{' at open_idx."""
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


class FeatureCode:
    """Everything one feature contributes: functions and struct fields."""

    def __init__(self, feature_dir: Path):
        self.dir = feature_dir
        # resolve symlinks so diagnostics point at the real source location
        self.rel = str(feature_dir.resolve().relative_to(REPO))
        self.name = None          # e.g. "Hello" from struct feature_Hello
        self.fns = []             # (fn_name, src_file, first_line, [lines])
        self.structs = []         # (struct_name, [(field, type, src_file, line)])
        for rs in sorted(feature_dir.glob("*.rs")):
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
            first, last = line_of(text, fn_start), line_of(text, body_close)
            self.fns.append((fm.group(1), src, first, lines[first - 1:last]))
            pos = body_close + 1


# ------------------------------------------------------------------ compose

class Emitter:
    """Accumulates generated lines with a per-line map back to feature source."""

    def __init__(self):
        self.lines = []   # text
        self.map = []     # (src_file, src_line) or None

    def emit(self, text: str, src=None, line=None):
        self.lines.append(text)
        self.map.append((src, line) if src else None)


def compose(features: list) -> Emitter:
    out = Emitter()
    out.emit("// generated by fm linker v0 — do not edit; edit features/ instead")
    out.emit("#![allow(non_camel_case_types, dead_code, non_snake_case, unused)]")
    out.emit("")

    # flat-merge structs, checking field collisions
    merged = {}       # struct name -> [(field, type, src, line)]
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

    # chain functions: head[fn_name] = feature struct name currently outermost
    head = {}
    for feature in features:
        if not feature.fns:
            continue
        if not feature.name:
            fail(f"{feature.rel} defines functions but no feature_ struct")
        out.emit(f"// ---- feature: {feature.rel}")
        out.emit(f"struct feature_{feature.name};")
        out.emit(f"impl feature_{feature.name} {{")
        for fn_name, src, first_line, fn_lines in feature.fns:
            for offset, text in enumerate(fn_lines):
                rewritten = rewrite_existing(text, fn_name, head, feature)
                out.emit(rewritten, src, first_line + offset)
        out.emit("}")
        out.emit("")
        for fn_name, *_ in feature.fns:
            head[fn_name] = feature.name

    if "main" not in head:
        fail("no feature defines main — nothing to run")
    out.emit("// ---- entry point (outermost definition in the chain)")
    out.emit(f"fn main() {{ feature_{head['main']}::main(); }}")
    return out


def rewrite_existing(text: str, fn_name: str, head: dict, feature) -> str:
    """Rewrite existing.fn( -> feature_Prev::fn( using chain state before this feature."""
    def sub(m):
        called = m.group(1)
        if called not in head:
            fail(f"{feature.rel}: existing.{called}() but no earlier feature defines {called}")
        return f"feature_{head[called]}::{called}("
    return re.sub(r"existing\s*\.\s*(\w+)\s*\(", sub, text)


# ------------------------------------------------------------------ build

CARGO_TOML = """[package]
name = "{name}"
version = "0.0.1"
edition = "2021"

[[bin]]
name = "{name}"
path = "src/main.rs"
"""


def build(product: str, emitter: Emitter, run: bool):
    build_dir = REPO / "products" / product / "build"
    (build_dir / "src").mkdir(parents=True, exist_ok=True)
    (build_dir / "Cargo.toml").write_text(CARGO_TOML.format(name=product))
    (build_dir / "src" / "main.rs").write_text("\n".join(emitter.lines) + "\n")
    print(f"emitted {build_dir.relative_to(REPO)}/src/main.rs "
          f"({len(emitter.lines)} lines)")

    result = subprocess.run(["cargo", "build", "--message-format=json"],
                            cwd=build_dir, capture_output=True, text=True)
    report_diagnostics(result.stdout, emitter)
    if result.returncode != 0:
        sys.exit("build FAILED")
    print("build OK")

    if run:
        binary = build_dir / "target" / "debug" / product
        print(f"running {binary.relative_to(REPO)}:")
        run_result = subprocess.run([binary], capture_output=True, text=True)
        print(run_result.stdout, end="")
        if run_result.returncode != 0:
            sys.exit(f"run FAILED ({run_result.returncode}): {run_result.stderr}")


def report_diagnostics(cargo_json: str, emitter: Emitter):
    """Print rustc errors/warnings with generated lines mapped to feature source."""
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
                         else f"build/src/main.rs:{gen_line}")
                break
        print(f"  {m['level']}: {where}: {m['message']}")


# ------------------------------------------------------------------ main

def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("product", nargs="?", default="demo")
    ap.add_argument("--run", action="store_true", help="run the binary after building")
    args = ap.parse_args()

    product_dir = REPO / "products" / args.product
    if not product_dir.is_dir():
        fail(f"no such product: products/{args.product}")
    if not (product_dir / "order.md").exists():
        fail(f"products/{args.product} has no order.md — a product is a feature "
             f"tree (symlinks into features/, or local overrides) plus order.md")

    feature_dirs, excluded = [], []
    linearise(product_dir, feature_dirs, excluded, product_dir)
    print("linearisation:", " → ".join(str(d.relative_to(product_dir)) for d in feature_dirs))
    for ex in excluded:
        print(f"  excluded (order.md unticked): {ex}")

    features = [FeatureCode(d) for d in feature_dirs]
    build(args.product, compose(features), args.run)


if __name__ == "__main__":
    main()
