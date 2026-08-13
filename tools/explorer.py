#!/usr/bin/env python3
"""fm explorer — server-rendered three-pane feature browser for an fm repo.

Left: the feature tree (nodes in order.md order; order.md itself is hidden;
unticked features shown dimmed). Center: the selected feature's spec rendered,
with its .rs implementation(s) below. Right: the conversation transcript,
auto-opened at the feature's provenance prompt (the tree links carry the #pN
fragment, so the browser scrolls the pane natively — no client JS).

Every page is plain HTML rendered server-side, so agents can curl it.

  /                       → redirects to the first feature
  /feature/<path>         three panes: tree | spec + code | transcript
  /feature/<path>?t=<transcript>   override the transcript shown
  /view/<repo-path>       render any repo file in the center pane (agents/debug)
  /raw/<repo-path>        file source as text/plain

Usage: explorer.py [--port 8123]. Serves the repo rooted one level above tools/.
"""

import argparse
import html
import re
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from urllib.parse import parse_qs, quote, unquote, urlparse

REPO = Path(__file__).resolve().parent.parent
FEATURES = REPO / "features"


# ---------------------------------------------------------------- markdown

def slug(text: str) -> str:
    return re.sub(r"[^\w]+", "-", text.strip().lower()).strip("-")


GLOSSARY = {}       # term slug -> (term, defining feature path, definition text)
FEATURE_PATHS = {}  # feature name / full path -> (feature path, one-line description)


def plain_inline(text: str) -> str:
    """Minimal inline rendering for popup bodies: escape + backtick code spans
    (no term resolution, so definitions can't nest popups)."""
    text = html.escape(text, quote=False)
    return re.sub(r"`([^`]+)`", r"<code>\1</code>", text)


def popup_html(link: str, title: str, source_href: str, source_label: str,
               body: str) -> str:
    """A term link with a hover/focus popup card (pure CSS, no JS)."""
    card = (f'<span class="popbox"><b>{title}</b> · '
            f'<a href="{source_href}">{source_label}</a><br>{plain_inline(body)}</span>')
    return f'<span class="tw">{link}<span class="pop">{card}</span></span>'


def term_link(name: str):
    """Resolve a backticked /term: glossary definition first, then feature page.
    Both get a hover popup showing the definition/description in place."""
    s = slug(name)
    if s in GLOSSARY:
        term, fpath, definition = GLOSSARY[s]
        href = f"/feature/{quote(fpath)}#term-{s}"
        link = f'<a class="term" href="{href}">{name}</a>'
        return popup_html(link, html.escape(term), href, html.escape(fpath),
                          definition)
    if name in FEATURE_PATHS:
        fpath, desc = FEATURE_PATHS[name]
        href = f"/feature/{quote(fpath)}"
        link = f'<a class="term" href="{href}">{name}</a>'
        if desc:
            return popup_html(link, html.escape(fpath), href, "open", desc)
        return link
    return None


def render_inline(text: str, rewrite) -> str:
    text = html.escape(text, quote=False)
    out = []
    for part in re.split(r"(`[^`]+`)", text):
        if len(part) > 1 and part.startswith("`") and part.endswith("`"):
            inner = part[1:-1]
            if inner.startswith("/") and len(inner) > 1:
                link = term_link(inner[1:])
                if link:
                    out.append(link)
                    continue
            out.append(f"<code>{inner}</code>")
            continue
        part = re.sub(r"\[([^\]]+)\]\(([^)\s]+)\)",
                      lambda m: f'<a href="{rewrite(m.group(2))}">{m.group(1)}</a>', part)
        # bare transcript refs (fm spec provenance lines) become links too
        part = re.sub(r"(?<!\")(transcripts/[\w./-]+\.md(?:#p\d+)?)",
                      lambda m: f'<a href="{rewrite(m.group(1))}">{m.group(1)}</a>', part)
        part = re.sub(r"\*\*([^*]+)\*\*", r"<b>\1</b>", part)
        part = re.sub(r"(?<!\*)\*([^*]+)\*(?!\*)", r"<i>\1</i>", part)
        part = re.sub(r"~~([^~]+)~~", r"<s>\1</s>", part)
        out.append(part)
    return "".join(out)


