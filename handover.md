# handover
*state of play for the next session — written 2026-08-25, evening, at the
end of the user-accounts day (transcripts/2026-08-25-accounts.md, ~80
prompts). Discipline in `agents.md`; ops in `deploy.md`; the pipeline in
`hybrid.md`; the ledger is `misses.md`. Read the composed skillset alongside
this — it now carries five agent-instruction nodes: /taste, /did-you-mean,
/attention, /glyphs, /anticipation.*

## THE HEADLINE: live at build 322, deploys no longer restart the server

Tara (the customer) visits 2026-08-26. The day built the account surface,
the card object under it, the invite loop, and made people visible to each
other; it also removed the deploy cliff and the data-loss class. Eleven
Opus workers, ~25 direct builds, builds 261→322, all pushed.

**What a user has now (in the order they meet it):**
- Install the PWA, log in with a texted code; 👤 shows **the people**: your
  card first, then whoever the invites put nearest (`/people`, grid ⇄ list
  pill top-left level with the lozenge; a map view can join it later).
- Your **card** (`/cards`, `/page/me`): name, picture (framed by pinch —
  `/frame`; from the photo's own GPS tag — `/from-picture`; full width —
  `/wide`), mission, a `profile` tag in its colour (`/tag`), **map
  location** with a pin (`/location`, `/map-pin`), on a dark ground that
  hugs it (`/ground`, `/hug`), edited in place with words kept as you type
  and Enter that keeps its line and its caret (`/keep`, `/newline`); undo
  works on it (`/undo/late`, `/guard/revert`).
- **Invite** (`/invite`, sub-tool of 👤 with a drawn plus): support/admin
  add a person by name + number; *invited* → *joined*; admin may invite
  pretend `_` people (`/pretend`). **Invite makes you visible to each
  other** (`/exchange` stage one: your card is copied into your inviter's
  and invitees' worlds on every write, seeded at join; foreign cards are
  read-only; `via` is an opaque tag, never a number).
- Nothing rings twice and nothing rings while you're looking
  (`/fresh-words`, `/present`).

**Foundations landed:** `/guard` (a cards write can never drop a card;
blank duplicates discarded; `/owner`: an id keeps its owner);
`/world-cache` (the device keeps its world, hydrates before first paint,
wipes on sign-out — no empty-world window exists any more); `/reuseport` +
`/handover` (SO_REUSEPORT, drain, deploy waits for the successor — the
plist is INSTALLED on the mini, backup at `com.noob.miso.plist.pre-handover`;
the first handover deploy was build 322, 0 requests in flight); the linker
tie-break fix (regroups are order-neutral by construction now; `serve_port()`
is a seam); `/roomier` + `/wider` (192KB messages, a 160KB cards list).

**Rehearsal state on the live mini:** `_alice` (+15551234567) and `_bob`
(+15551234741) are on the guest list, invited by ash, each with a card;
ash's phone has a second home-screen icon logged in as alice. Their codes
are read off `/tmp/miso.log` (`scratchpad/walk/person.py` drives them
headless against the live site). Ash's own entry is `admin`.

## FOR ASH

- **Edit your card once** if you haven't since build 319: that hands your
  card to alice and bob (they joined before the seed existed).
- Tomorrow with Tara: invite her from 👤 (real number → real SMS); she
  installs, logs in, sees you and her card; you see hers. She is `member`;
  make her `support` on the mini if she should invite her team.
- Things I judged and you may want to re-rule: no accept tap on a card
  arriving (only invite-linked people can reach you); distance word in the
  list is "you / n away"; non-profile cards are not on 👤 (projects get
  their own surface).

## THE NEXT WORK (chosen, not owed)

1. **Projects + links** (#p7, #p14, #p71): `miso` project card, `ash —lead
   dev→ miso`, Tara `—candidate→ sevenoaks 2029`; a projects surface reusing
   `/browse`'s two seams (`browse_cards`, `browse_row_left`); **shared
   project membership as the second visibility cue** (joins `people_order`
   and `users/near`); "current project" as a per-user var feeding the
   contexts machinery.
2. **Exchange stage two**, only when asked: send to a number, an inbox,
   withdrawing a card (`exchange_copy`/`exchange_give` are the seams).
3. **Var-per-card + blob path** — every edit still resends the whole list
   (picture included) to every invite-linked person; `/wider` bought room,
   not a fix.
4. Named foundations: a rename map for vars when a declaring node moves
   (path-keyed op log + cache); `/remember`'s append is read-modify-write
   (single writer only — the handover sequences around it); `/tmp/miso-
   broadcast.json` is a fixed path shared by every server on a machine;
   `/tap`'s undo/redo counter semantics; a relative "3h ago" needs a clock
   in `render`.
5. Older: tunables, webgpu restart, redo.md items 3–5, the rewind experiment.

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

