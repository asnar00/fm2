#!/usr/bin/env python3
"""ask_ack.py — announce a field ask the moment the monitor sees it, and say
who asked, so triage can stamp it without a lookup.

It used to stamp too: `building` for an admin or support asker, `proposed`
for anyone else, before triage had written a word (ash, 2026-09-02: "it would
be good to get quick feedback when something is being built"). Ash withdrew
that on 2026-09-04 (field-walk #p199): a request that files itself AND stamps
itself building reads as the machine deciding, and it was doing so beside a
popup that argued the feature already existed. **Every ask now sits at
`asked` until a person stamps it.**

    python3 tools/ask_monitor.py --local | python3 -u tools/ask_ack.py

Reads the monitor's stream on stdin, passes every line through to stdout
unchanged (the Monitor's event stream), and for each new ASK (not the BACKLOG
at startup) prints one line naming the asker's authority — which is what
triage needs to choose the stamp:

    ASK from admin — stamp: building / answered / a did-you-mean

Feature flow, ruled 2026-09-03 (invite-test #p160) and unchanged: anyone may
ask, but the person paying for the builds decides what gets built. An ask from
an admin or support user is normally stamped `building`; an ask from anyone
else is stamped `proposed` — ash accepts and orders proposals by hand, they
are built in a batch, and everyone gets them. What changed in #p199 is only
who writes the stamp: a person, always. And there is a fourth answer now — if
the thing already exists, triage replies instead of building:

    tools/stamp_ask.py --text "<the ask>" --status answered --note "<how>"

The asker's authority is read from ~/.miso-auth/users.json by the world key
the monitor prints.
"""
import json, sys, os
AUTH = os.environ.get("MISO_AUTH_DIR") or os.path.expanduser("~/.miso-auth")

# what a person might stamp, by who asked — printed as a reminder, never written
ADVICE = {
    "admin": "building / answered / a did-you-mean",
    "support": "building / answered / a did-you-mean",
    "": "proposed / answered / a did-you-mean",
}


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
        auth = authority_of(who)
        # no stamp: the ask stays `asked` until a person looks at it (#p199)
        print(f"ASK from {auth or 'guest'} — stays `asked`; "
              f"stamp: {ADVICE.get(auth, ADVICE[''])}  [{words[:48]}]", flush=True)
