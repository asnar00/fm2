#!/usr/bin/env python3
"""reset_user.py — take a test user out of miso, and everything of theirs.

    python3 tools/reset_user.py <name>            # on the mini (or MISO_HOST)
    python3 tools/reset_user.py <name> --dry-run  # say what would go, touch nothing
    python3 tools/reset_user.py --list            # the guest list, with who invited whom

What a user is, and what goes (ash, 2026-09-03, "quickly reset after a test"):

  1. Their cards held by OTHER people — the copies /exchange handed over
     (owner == name) — become tombstones (/delete's shape, `deleted` and
     `edited` stamped now), and the role links naming them leave other
     people's projects. Written through the op door, `POST /diag/context`,
     the same road as any edit: /guard merges by id and keeps the newer
     `edited`, so a tombstone is the only write that removes anything (an
     absent id would be put straight back), and every prior value stays
     in those owners' op logs for /revert.
  2. Their guest-list row leaves users.json (written the way /invite
     writes it: temp file, rename); the row is appended to
     `~/.miso-auth/removed.json` so it can be put back by hand.
  3. Their auth lines go: the pending PIN, passkeys, push subscriptions,
     challenges, sends, and their own live invite codes.
  4. Their world — the op log `~/.miso-context/<key>.log` — is MOVED to
     `~/.miso-context/removed/<key>.<time>.log`, never deleted.
  5. The server is restarted the way deploy.sh restarts it (a reuseport
     handover, no dropped requests), because the world it holds in memory
     has no other way out and the next login must find nothing.

Nothing here is a delete: every step leaves the bytes where a hand can
restore them. Refuses to touch a user with `authority` set (an admin or
support person) unless --force is given.
"""
import argparse
import json
import os
import subprocess
import sys
import time
import urllib.parse

MINI = os.environ.get("MISO_HOST") or "microserver@microservers-Mac-mini.local"
PORT = os.environ.get("MISO_PORT", "8095")
AUTH = os.environ.get("MISO_AUTH_DIR") or os.path.expanduser("~/.miso-auth")
CTX = os.environ.get("MISO_CONTEXT_DIR") or os.path.expanduser("~/.miso-context")
CARDS_PATH = "miso/loop/cards"


def local():
    """this script runs on the machine that holds the state. From elsewhere it
    re-runs itself over ssh on MISO_HOST, in the checkout there."""
    return os.environ.get("MISO_RESET_LOCAL") == "1" or os.path.isdir(AUTH)


def sh(cmd):
    r = subprocess.run(["bash", "-c", cmd], capture_output=True, text=True)
    return r.returncode, r.stdout, r.stderr


def door_get(key):
    code, out, _ = sh(f"curl -s 'localhost:{PORT}/diag/context?user={key}'")
    try:
        v = json.loads(out)
    except json.JSONDecodeError:
        sys.exit(f"reset_user: the op door gave no snapshot for {key}: {out[:120]}")
    if isinstance(v, dict):
        sys.exit(f"reset_user: the op door refused {key}: {v}")
    return v


def door_set(key, path, name, value):
    body = json.dumps({"path": path, "name": name, "value": value}).replace("'", "'\\''")
    code, out, _ = sh(f"curl -s -X POST 'localhost:{PORT}/diag/context?user={key}' -d '{body}'")
    if '"ok":true' not in out:
        sys.exit(f"reset_user: POST refused for {key}: {out[:200]}")


def key_file(key):
    safe = "".join(c if (c.isalnum() or c in "._-") else "%%%02X" % ord(c) for c in key)
    return os.path.join(CTX, safe + ".log")


def key_of(u):
    return "phone:" + u.get("phone", "")


def world_keys():
    out = []
    for f in os.listdir(CTX):
        if f.endswith(".log") and f != "_global.log":
            out.append(urllib.parse.unquote(f[:-4]))
    return out


def users():
    with open(os.path.join(AUTH, "users.json")) as fh:
        return json.load(fh)


def save_users(lst):
    p = os.path.join(AUTH, "users.json")
    tmp = p + ".tmp"
    with open(tmp, "w") as fh:
        json.dump(lst, fh, indent=2, sort_keys=True)
    os.chmod(tmp, 0o600)
    os.replace(tmp, p)


def strip_cards(cards, name, now):
    """the copies of their cards become tombstones — /delete's shape: `deleted`
    and `edited` stamped now, the body one empty title, no links — and the
    role links naming them leave other people's projects, with `edited` moved
    so the change is the newer one. /guard merges a cards set by id and takes
    the newer `edited`, so this is the only shape of write that can remove
    anything: an absent id is quietly put back (guard.md), a tombstone is not."""
    out = []
    dropped = 0
    unlinked = 0
    for c in cards:
        if c.get("owner") == name:
            if not c.get("deleted"):
                c = dict(c, blocks=[{"kind": "title", "text": ""}], links=[],
                         deleted=now, edited=now)
                dropped += 1
            out.append(c)
            continue
        links = c.get("links")
        if isinstance(links, list):
            keep = [l for l in links
                    if not (l.get("kind") == "role"
                            and str(l.get("to", "")).startswith(name + "."))]
            if len(keep) != len(links):
                unlinked += len(links) - len(keep)
                c = dict(c, links=keep, edited=now)
        out.append(c)
    return out, dropped, unlinked


