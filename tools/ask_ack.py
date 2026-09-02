#!/usr/bin/env python3
"""ask_ack.py — quick feedback on a field ask: stamp it `building` the moment
the monitor sees it, before triage writes a word (ash, 2026-09-02: "it would
be good to get quick feedback when something is being built").

Reads tools/ask_monitor.py's stream on stdin, passes every line through to
stdout unchanged (the Monitor's event stream), and for each new ASK (not the
BACKLOG at startup) calls tools/stamp_ask.py --local --text <the ask's first
words> --status building. Triage's own stamps follow as usual (shipped, or a
did-you-mean question).

    python3 tools/ask_monitor.py --local | python3 -u tools/ask_ack.py
"""
import subprocess, sys, os
HERE = os.path.dirname(os.path.abspath(__file__))
pending = False
for line in sys.stdin:
    sys.stdout.write(line); sys.stdout.flush()
    s = line.strip()
    if s.startswith("ASK asked"):
        pending = True
        continue
    if pending and s.startswith("text:"):
        pending = False
        words = s[len("text:"):].strip()
        key = words[:48]
        try:
            r = subprocess.run([sys.executable, os.path.join(HERE, "stamp_ask.py"), "--local",
                                "--text", key, "--status", "building"],
                               capture_output=True, text=True, timeout=30)
            out = (r.stdout.strip().splitlines() or [""])[-1]
            print(f"ACK building: {out}", flush=True)
        except Exception as e:
            print(f"ACK failed: {e}", flush=True)