def md_to_html(text: str, rewrite=lambda href: href) -> str:
    """Small renderer for the markdown subset fm documents use."""
    lines = text.splitlines()
    out, para, quote_buf, list_stack = [], [], [], []

    def flush_para():
        if para:
            out.append(f"<p>{render_inline(' '.join(para), rewrite)}</p>")
            para.clear()

    def flush_quote():
        if quote_buf:
            body = "<br>".join(render_inline(l, rewrite) for l in quote_buf)
            out.append(f"<blockquote>{body}</blockquote>")
            quote_buf.clear()

    def close_lists(to_indent=-1):
        while list_stack and list_stack[-1][0] >= to_indent:
            out.append(f"</{list_stack.pop()[1]}>")

    def flush_all():
        flush_para(); flush_quote(); close_lists(0)

    i = 0
    while i < len(lines):
        line = lines[i]

        if line.lstrip().startswith("```"):                       # fenced code
            flush_all()
            block, i = [], i + 1
            while i < len(lines) and not lines[i].lstrip().startswith("```"):
                block.append(lines[i]); i += 1
            out.append(f"<pre>{html.escape(chr(10).join(block))}</pre>")
            i += 1
            continue

        m = re.match(r"^(\s*)([-*]|\d+\.)\s+(.*)$", line)
        is_code_indent = line.startswith("    ") and line.strip() and not (m and list_stack)

        if is_code_indent:                                        # indented code
            flush_all()
            block = []
            while i < len(lines) and (lines[i].startswith("    ") or not lines[i].strip()):
                if not lines[i].strip() and not (i + 1 < len(lines) and
                        (lines[i + 1].startswith("    ") or not lines[i + 1].strip())):
                    break
                block.append(lines[i][4:]); i += 1
            while block and not block[-1].strip():
                block.pop()
            out.append(f"<pre>{html.escape(chr(10).join(block))}</pre>")
            continue

        h = re.match(r"^(#{1,6})\s+(.*)$", line)
        if h:                                                     # heading
            flush_all()
            level, title = len(h.group(1)), h.group(2)
            out.append(f'<h{level} id="{slug(title)}">{render_inline(title, rewrite)}</h{level}>')
        elif re.match(r"^\s*>\s?", line):                         # blockquote
            flush_para(); close_lists(0)
            quote_buf.append(re.sub(r"^\s*>\s?", "", line))
        elif re.match(r"^-{3,}\s*$", line):                       # hr
            flush_all(); out.append("<hr>")
        elif m:                                                   # list item
            flush_para(); flush_quote()
            indent, marker, content = len(m.group(1)), m.group(2), m.group(3)
            tag = "ol" if marker[0].isdigit() else "ul"
            close_lists(indent + 1)
            if not list_stack or list_stack[-1][0] < indent or list_stack[-1][1] != tag:
                if list_stack and list_stack[-1][1] != tag and list_stack[-1][0] == indent:
                    out.append(f"</{list_stack.pop()[1]}>")
                list_stack.append((indent, tag)); out.append(f"<{tag}>")
            # glossary definition bullets (- **term**: ...) get anchor ids
            li_id = ""
            dm = re.match(r"^\*\*(.+?)\*\*\s*:", content)
            if dm:
                li_id = f' id="term-{slug(dm.group(1))}"'
            task = re.match(r"^\[( |x)\]\s+(.*)$", content)
            if task:
                checked = " checked" if task.group(1) == "x" else ""
                content = (f'<input type="checkbox" disabled{checked}> '
                           + render_inline(task.group(2), rewrite))
            else:
                content = render_inline(content, rewrite)
            out.append(f"<li{li_id}>{content}</li>")
        elif not line.strip():                                    # blank
            flush_para(); flush_quote(); close_lists(0)
        else:                                                     # paragraph
            flush_quote()
            para.append(line.strip())
        i += 1

    flush_all()
    return "\n".join(out)


# ---------------------------------------------------------------- feature tree

class Feature:
    """A feature node: spec, implementations, ordered children."""

    def __init__(self, directory: Path, included=True):
        self.dir = directory
        self.name = directory.name
        self.path = str(directory.relative_to(FEATURES))  # e.g. "hello/goodbye"
        self.included = included
        self.spec = directory / f"{self.name}.md"
        self.rs_files = sorted(directory.glob("*.rs"))
        self.children = []

    @property
    def provenance(self):
        """(transcript_rel_path, '#pN' or '') parsed from the spec's reference."""
        if not self.spec.exists():
            return None, ""
        m = re.search(r"\((transcripts/[^#)\s]+)(#p\d+)?\)", self.spec.read_text())
        return (m.group(1), m.group(2) or "") if m else (None, "")


def read_order(directory: Path):
    f = directory / "order.md"
    if not f.exists():
        return None
    entries = []
    for line in f.read_text().splitlines():
        m = re.match(r"-\s*\[( |x)\]\s*(\S+)", line.strip())
        if m:
            entries.append((m.group(2), m.group(1) == "x"))
    return entries


