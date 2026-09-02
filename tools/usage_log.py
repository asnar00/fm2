#!/usr/bin/env python3
"""usage_log.py — sample Claude Code's plan usage and say how long the Fable
budget lasts.

Three modes:

    usage_log.py            sample once, append to the log, print one line
    usage_log.py --report   the session-start report (samples first, then
                            reads the log and estimates days remaining)
    usage_log.py --history  print the log's samples, newest last

The sample comes from the same endpoint /usage in Claude Code reads, with
the OAuth token Claude Code keeps in ~/.claude/.credentials.json. Nothing
from the token is logged. The log is ~/.claude/usage-log.jsonl, one JSON
object per line; it lives outside the repo because it is per-machine.

The Fable number is the weekly limit whose scope names the Fable model
(`limits[kind=weekly_scoped]`). It resets weekly; the report works out the
burn rate inside the current window and says whether the budget runs out
before the reset, and on which day if so.

An hourly launchd job (tools/com.noob.usagelog.plist) keeps the time series
honest between sessions. When Claude Code is not running the token can
expire and the sample fails; that is fine, because usage does not move
while nothing is running — the sampler logs the failure and moves on.
"""
import argparse
import datetime as dt
import json
import os
import sys
import urllib.error
import urllib.request

CREDS = os.path.expanduser("~/.claude/.credentials.json")
LOG = os.path.expanduser("~/.claude/usage-log.jsonl")
URL = "https://api.anthropic.com/api/oauth/usage"
WEEK = dt.timedelta(days=7)


def now():
    return dt.datetime.now(dt.timezone.utc)


def parse_ts(s):
    return dt.datetime.fromisoformat(s.replace("Z", "+00:00"))


def fetch():
    """One call to the usage endpoint. Returns the decoded JSON or raises."""
    with open(CREDS) as f:
        tok = json.load(f)["claudeAiOauth"]["accessToken"]
    req = urllib.request.Request(URL, headers={
        "Authorization": "Bearer " + tok,
        "anthropic-beta": "oauth-2025-04-20",
        "User-Agent": "fm2-usage-log/1",
    })
    with urllib.request.urlopen(req, timeout=20) as r:
        return json.load(r)


def distil(raw):
    """The few numbers we keep from a raw usage response."""
    out = {"ts": now().isoformat(timespec="seconds")}
    for lim in raw.get("limits", []):
        kind = lim.get("kind")
        scope = (lim.get("scope") or {}).get("model") or {}
        if kind == "session":
            out["session_pct"] = lim.get("percent")
            out["session_resets_at"] = lim.get("resets_at")
        elif kind == "weekly_all":
            out["weekly_pct"] = lim.get("percent")
            out["weekly_resets_at"] = lim.get("resets_at")
        elif kind == "weekly_scoped":
            name = (scope.get("display_name") or "scoped").lower()
            out[name + "_pct"] = lim.get("percent")
            out[name + "_resets_at"] = lim.get("resets_at")
    return out


def sample(quiet=False):
    """Fetch, append to the log, return the sample (or a failure record)."""
    try:
        rec = distil(fetch())
    except Exception as e:  # expired token, no network, rate limit
        rec = {"ts": now().isoformat(timespec="seconds"),
               "error": type(e).__name__ + ": " + str(e)[:120]}
    os.makedirs(os.path.dirname(LOG), exist_ok=True)
    with open(LOG, "a") as f:
        f.write(json.dumps(rec) + "\n")
    if not quiet:
        print(one_line(rec))
    return rec


def one_line(rec):
    if "error" in rec:
        return f"{rec['ts']}  sample failed: {rec['error']}"
    return (f"{rec['ts']}  fable {rec.get('fable_pct', '?')}%  "
            f"weekly {rec.get('weekly_pct', '?')}%  "
            f"session {rec.get('session_pct', '?')}%")


