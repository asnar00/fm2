#!/usr/bin/env python3
"""A sweep, from the black box: the gesture, the switch, and what the arriving
card's picture was at the moment it reached the screen.

    python3 tools/sweeps.py                  # ash's last few sweeps
    python3 tools/sweeps.py _ash             # somebody else's
    python3 tools/sweeps.py asnaroo 12       # and how many

The record is `/diag/blackbox`'s: every turn's event, `/touches`' pointer and
touch lines, and — since `/arriving-picture` — two lines per card paint saying
what the picture was at insertion and at the next frame. Written after three
rig readings of the "flashing" ask turned out wrong (field-walk #p95): the
phone's own log is the evidence, and this is the one query over it.
"""
import json, pathlib, sys

LOGS = ["/tmp/miso-blackbox.log", "/tmp/miso-blackbox.log.old"]
WIN = 2500          # ms either side of a switch


def entries(who):
    out = []
    for p in LOGS:
        f = pathlib.Path(p)
        if not f.exists():
            continue
        for line in f.read_text(errors="replace").splitlines():
            parts = line.split(" ", 2)
            if len(parts) < 3 or not parts[1].startswith(who):
                continue
            try:
                batch = json.loads(parts[2])
            except Exception:
                continue
            for e in (batch.get("entries") or []):
                if isinstance(e, dict) and "t" in e:
                    out.append(e)
    out.sort(key=lambda e: e["t"])
    seen, uniq = set(), []
    for e in out:                       # a batch ships more than once
        k = (e["t"], json.dumps(e.get("event"), sort_keys=True)[:200])
        if k in seen:
            continue
        seen.add(k)
        uniq.append(e)
    return uniq


def line(e):
    ev = e.get("event") or {}
    t = ev.get("type", "?")
    if t == "click":
        return "click " + str(ev.get("ev"))
    if t == "ui":
        return ("ui " + str(ev.get("kind")) + " @" + str(ev.get("x")) + "," + str(ev.get("y"))
                + " " + str(ev.get("target"))[:22])
    if t == "media":
        return ("MEDIA  card " + str(ev.get("card")) + "  src " + str(ev.get("src"))
                + "  complete " + str(ev.get("complete")) + "  w " + str(ev.get("w"))
                + "  away " + str(ev.get("away")) + "  video " + str(ev.get("video"))
                + "/" + str(ev.get("ready")) + "  by " + str(ev.get("cause"))
                + "  +" + str(ev.get("since")) + "ms"
                + ("  ghost" if ev.get("ghost") else ""))
    if t == "media2":
        return ("  ...one frame on: still " + str(ev.get("still"))
                + "  src changed " + str(ev.get("changed")) + " -> " + str(ev.get("src"))
                + "  complete " + str(ev.get("complete")) + "  w " + str(ev.get("w")))
    return t + (" " + str(ev.get("id", ""))[:18] if ev.get("id") else "")


def main():
    who = sys.argv[1] if len(sys.argv) > 1 else "asnaroo"
    many = int(sys.argv[2]) if len(sys.argv) > 2 else 4
    es = entries(who)
    print(f"{len(es)} entries for {who}")
    at = [i for i, e in enumerate(es)
          if (e.get("event") or {}).get("ev") in ("browse_next", "browse_prev")]
    print(f"{len(at)} sweeps\n")
    if not at:
        print("no sweeps in the window the ring keeps")
        return
    for i in at[-many:]:
        t0 = es[i]["t"]
        print(f"== sweep at {t0} ({line(es[i])})")
        for e in es:
            if abs(e["t"] - t0) <= WIN:
                print(f"   {e['t']-t0:+6d} ms  {line(e)}")
        print()


if __name__ == "__main__":
    main()