def load_children(directory: Path) -> list:
    """Child features in order.md order; unlisted folders appended at the end."""
    subs = {p.name: p for p in directory.iterdir() if p.is_dir()}
    order = read_order(directory) or [(name, True) for name in sorted(subs)]
    children = []
    listed = set()
    for name, included in order:
        listed.add(name)
        if name in subs:
            child = Feature(subs[name], included)
            child.children = load_children(child.dir)
            children.append(child)
    for name in sorted(set(subs) - listed):
        child = Feature(subs[name], True)
        child.children = load_children(child.dir)
        children.append(child)
    return children


def find_feature(root_children: list, path: str):
    for feature in root_children:
        if feature.path == path:
            return feature
        found = find_feature(feature.children, path)
        if found:
            return found
    return None


def first_feature(root_children: list):
    return root_children[0] if root_children else None


def build_indexes(roots: list):
    """Populate GLOSSARY (from every spec's ## glossary section) and FEATURE_PATHS."""
    GLOSSARY.clear()
    FEATURE_PATHS.clear()

    def walk(children):
        for feature in children:
            desc = ""
            if feature.spec.exists():
                text = feature.spec.read_text(errors="replace")
                dm = re.search(r"^\*(.+)\*\s*$", text, re.M)
                desc = dm.group(1) if dm else ""
                gm = re.search(r"^##\s*glossary\s*$(.*?)(?=^##\s|\Z)", text,
                               re.S | re.M)
                if gm:
                    for tm in re.finditer(r"-\s*\*\*(.+?)\*\*\s*:\s*(.+)",
                                          gm.group(1)):
                        GLOSSARY[slug(tm.group(1))] = (tm.group(1), feature.path,
                                                       tm.group(2).strip())
            FEATURE_PATHS[feature.name] = (feature.path, desc)
            FEATURE_PATHS[feature.path] = (feature.path, desc)
            walk(feature.children)

    walk(roots)


def all_transcripts() -> list:
    tdir = REPO / "transcripts"
    if not tdir.is_dir():
        return []
    return sorted(str(p.relative_to(REPO)) for p in tdir.glob("*.md"))


# ---------------------------------------------------------------- panes

def tree_html(children: list, current: str) -> str:
    if not children:
        return ""
    rows = ["<ul>"]
    for feature in children:
        transcript, anchor = feature.provenance
        href = f"/feature/{quote(feature.path)}{anchor}"
        classes = "feature"
        if feature.path == current:
            classes += " sel"
        if not feature.included:
            classes += " off"
        link = f'<a class="{classes}" href="{href}">{html.escape(feature.name)}</a>'
        if feature.children:
            # expanded only along the path to (and including) the current feature
            on_path = current == feature.path or current.startswith(feature.path + "/")
            rows.append(f'<li><details{" open" if on_path else ""}>'
                        f"<summary>{link}</summary>"
                        + tree_html(feature.children, current)
                        + "</details></li>")
        else:
            rows.append(f'<li class="leaf">{link}</li>')
    rows.append("</ul>")
    return "\n".join(rows)


def make_rewrite(base_url: str, shown_transcript: str):
    """Rewrite links in rendered markdown. Refs to the transcript already shown
    in the right pane become bare fragments (native scroll); other transcript
    refs reload the page with ?t=; other repo links go to /view/."""
    def rewrite(href: str) -> str:
        if href.startswith(("http://", "https://", "#", "mailto:")):
            return href
        m = re.match(r"(?:\./)?(transcripts/[^#]+)(#.*)?$", href)
        if m:
            target, fragment = m.group(1), m.group(2) or ""
            if target == shown_transcript:
                return fragment or "#"
            return f"{base_url}?t={quote(target)}{fragment}"
        return "/view/" + quote(href.lstrip("./"))
    return rewrite


def feature_center_html(feature: Feature, rewrite) -> str:
    parts = []
    if feature.spec.exists():
        parts.append(md_to_html(feature.spec.read_text(errors="replace"), rewrite))
    else:
        parts.append(f'<p class="placeholder">no spec ({feature.name}.md missing)</p>')
    for rs in feature.rs_files:
        parts.append(f'<h2 class="filename">{html.escape(rs.name)}</h2>')
        parts.append(f'<pre class="raw">{html.escape(rs.read_text(errors="replace"))}</pre>')
    return "\n".join(parts)


