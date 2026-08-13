#!/usr/bin/env python3
"""Export a Claude Code session log (JSONL) to an fm transcript (markdown).

Usage:
    export_transcript.py [--session PATH] [--out DIR] [--slug SLUG] [--title TITLE]

Defaults: the most recently modified session in this project's Claude log
directory, written to <repo>/transcripts/<date>-<slug>.md.

Transcript format (the contract feature specs rely on):
  - each user prompt gets a stable anchor heading `### pN` (append-only, so
    anchors never move); its timestamp follows in italics on the next line
  - the prompt text is quoted verbatim as a blockquote
  - assistant replies follow as plain prose (text blocks only — thinking and
    tool traffic are omitted)
  - feature specs reference prompts as: transcripts/<file>.md#pN
  - the prompt timestamp is the creation time used by ordering fallback

Re-running regenerates the file deterministically; since prompts are only
ever appended to a session, existing anchors are stable across re-exports.
"""

import argparse
import json
import re
from datetime import datetime
from pathlib import Path

PROJECT_LOG_DIR = Path.home() / ".claude/projects/-Users-asnaroo-Desktop-experiments"
REPO_ROOT = Path(__file__).resolve().parent.parent

# user-message content that is command noise, not a prompt
NOISE_PREFIXES = (
    "<local-command-caveat>",
    "<command-name>",
    "<local-command-stdout>",
    "<system-reminder>",
)


def latest_session(log_dir: Path) -> Path:
    sessions = sorted(log_dir.glob("*.jsonl"), key=lambda p: p.stat().st_mtime)
    if not sessions:
        raise SystemExit(f"no session logs found in {log_dir}")
    return sessions[-1]


def strip_system_reminders(text: str) -> str:
    return re.sub(r"<system-reminder>.*?</system-reminder>", "", text, flags=re.S).strip()


def is_prompt(entry: dict) -> bool:
    """True if this log entry is a real user prompt (not meta/tool/command noise)."""
    if entry.get("type") != "user" or entry.get("isMeta") or entry.get("isSidechain"):
        return False
    content = entry.get("message", {}).get("content")
    if not isinstance(content, str):
        return False  # tool results arrive as content lists
    return bool(content.strip()) and not content.strip().startswith(NOISE_PREFIXES)


def assistant_texts(entry: dict) -> list[str]:
    """Visible text blocks of an assistant entry (no thinking, no tool calls)."""
    if entry.get("type") != "assistant" or entry.get("isSidechain"):
        return []
    content = entry.get("message", {}).get("content") or []
    if isinstance(content, str):
        return [content] if content.strip() else []
    return [c["text"] for c in content if c.get("type") == "text" and c.get("text", "").strip()]


def fmt_time(iso: str) -> str:
    return datetime.fromisoformat(iso.replace("Z", "+00:00")).strftime("%Y-%m-%d %H:%M")


def export(session_path: Path, out_path: Path, title: str) -> int:
    entries = [json.loads(line) for line in session_path.read_text().splitlines() if line.strip()]

    lines = [
        f"# transcript: {title}",
        f"*session `{session_path.stem}`, exported by tools/export_transcript.py — do not edit; anchors `#pN` are stable*",
        "",
    ]
    prompt_n = 0
    for entry in entries:
        if is_prompt(entry):
            text = strip_system_reminders(entry["message"]["content"])
            if not text:
                continue
            prompt_n += 1
            lines += [f"### p{prompt_n}", f"*{fmt_time(entry['timestamp'])}*", ""]
            lines += [f"> {line}" for line in text.splitlines()]
            lines.append("")
        else:
            for text in assistant_texts(entry):
                lines += [text, ""]

    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text("\n".join(lines))
    return prompt_n


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--session", type=Path, help="session .jsonl (default: most recent)")
    ap.add_argument("--out", type=Path, default=REPO_ROOT / "transcripts", help="output directory")
    ap.add_argument("--slug", default="session", help="filename slug")
    ap.add_argument("--title", default=None, help="transcript title (default: slug)")
    args = ap.parse_args()

    session = args.session or latest_session(PROJECT_LOG_DIR)
    date = datetime.fromtimestamp(session.stat().st_mtime).strftime("%Y-%m-%d")
    out_path = args.out / f"{date}-{args.slug}.md"
    n = export(session, out_path, args.title or args.slug)
    print(f"wrote {out_path} ({n} prompts)")


if __name__ == "__main__":
    main()
