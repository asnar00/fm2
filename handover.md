# handover
*state of play for the next session — written 2026-08-25, evening, at the
end of the user-accounts day (transcripts/2026-08-25-accounts.md, ~80
prompts). Discipline in `agents.md`; ops in `deploy.md`; the pipeline in
`hybrid.md`; the ledger is `misses.md`. Read the composed skillset alongside
this — it now carries five agent-instruction nodes: /taste, /did-you-mean,
/attention, /glyphs, /anticipation.*

## THE HEADLINE: the hitlist is live — projects, posts, map — deploys never restart

Tara (the customer) visits 2026-08-26. Build 332 is live. The day built
the account surface, the card object, invite, people seeing each other,
projects with roles, posts, a real map — and removed the deploy cliff and
the data-loss class. Fourteen Opus workers, ~30 direct builds, builds
261→332, all pushed.

**What a user has now (in the order they meet it):**
- Install the PWA, log in with a texted code; 👤 shows **the people**: your
  card first, then whoever the invites put nearest (`/people`), as a grid,
  a picture-led list (`/portrait`), or a **real map** (`/map`: CARTO's dark
  render of OpenStreetMap through the mini's own tile proxy `/tiles`,
  Leaflet vendored, pins with faces, tap to open) — the picker top-left.
- Your **card** (`/cards`, `/page/me`): name, picture (framed — `/frame`;
  place from the photo's GPS tag — `/from-picture`), mission, a coloured
  type tag, **map location** with a pin, on a dark ground; edited in place
  with words kept as you type, Enter keeping its line and caret; undo
  works on it. Roles you hold show under the mission ("lead dev for miso").
- **Invite** (sub-tool of 👤): support/admin add by name + number;
  *invited* → *joined*; admin may invite pretend `_` people. **Invite
  makes you visible to each other** (`/exchange`: your cards are copied
  into your inviter's and invitees' worlds on every write; foreign cards
  read-only).
- **Projects** (flag tool, `/kinds/projects`): "something we're trying to
  get done"; **new**; a **people** section on a project you own — add a
  person you hold with a role; roles are links on the project card; a
  project travels only to its members. **Posts** (bubble tool,
  `/kinds/posts`): **+** → say something, from where you are; newest
  first; flows to invite-linked people. Both share `/kinds/new`'s
  `CardNew` door.
- Nothing rings twice, nothing rings while you're looking.

**Foundations landed:** `/guard` (+ `/owner`, `/revert`, `/singleton`: a
cards write can never drop a card; an id keeps its owner; only a profile
is one-per-owner); `/world-cache` (the device keeps its world, no empty
window, wiped on sign-out); `/reuseport` + `/handover` (**installed on the
mini** — every deploy since build 322 has been a handover with 0 requests
in flight; backup plist `com.noob.miso.plist.pre-handover`); the linker:
tie-break by contributing depth (regroups are order-neutral, nested
groupings included), `serve_port()` seam, stale-asset sweep on untick;
`/roomier` + `/wider` (192KB messages, 160KB list).

**Rehearsal state on the live mini:** `_alice` (+15551234567) and `_bob`
(+15551234741), invited by ash, each with a card; ash's phone has a second
icon logged in as alice; `scratchpad/walk/person.py` drives them headless
against the live site (codes off `/tmp/miso.log`). Ash is `admin`.

## FOR ASH (tomorrow morning, before Tara)

- Update the phone. 👤 → the map glyph: pins for you, alice, bob. The flag
  tool: **new** → "miso", add yourself as lead dev, add alice as
  canvasser → alice's 👤 card reads "canvasser for miso". The bubble tool:
  **+** → a post from where you stand.
- With Tara: invite her (real number → SMS); she installs, logs in, sees
  you; make a project "sevenoaks 2029", add her as candidate. She is
  `member`; make her `support` on the mini if she should invite her team.
- Two rulings you may want to make: CARTO dark tiles vs plain OSM (one env
  var, `MISO_TILE_URL`); a project reaching only its members (not your
  whole invite tree).

## THE NEXT WORK (chosen, not owed)

0. **The smoke gate is in deploy.sh** (`tools/smoke.py`, accounts #p96):
   nine steps × three passes must be green or nothing ships. Next rungs:
   (a) tree-owned steps — each node carries `<name>.smoke.py`, fmlink
   composes them; (b) a boot self-check on the device reporting through
   `/diag` (the tap seam is `open()`, the veil lifted, no orphaned
   wrapper) — the only layer that sees the real phone; (c) check the
   update-policy default a NEW user gets (`update_policy` is the empty
   string — find what that means) so a dead control can never trap
   someone on an old build: the lozenge was the only road to an update
   when it died (#p95).
1. **Known bug, fix first:** `/keep` — typing right after a fresh repaint
   can land a character one place early ("buildin — v2g"); seen in two
   rigs; the keystroke races the caret restore. Reproduce with
   `scratchpad/invite-rig/caret.py`'s pattern and a keystroke inside the
   600ms debounce window.
2. **Project membership as the second visibility cue** (#p71 "later"):
   members of a project see each other (`people_order`, `users/near`,
   `exchange_give` are the seams). Posts in a project (`links:[{kind:"in"}]`
   reserved). Current-project filtering.
3. **Exchange stage two** only when asked: send to a number, withdrawal,
   an inbox.
4. **Var-per-card + blob path**: every edit resends the whole list to every
   invite-linked person and project member (four world reads per write).
5. Named foundations: a var rename map when a declaring node moves;
   `/remember`'s append is read-modify-write (single writer); the fixed
   `/tmp/miso-broadcast.json`; vector tiles we style ourselves; a `loop`
   agent-instruction ("no clock inside update — time rides on the event");
   a singleton/`guard` note for new types.
6. Older: tunables, webgpu restart, redo.md items 3–5, the rewind
   experiment.

## standing doctrine landed today

- **Residuals are fixed in the run, never listed for signature** (#p50);
  a documented way to lose user data is a defect, not a residual (misses.md
  "the lost card"). Recovery move: the op log holds every prior value;
  replay one through `POST /diag/context?user=<raw key>`.
- **Anticipation** (#p74, `/anticipation`): ship the literal ask, shaped
  for the next three asks — seams, not builds. Its two failure modes are
  in the ledger (the exchange brief that built the foundation; the cards
  blob built with none).
- **Toolbar glyphs are ink** (`/glyphs`): filtered emoji or drawn SVG in
  currentColor; never an emoji-presentation character; undo stays last in
  every row — a newer node inserts before it.
- hybrid.md: the check-in (2× estimate, from outside, diagnosis as
  hypothesis); tear down rigs by PID; **`set -o pipefail` and assert the
  fragment composed before reading any evidence** (deploy.md) — four
  broken commits today came from a `| tail` hiding a link error.
- Ask tooling honours `MISO_HOST` (`.local` does not resolve from ash's
  laptop; use `microserver@185.96.221.52`); `?user=` takes the RAW key.

## tooling state

- Ask monitor via the Monitor tool; ~15 field asks today, every one
  stamped shipped with its build; one live did-you-mean answered on the
  phone.
- Rigs: `serve_port()` is a seam — retarget its body in the emitted
  main.rs (the `8095u16` literal is gone); one rig dir per worker; the
  invite/exchange/people rigs in the scratchpad are reusable patterns.
- Worktrees: ~12 agent worktrees under `.claude/worktrees/` — prune when
  convenient (`git worktree prune` after removing the dirs).
- Local `~/.miso-auth/users.json` carries test users from rigs; harmless.

