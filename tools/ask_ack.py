#!/usr/bin/env python3
"""ask_ack.py — quick feedback on a field ask: stamp it `building` the moment
the monitor sees it, before triage writes a word (ash, 2026-09-02: "it would
be good to get quick feedback when something is being built").

Reads tools/ask_monitor.py's stream on stdin, passes every line through to
stdout unchanged (the Monitor's event stream), and for each new ASK (not the
BACKLOG at startup) calls tools/stamp_ask.py --local --text <the ask's first
words> --status building. Triage's own stamps follow as usual (shipped, or a
did-you-mean question).

Feature flow, ruled 2026-09-03 (invite-test #p160): anyone may ask, but the
person paying for the builds decides what gets built. An ask from an admin
or support user is stamped `building` at once; an ask from anyone else is
stamped `proposed` — ash accepts and orders proposals by hand, they are
built in a batch, and everyone gets them (a person can switch a feature off
in the chooser). The asker's authority is read from ~/.miso-auth/users.json
by the world key the monitor prints.

    python3 tools/ask_monitor.py --local | python3 -u tools/ask_ack.py
"""
import json, subprocess, sys, os
HERE = os.path.dirname(os.path.abspath(__file__))
AUTH = os.environ.get("MISO_AUTH_DIR") or os.path.expanduser("~/.miso-auth")


def authority_of(key):
    """'admin' / 'support' / '' for the guest-list entry whose phone is the key"""
    try:
        with open(os.path.join(AUTH, "users.json")) as fh:
            users = json.load(fh)
    except Exception:
        return ""
    digits = "".join(c for c in key.split(":", 1)[-1] if c.isdigit())
    for u in users:
        p = "".join(c for c in str(u.get("phone", "")) if c.isdigit())
        if p and p == digits:
            return str(u.get("authority") or "")
    return ""


pending = ""
for line in sys.stdin:
    sys.stdout.write(line); sys.stdout.flush()
    s = line.strip()
    if s.startswith("ASK asked"):
        pending = "?"
        for part in s.split():
            if part.startswith("user="):
                pending = part[len("user="):]
        continue
    if pending and s.startswith("text:"):
        who, pending = pending, ""
        words = s[len("text:"):].strip()
        key = words[:48]
        status = "building" if authority_of(who) in ("admin", "support") else "proposed"
        try:
            r = subprocess.run([sys.executable, os.path.join(HERE, "stamp_ask.py"), "--local",
                                "--text", key, "--status", status],
                               capture_output=True, text=True, timeout=30)
            out = (r.stdout.strip().splitlines() or [""])[-1]
            print(f"ACK {status}: {out}", flush=True)
        except Exception as e:
            print(f"ACK failed: {e}", flush=True)