STYLE = """
:root { --bg:#1a1d21; --panel:#22262b; --border:#33383f; --text:#d6dae0;
        --dim:#8b929b; --accent:#6fb3ff; --hover:#2c3138; --code:#16181c; }
* { box-sizing:border-box; margin:0; }
body { background:var(--bg); color:var(--text); height:100vh; overflow:hidden;
  font:14px/1.55 -apple-system,"Segoe UI",sans-serif;
  display:grid; grid-template-columns:220px 1fr 400px; }
.pane { overflow-y:auto; padding:14px 18px; border-right:1px solid var(--border); }
.pane:last-child { border-right:none; background:var(--panel); }
.pane > h1.label { font-size:11px; text-transform:uppercase; letter-spacing:.08em;
  color:var(--dim); margin:0 0 10px; font-weight:600; }
#tree ul { list-style:none; padding-left:16px; }
#tree > ul { padding-left:0; }
#tree summary { list-style:none; cursor:pointer; display:flex; align-items:center; }
#tree summary::before { content:"\\25B8"; color:var(--dim); font-size:16px;
  width:24px; height:24px; flex:none; display:flex; align-items:center;
  justify-content:center; border-radius:5px; }
#tree summary:hover::before { background:var(--hover); color:var(--text); }
#tree details[open] > summary::before { content:"\\25BE"; }
#tree li.leaf { padding-left:24px; }
#tree a.feature { display:block; flex:1; color:var(--text); text-decoration:none;
  padding:2px 8px; border-radius:5px; white-space:nowrap;
  overflow:hidden; text-overflow:ellipsis; }
#tree a.feature:hover { background:var(--hover); }
#tree a.feature.sel { background:var(--hover); color:var(--accent); }
#tree a.feature.off { color:var(--dim); text-decoration:line-through; }
.md { max-width:760px; }
.md h1 { font-size:25px; margin:0 0 8px; color:#fff; }
.md h2 { font-size:19px; margin:22px 0 8px; color:#fff; }
.md h2.filename { font-size:13px; color:var(--dim); font-weight:600;
  font-family:"SF Mono",Menlo,monospace; margin:26px 0 6px;
  border-top:1px solid var(--border); padding-top:14px; }
.md h3 { font-size:15px; margin:18px 0 6px; color:var(--accent); }
.md h4 { font-size:14px; margin:14px 0 6px; color:var(--accent); }
.md p, .md ul, .md ol { margin:0 0 10px; }
.md ul, .md ol { padding-left:22px; }
.md li { margin:2px 0; }
.md li > ul, .md li > ol { margin:0; }
.md blockquote { border-left:3px solid var(--accent); padding:4px 12px;
  margin:0 0 10px; background:var(--hover); border-radius:0 6px 6px 0; }
.md code { background:var(--code); padding:1px 5px; border-radius:4px;
  font:12.5px/1.5 "SF Mono",Menlo,monospace; color:#a8d3ff; }
.md pre { background:var(--code); padding:12px; border-radius:8px; overflow-x:auto;
  margin:0 0 12px; border:1px solid var(--border);
  font:12.5px/1.6 "SF Mono",Menlo,monospace; color:#c9e3ff; }
.md a { color:var(--accent); text-decoration:none; }
.md a:hover { text-decoration:underline; }
.md a.term { color:var(--accent); border-bottom:1px dotted var(--accent); }
.md a.term:hover { text-decoration:none; border-bottom-style:solid; }
.md .tw { position:relative; }
.md .tw .pop { visibility:hidden; opacity:0; transition:opacity .12s ease .18s,
  visibility 0s linear .18s; position:absolute; left:0; top:100%;
  padding-top:6px; z-index:10; width:320px; }
.md .tw:hover .pop, .md .tw:focus-within .pop { visibility:visible; opacity:1; }
.md .popbox { display:block; background:var(--panel); border:1px solid var(--border);
  border-radius:8px; padding:9px 12px; box-shadow:0 6px 20px rgba(0,0,0,.45);
  font-size:13px; line-height:1.5; color:var(--text); }
.md .popbox b { color:var(--accent); }
.md i { color:var(--dim); }
.md hr { border:none; border-top:1px solid var(--border); margin:16px 0; }
.md input[type=checkbox] { accent-color:var(--accent); }
.md :target { background:var(--hover); border-radius:6px;
  outline:6px solid var(--hover); }
pre.raw { background:var(--code); padding:12px; border-radius:8px; overflow-x:auto;
  font:12.5px/1.6 "SF Mono",Menlo,monospace; color:#c9e3ff;
  border:1px solid var(--border); }
p.placeholder { color:var(--dim); font-style:italic; }
"""