def strip_lines(path, phone, name, dry):
    """every line of an auth file mentioning the user's phone (as a whole
    field) goes; the file is rewritten in place. Returns the count."""
    if not os.path.exists(path):
        return 0
    with open(path) as fh:
        lines = fh.read().split("\n")
    keep, gone = [], 0
    for l in lines:
        parts = l.split()
        if phone in parts or name in parts:
            gone += 1
        else:
            keep.append(l)
    if gone and not dry:
        with open(path, "w") as fh:
            fh.write("\n".join(keep))
    return gone


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("name", nargs="?", help="the user's name on the guest list")
    ap.add_argument("--dry-run", action="store_true")
    ap.add_argument("--list", action="store_true")
    ap.add_argument("--force", action="store_true", help="also a user with authority")
    ap.add_argument("--no-restart", action="store_true")
    a = ap.parse_args()

    if not local():
        argv = " ".join(map(lambda s: "'" + s.replace("'", "'\\''") + "'", sys.argv[1:]))
        r = subprocess.run(["ssh", "-o", "BatchMode=yes", MINI,
                            f"cd ~/fm2 && MISO_RESET_LOCAL=1 python3 tools/reset_user.py {argv}"])
        sys.exit(r.returncode)

    lst = users()
    if a.list or not a.name:
        by_key = {key_of(u): u.get("name") for u in lst}
        for u in lst:
            inv = by_key.get(u.get("invited_by", ""), u.get("invited_by", ""))
            auth = f"  [{u['authority']}]" if u.get("authority") else ""
            print(f"{u.get('name'):16} {u.get('phone', ''):16} invited by {inv or '-'}{auth}")
        return

    name = a.name
    me = next((u for u in lst if u.get("name") == name), None)
    if not me:
        sys.exit(f"reset_user: no '{name}' on the guest list (--list shows it)")
    if me.get("authority") and not a.force:
        sys.exit(f"reset_user: '{name}' has authority {me['authority']!r} — refusing without --force")
    phone = me.get("phone", "")
    key = key_of(me)
    dry = a.dry_run
    say = (lambda s: print(("would " if dry else "") + s))

    # 1. what other people hold of theirs
    for other in world_keys():
        if other == key:
            continue
        snap = door_get(other)
        row = next((v for v in snap if v["name"] == "cards" and v["path"] == CARDS_PATH), None)
        if not row:
            continue
        cards = json.loads(row.get("value") or "[]")
        kept, dropped, unlinked = strip_cards(cards, name, int(time.time() * 1000))
        if dropped or unlinked:
            say(f"tombstone {dropped} of {name}'s cards held by {other}, drop {unlinked} role link(s)")
            if not dry:
                door_set(other, CARDS_PATH, "cards", json.dumps(kept))

    # 2. the guest list
    say(f"remove {name} ({phone}) from users.json")
    if not dry:
        with open(os.path.join(AUTH, "removed.json"), "a") as fh:
            fh.write(json.dumps(dict(me, removed=int(time.time() * 1000))) + "\n")
        save_users([u for u in lst if u is not me])

    # 3. auth lines
    for f in ("pending.txt", "passkeys.txt", "push-subs.txt", "challenges.txt", "sends.txt"):
        n = strip_lines(os.path.join(AUTH, f), phone, name, dry)
        if n:
            say(f"drop {n} line(s) from {f}")
    qr = os.path.join(AUTH, "invite-qr.json")
    if os.path.exists(qr):
        try:
            rows = json.load(open(qr))
        except json.JSONDecodeError:
            rows = None
        if isinstance(rows, list):
            keep = [r for r in rows if r.get("by") != key]
            if len(keep) != len(rows):
                say(f"drop {len(rows) - len(keep)} live invite code(s) of theirs")
                if not dry:
                    json.dump(keep, open(qr, "w"), indent=2)

    # 4. the world
    log = key_file(key)
    if os.path.exists(log):
        os.makedirs(os.path.join(CTX, "removed"), exist_ok=True)
        dest = os.path.join(CTX, "removed",
                            os.path.basename(log)[:-4] + "." + str(int(time.time())) + ".log")
        say(f"move their world {os.path.basename(log)} -> removed/{os.path.basename(dest)}")
        if not dry:
            os.replace(log, dest)
    else:
        say(f"no world log for {key} (never logged in)")

    # 5. the server forgets
    if a.no_restart or dry:
        say("restart com.noob.miso (the world in memory)")
        return
    uid = os.getuid()
    code, out, err = sh(f"launchctl kickstart -k gui/{uid}/com.noob.miso")
    if code != 0:
        sys.exit(f"reset_user: restart failed: {err.strip()} — the world is still in memory; "
                 f"restart the server by hand")
    for _ in range(40):
        time.sleep(0.5)
        code, out, _ = sh(f"curl -s -m 2 localhost:{PORT}/version")
        if code == 0 and out.strip():
            print(f"restarted, build {out.strip()}; {name} is gone")
            return
    sys.exit("reset_user: the server did not come back within 20 s — check /tmp/miso.log")


if __name__ == "__main__":
    main()