def load_log():
    if not os.path.exists(LOG):
        return []
    out = []
    with open(LOG) as f:
        for line in f:
            line = line.strip()
            if line:
                try:
                    out.append(json.loads(line))
                except json.JSONDecodeError:
                    pass
    return out


def good(samples, key):
    return [s for s in samples if "error" not in s and s.get(key) is not None]


def rate_per_day(samples, key, since, until):
    """Percentage points per day between the first and last good sample in
    [since, until]. None when the samples do not span at least six hours."""
    pts = [(parse_ts(s["ts"]), s[key]) for s in good(samples, key)
           if since <= parse_ts(s["ts"]) <= until]
    if len(pts) < 2:
        return None
    (t0, p0), (t1, p1) = pts[0], pts[-1]
    span = (t1 - t0).total_seconds() / 86400
    if span < 0.25:
        return None
    return max(p1 - p0, 0) / span


def fmt_days(d):
    if d is None:
        return "n/a"
    if d < 1:
        return f"{d * 24:.0f} hours"
    return f"{d:.1f} days"


def report():
    latest = sample(quiet=True)
    samples = load_log()
    t = now()
    lines = ["usage report  " + t.strftime("%Y-%m-%d %H:%M UTC")]
    if "error" in latest:
        lines.append("  the fresh sample failed: " + latest["error"])
        latest = (good(samples, "fable_pct") or [None])[-1]
        if latest is None:
            lines.append("  and the log has no good sample yet — nothing to estimate from")
            return "\n".join(lines)
        lines.append("  falling back to the last good sample from " + latest["ts"])

    for key, label in (("fable", "Fable (weekly, scoped)"),
                       ("weekly", "All models (weekly)")):
        pct = latest.get(key + "_pct")
        reset_s = latest.get(key + "_resets_at")
        if pct is None or not reset_s:
            lines.append(f"  {label}: not reported by the endpoint")
            continue
        reset = parse_ts(reset_s)
        start = reset - WEEK
        elapsed = (t - start).total_seconds() / 86400
        to_reset = (reset - t).total_seconds() / 86400
        avg = pct / elapsed if elapsed > 0.02 else None
        recent = rate_per_day(samples, key + "_pct", max(start, t - dt.timedelta(days=3)), t)
        rate = max(r for r in (avg, recent) if r is not None) if (avg or recent) else None
        left = 100 - pct
        lines.append(f"  {label}: {pct}% used, {elapsed:.1f} days into the window, "
                     f"resets in {fmt_days(to_reset)} ({reset.strftime('%a %d %b %H:%M UTC')})")
        if rate is None or rate <= 0:
            lines.append("    no burn yet this window, so no estimate")
            continue
        days_left = left / rate
        lines.append(f"    burn: {avg:.1f} pts/day averaged over the window"
                     + (f", {recent:.1f} pts/day over the last 3 days" if recent is not None else "")
                     + f"; using {rate:.1f}")
        if days_left >= to_reset:
            end_pct = pct + rate * to_reset
            lines.append(f"    ESTIMATE: lasts the week — about {days_left:.1f} days of budget at this rate, "
                         f"ending the window near {end_pct:.0f}%")
        else:
            when = t + dt.timedelta(days=days_left)
            lines.append(f"    ESTIMATE: runs out in {fmt_days(days_left)}, "
                         f"around {when.strftime('%a %d %b %H:%M UTC')}, "
                         f"{fmt_days(to_reset - days_left)} before the reset")

    spct = latest.get("session_pct")
    if spct is not None:
        lines.append(f"  Session (5-hour): {spct}%")
    n_good = len(good(samples, "fable_pct"))
    lines.append(f"  log: {len(samples)} samples ({n_good} good) at {LOG}")
    return "\n".join(lines)


def history():
    for s in load_log():
        print(one_line(s))


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--report", action="store_true")
    ap.add_argument("--history", action="store_true")
    a = ap.parse_args()
    if a.report:
        print(report())
    elif a.history:
        history()
    else:
        sample()


if __name__ == "__main__":
    sys.exit(main())