PAGE = """<!doctype html>
<html><head><meta charset="utf-8"><title>fm: {title}</title>
<style>{style}</style></head>
<body>
<div class="pane" id="left"><h1 class="label">features</h1><div id="tree">{tree}</div></div>
<div class="pane" id="center"><div class="md">{center}</div></div>
<div class="pane" id="right"><h1 class="label">transcript · {tname}</h1>
<div class="md">{transcript}</div></div>
</body></html>"""


def build_page(title, tree, center, transcript_path, rewrite_base) -> str:
    if transcript_path and (REPO / transcript_path).is_file():
        tbody = md_to_html((REPO / transcript_path).read_text(errors="replace"),
                           make_rewrite(rewrite_base, transcript_path))
        tname = transcript_path.split("/")[-1]
    else:
        tbody, tname = '<p class="placeholder">no transcripts yet</p>', "none"
    return PAGE.format(title=html.escape(title), style=STYLE, tree=tree,
                       center=center, transcript=tbody, tname=html.escape(tname))


def render_feature_page(fpath: str, transcript_override: str) -> str:
    roots = load_children(FEATURES)
    build_indexes(roots)
    feature = find_feature(roots, fpath)
    tree = tree_html(roots, fpath)
    base_url = f"/feature/{quote(fpath)}"
    if not feature:
        return build_page(fpath, tree,
                          f'<p class="placeholder">feature {html.escape(fpath)} not found</p>',
                          latest_transcript(), base_url)
    prov_transcript, _ = feature.provenance
    transcript = transcript_override or prov_transcript or latest_transcript()
    center = feature_center_html(feature, make_rewrite(base_url, transcript))
    return build_page(feature.path, tree, center, transcript, base_url)


def render_file_page(rel: str, transcript_override: str) -> str:
    roots = load_children(FEATURES)
    build_indexes(roots)
    tree = tree_html(roots, "")
    base_url = f"/view/{quote(rel)}"
    transcript = transcript_override or latest_transcript()
    target = REPO / rel
    if target.is_file():
        text = target.read_text(errors="replace")
        center = (md_to_html(text, make_rewrite(base_url, transcript))
                  if rel.endswith(".md")
                  else f'<pre class="raw">{html.escape(text)}</pre>')
    else:
        center = f'<p class="placeholder">{html.escape(rel)} not found</p>'
    return build_page(rel, tree, center, transcript, base_url)


def latest_transcript() -> str:
    transcripts = all_transcripts()
    return transcripts[-1] if transcripts else ""


# ---------------------------------------------------------------- server

def safe_rel(rel: str):
    target = (REPO / rel).resolve()
    if target == REPO or REPO in target.parents:
        return target
    return None


class Handler(BaseHTTPRequestHandler):
    def do_GET(self):
        url = urlparse(self.path)
        transcript = unquote(parse_qs(url.query).get("t", [""])[0])
        if transcript and safe_rel(transcript) is None:
            self.reply(404, b"not found", "text/plain")
            return

        if url.path in ("/", "/feature", "/feature/"):
            first = first_feature(load_children(FEATURES))
            self.redirect(f"/feature/{quote(first.path)}" if first else "/view/fm.md")
        elif url.path.startswith("/feature/"):
            fpath = unquote(url.path[len("/feature/"):]).strip("/")
            self.reply(200, render_feature_page(fpath, transcript).encode(),
                       "text/html; charset=utf-8")
        elif url.path.startswith("/view/"):
            rel = unquote(url.path[len("/view/"):])
            if safe_rel(rel) is None:
                self.reply(404, b"not found", "text/plain")
                return
            self.reply(200, render_file_page(rel, transcript).encode(),
                       "text/html; charset=utf-8")
        elif url.path.startswith("/raw/"):
            target = safe_rel(unquote(url.path[len("/raw/"):]))
            if target and target.is_file():
                self.reply(200, target.read_bytes(), "text/plain; charset=utf-8")
            else:
                self.reply(404, b"not found", "text/plain")
        else:
            self.reply(404, b"not found", "text/plain")

    def redirect(self, location: str):
        self.send_response(302)
        self.send_header("Location", location)
        self.end_headers()

    def reply(self, code: int, body: bytes, ctype: str):
        self.send_response(code)
        self.send_header("Content-Type", ctype)
        self.send_header("Cache-Control", "no-store")
        self.end_headers()
        self.wfile.write(body)

    def log_message(self, *args):
        pass


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--port", type=int, default=8123)
    args = ap.parse_args()
    server = ThreadingHTTPServer(("localhost", args.port), Handler)
    print(f"fm explorer: http://localhost:{args.port}  (serving {REPO})")
    server.serve_forever()


if __name__ == "__main__":
    main()
