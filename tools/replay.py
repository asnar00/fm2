#!/usr/bin/env python3
"""Stand a recorded muon session up in a test browser and replay it.

Pulls blackbox batches off the mini over ssh, assembles the chosen window into
site/replay.json (local only — deploy removes it), makes sure the local muon
server is running (localhost is ungated: no login in the test browser), and
opens the app with ?replay=1 — in a booted iPhone simulator with --simulator,
else the default browser.

Usage: replay.py [--who 3023] [--minutes 30] [--speed 1] [--simulator]
"""

import argparse
import json
import socket
import subprocess
import time
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
BUILD = REPO / "products" / "muon" / "build"
MINI = "microserver@microservers-Mac-mini.local"


def pull_batches():
    raw = subprocess.run(["ssh", MINI, "cat /tmp/muon-blackbox.log 2>/dev/null"],
                         capture_output=True, text=True).stdout
    for line in raw.splitlines():
        parts = line.split(" ", 2)
        if len(parts) != 3:
            continue
        try:
            yield parts[1], json.loads(parts[2])
        except json.JSONDecodeError:
            continue


def assemble(who: str, minutes: float):
    keyframes, entries = [], []
    tags = set()
    for tag, batch in pull_batches():
        tags.add(tag)
        if who and not tag.endswith(who):
            continue
        keyframes += batch.get("keyframes") or []
        entries += batch.get("entries") or []
    if not entries:
        raise SystemExit(f"no recorded events" +
                         (f" for who={who}" if who else "") +
                         (f" (tags seen: {', '.join(sorted(tags))})" if tags else
                          " — the blackbox log on the mini is empty"))
    keyframes.sort(key=lambda k: k["t"])
    entries.sort(key=lambda e: e["t"])
    cutoff = entries[-1]["t"] - minutes * 60000
    entries = [e for e in entries if e["t"] >= cutoff]
    return {"keyframes": keyframes, "entries": entries}


def ensure_server():
    try:
        socket.create_connection(("localhost", 8095), timeout=0.5).close()
        return
    except OSError:
        pass
    binary = BUILD / "server" / "target" / "release" / "muon_server"
    if not binary.exists():
        raise SystemExit("no local build — run: python3 tools/fmlink.py muon")
    subprocess.Popen([binary], cwd=BUILD,
                     stdout=open("/tmp/muon-replay-server.log", "w"),
                     stderr=subprocess.STDOUT)
    time.sleep(0.6)
    print("local muon server started (logs: /tmp/muon-replay-server.log)")


def open_target(url: str, simulator: bool):
    if simulator:
        booted = subprocess.run(["xcrun", "simctl", "list", "devices", "booted"],
                                capture_output=True, text=True).stdout
        if "Booted" not in booted:
            print("booting an iPhone simulator (first boot takes a moment)…")
            avail = subprocess.run(["xcrun", "simctl", "list", "devices", "available"],
                                   capture_output=True, text=True).stdout
            for line in avail.splitlines():
                if "iPhone" in line and "(" in line:
                    udid = line.split("(")[1].split(")")[0]
                    subprocess.run(["xcrun", "simctl", "boot", udid])
                    subprocess.run(["open", "-a", "Simulator"])
                    time.sleep(6)
                    break
        subprocess.run(["xcrun", "simctl", "openurl", "booted", url])
    else:
        subprocess.run(["open", url])


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--who", default="", help="match the last digits of the phone tag")
    ap.add_argument("--minutes", type=float, default=30)
    ap.add_argument("--speed", type=float, default=1)
    ap.add_argument("--simulator", action="store_true")
    args = ap.parse_args()

    data = assemble(args.who, args.minutes)
    (BUILD / "site" / "replay.json").write_text(json.dumps(data))
    span = (data["entries"][-1]["t"] - data["entries"][0]["t"]) / 1000
    print(f"session window: {len(data['entries'])} events over {span:.1f}s, "
          f"{len(data['keyframes'])} keyframes")
    ensure_server()
    url = f"http://localhost:8095/?replay=1&speed={args.speed}"
    open_target(url, args.simulator)
    print(f"replaying at {url}")


if __name__ == "__main__":
    main()
