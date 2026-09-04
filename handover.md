# handover
*state of play for the next session — rewritten 2026-09-04 at session end
(transcripts/2026-09-04-field-walk.md); before that 2026-09-03
(transcripts/2026-09-03-housekeeping.md). Discipline in `agents.md`; ops in
`deploy.md`; the pipeline in `hybrid.md`; the ledger is `misses.md`. Read the
composed skillset alongside this — `/learned` carries 21 defaults now.*

## TODAY, EVENING (2026-09-04): builds 650–694 — the walkabout's asks, live as they came

*Written late, before the last node landed. Ash walked and filed from the
phone; each ask went to the worker that owned its ground (SendMessage to
a finished worker keeps its context — cheaper than a fresh brief) and
shipped on its own. Fable 60% → ~62%.*

- **Field asks shipped:** the time filter's marks ride every event
  (`since/marks-with-the-tap`; the boot send was a latch that never
  retried), `since/one-word` + `hugs-its-words` + `in-the-middle`; the
  thumbnail at once (`poster/at-once`, `at-once/first-frame` — never no
  picture); camera switch mid-take (`flip/while-recording`, a canvas-backed
  recorder — battery unmeasured on a phone); `audience/plain-words`,
  `plain-words/on-every-post`, `armed/explained/own-role` (no "same as me";
  "role" everywhere, never "rank"), `audience/visibility` (the eye replaces
  promote; `PostSetFloor`); `reel/current/from-the-pin` (a pin tap selects,
  the lozenge opens); `opens-over-map/on-every-tool`; `map-only/strip-on-
  black`, `map-only/always-the-ground` (a level inherits its parent's
  ground — learned 28; the launcher keeps its dots); `region/baked/lines-
  too` (the map is ONE tile layer; the vector overlay is gone);
  `being-built/stamp-stands` (fields have owners; the ack no longer loses to
  the phone's resend); `request-box/straight-through` (the ask box files
  at once, pops the guide at cosine ≥ 0.50; urgency is now triage's stamp);
  `transcribed/keeps-trying` (a keeper every 10 s, backoff to hourly, parked
  after a day, engineer lines), `shows-progress` + `on-the-reel`, `titled/
  from-the-words` (Haiku 4.5 titles an untitled post, ~$0.0003 a call);
  `carries-the-card/unbroken` (the switch fires under the finger; the old
  card ghosts; two cards on screen), `rubber-band/no-flash` (the compositing
  hint held 140 ms past the spring); `audience/withdrawn` (a raised floor
  hands the holder a tombstone; `tools/withdraw_copies.py` ran live —
  Tara's and bob's stale copies gone).
- **Found live and fixed:** ffmpeg not on the launchd PATH (every clip "no
  words" — both scripts resolve it by path now); the transcription drain ran
  only on a phone's message (now the keeper); a stamp written over by the
  phone's resend; a stale `open_tool` mirror under map-only; the map's
  placeholder view (see below).
- **The flash, settled by the phone's own log:** `blackbox/arriving-picture`
  + `tools/sweeps.py` — mid-list the picture is complete at insertion and
  never changes; the ends had no rebuild at all, it was the compositor.
  Three rig hypotheses were disproved before the readout was built.
- **Rolled back, then re-landed as build 696:** `map/keeps-its-view` (build 690) stuck ash's phone at
  "syncing…" and crashed the page; reverted as 691 (came up fine); the
  cause was a send from inside the paint (misses.md "the send inside the
  paint", learned 29); the fix hushes the whole of `/map`'s `sync`, defers
  the record to after the paint, restores only when a Leaflet is made, and
  treats a stored zoom at the placeholder as no memory — proven on the
  installed clip through four updates in a row. Phones that ran 690 hold a
  poisoned `map_view` at zoom 0, healed by that floor.
- **The flip mid-take is native again (`while-recording/segments`, the
  last build of the night):** a flip stops the recorder and starts another
  on the other camera (same microphone); `marks` on the clip's metadata say
  where each container starts; the mini joins N segments through MPEG-TS
  (a plain mp4 concat produced a 991-second file once — the header lies);
  the phone plays its first segment until it fetches the joined file. The
  canvas road stands down while this is ticked. **Watch on a real phone:**
  the cut at a flip is the camera's open time (100–400 ms on hardware; 7–9
  ms on the mock), and transcription on a joined file is proven by
  construction only.
- **The older clips, re-run through the new path (late):** the Sep 3 clip
  re-transcribed (Speechmatics, seeded); the Sep 1 and Sep 2 clips came
  back "no speech" from Speechmatics AND local whisper large (a hallucinated
  "You"), though their video audio is loud (−18 dB) — on those builds the
  phone's whisper-tiny read a separate companion audio recording that never
  left the phone, and the video's own audio track carries no speech. Their
  old words stay. Found on the way: a title answer of "none" was retried
  forever — fixed in `from-the-words` (a none ends the job; a rejected answer
  is logged as said). Seeding is by the post's location, so a London post
  gets Sevenoaks streets 30 km away — harmless, but the vocabulary should be
  empty when the nearest street is far.
- **The deploy stamps (build 699, `announced/by-the-ship`):** announce
  with `--node <tree path>`; deploy.sh, after `released.sha`, stamps
  shipped on every announcement whose node the release touched and every
  ask whose `asks#<t>` a commit subject cites, and prints any announcement
  still building after a day with no node. No hand stamps from now on —
  the skillset carries the instruction; the two-clips flip announcement
  has no node and will show in the reminder until its ship cites it.
- **Proposals in notes.md:** de-crufting the tree as a periodic fold pass
  (#p80, for after the field test).
- **Residuals named by workers, not yet ruled on:** once a level is picked
  it no longer follows a role change; a demoted/removed member keeps copies
  (the repair covers it); `/map` is at the six-child cap; five wrappers on
  `feature_Map` where order is load-bearing; the op door accepts a key that
  is not a world (writes nowhere, silently); old baked-tile stamp dirs never
  swept; the clip's first frame is black on the canvas recorder; a
  backgrounded app freezes video, not audio; `/steady`/`/glide` pay one
  extra animation on a way-back tap; the transcript export dirties main and
  blocked three deploys today (export only right before a commit).

## TODAY (2026-09-04): the field-walk day — five Opus workers in parallel, builds 615–649

*Written at session end. Fable 57% → 59% for the whole day: triage on Fable,
every build by an Opus worker seat in its own worktree and rig, reviewed
and shipped by triage. Twenty-two nodes landed, all proven on a rig, most
on the iPhone simulator. Ash walks tomorrow (Saturday) with the Sevenoaks
canvassing team; a test walkabout tonight or in the morning first.*

- **The morning:** the still and audio posts tombstoned in every world
  (`tools/prune_posts.py`, dry by default); `photo` unticked in the miso
  product (the override now materialises `loop/dictate`, `as-posts`,
  `capture`). Audience rulings recorded in notes.md: a post is stamped at
  its author's grade, promote lowers it, newcomers see what reached them —
  the app already did this.
- **Transcription** (`dictate/transcribed` + `vocabulary` + `api` +
  `mini`, `capture/video/streams`, `phone` unticked — 133 MB off the app):
  design in notes.md ("video notes to the mini" + amendment). The api rung
  is **Speechmatics with fieldnote's exact config** (key in
  `~/.agent-config.json` `speechmatics.api_key`, mode 600, never in the
  repo); the mini rung is mlx whisper turbo from fieldnote's venv, **its
  launchd plist `tools/com.noob.transcriber.plist` is written and NOT
  loaded** — the mini had 0.1 GB free with five rigs up; load it when the
  box is quiet and measure. `tools/streets.py --go` was run: 1,986 named
  places in `~/.miso-context/streets.json`. The worker found and fixed a
  real defect: a server-landed transcript did not travel to other people
  (`/exchange` shares on the route link only). **Unproven:** whether iOS
  hands over two-second chunks during recording or one blob at stop (the
  rig has no camera); the joined MP4 was never played back. Ash's walk is
  the test — watch `/transcribed`'s queue dir under `~/.miso-blobs` and the
  post's words arriving.
- **The map view:** `until-play` (+`incoming-too`), `from-the-lozenge`,
  `carries-the-card` (+`rubber-band`), `back-to-the-lozenge` (+`size-too`;
  the swipe-away close now shrinks in place — ash: "that's fine");
  `browse/map-only` (grid and list unreachable, every tool on the map)
  and `map-only/since` (today / week / month / all, default `all` —
  triage's call, one line in `since.vars`); the stale `open_tool` mirror
  fixed in `/under-account`, `projects`, `me`, `invite-tool`, `restore`
  (the map mints fewer healing turns than the grid did — the durable fix
  is `/payload` republishing at turn end, a core ask nobody has made).
- **The recording row** (`video-only/armed` + `explained` + `in-place`):
  + arms a row — rec, stop, camera (front by default, its own `camera`
  var), publish level (a column with a sentence each, popping over the
  row). `/audience` gained `audience_new_floor(grade)`; the chosen level
  is clamped to the author's rank. Residual: the record level shows bare
  ground, not the posts list.
- **Boundaries** (`boundaries/outlined`, `region`, `region/baked`): ward
  names gone, black outlines; a region sub-tool in the posts row (teal)
  picks the constituency or a ward, drawn in Stadia Outdoors from
  server-baked tiles (`tiles/region/<CODE>/z/x/y.png`, cached under
  `~/.miso-context/tiles-baked/<stamp>/`, ~3 ms a bake, RSS unchanged) so
  it zooms in step with the ground. Old stamp dirs are never swept. The
  Stadia budget should go down (only straddling and inside squares are
  fetched) — a hypothesis, check the account.
- **Rulings today:** dictate is deprecated, every note is a video;
  parity with fieldnote (Speechmatics) for transcription; the map is the
  only view; the + arms rather than fires; shrink-in-place on the swipe
  close is fine; anyone asks, the payer decides (from yesterday).
- **Misses today (misses.md):** the fast-forward that never happened
  (build 621 shipped without transcription; three stamps corrected); the
  merge commit that touched four nodes (main is linear: land by
  cherry-pick or ff); five workers, one scratchpad, one simulator (rig dirs
  named per worktree; `/tmp/miso-readout.json` and `/tmp/miso-drive.json`
  want an env var — every worker hit this). Also: a transcript export
  dirties main and blocks the next ff — export only right before a commit.
- **Residuals, named by the workers, not yet ruled on:** the projects
  new / me / invite reads are fixed but `/steady` and `/glide` still pay
  one extra animation on a way-back tap; four copies of the grade ladder
  (audience.rs, audience.js, armed.rs, explained.rs); the `level` tool
  level is dead code under `in-place`; `/region`'s row button gates on
  `browse_view_read() == "map"`, now always true; `.proj-title` is
  positioned from `since.css` with a hardcoded 200px; DST inside a month
  can move a card an hour across a boundary; the people-side time filter
  is proven only through the shared chain; `fmlink --prove` still calls a
  child-plus-parent-refactor "implied" (hit for real twice today — the
  workers carried trailers anyway).
- **Not loaded / not run:** the transcriber launchd job (above);
  `tools/reset_user.py` for the test rows — **ash resets before the real
  session**; main is ~70 commits ahead of origin, not pushed.

## FOR SATURDAY

Reset any test rows (`tools/reset_user.py --list`). Ethernet cable in the
mini (ash bought one today — plug it in and confirm `en0` carries the
default route before the watchdog does). Show one code from 👤 with the
project current; every canvasser scans it, types a name, taps join, lands
on the welcome, fills the card, enables Face ID and notifications, taps
done. They see each other at once. Posts they should see must have a
floor at or below their grade — Tara's posts start at candidate; she
promotes, or sets a publish level in the recording row before filming.

## TODAY, EVENING (2026-09-03): builds 565–589 — the invite test with two iPhones

*Written at session end (transcripts/2026-09-03-invite-test.md). 24
commits, Fable 50% → 54% for three hours of triage-built work; the
worker seat was not used. Ash drove the second iPhone; every build was
proven on the rig (curl, headless Chrome as a fresh user, the simulator
for the join page) before it shipped, and then again by Tara's phone.*

- **The reset tool** (`tools/reset_user.py <name>`, deploy.md): copies
  tombstoned through the op door (a `set` cannot delete — `/guard`; a
  tombstone is the only write that removes), guest-list row to
  `~/.miso-auth/removed.json`, world log to `~/.miso-context/removed/`,
  then a handover restart. `--list`, `--dry-run`. Removes the first row
  by name — two Taras took two runs.
- **The scan is the login** (`qr/scan-is-proof`, 570): a fresh number on a
  live code gets the cookie at the claim; a number already on the list
  keeps the PIN (a code proves the lead let you in, not that you own a
  colleague's number — ash may relax it). `qr/one-hour` (572). `qr/name-only`
  (573): no number needed; a 17-digit placeholder number, `/add-number`'s
  row on the card for a real one later. `scan-is-proof/seeded` (577): the
  scan seeds the inviter's cards as the PIN road did. **Ruling (#p15): the
  code is for the canvassing team at a session's start, never the public.**
- **Project membership is the second visibility cue** (`exchange/co-members`,
  577): `exchange_links` widens to every role on every project card the
  person holds; at `invited_into_stamp` the newcomer is seeded with every
  member's profile and hands theirs to all — nobody waits on anyone (#p59).
  Parked: the people page ranks a co-member as unknown; leaving a project
  keeps the copies.
- **The world that stayed behind** (`patch/world-along`, 574; misses.md):
  `/patch`'s hot swap started a fresh wasm world — switches at default,
  project dropped, epoch 0 — since `/context` moved the world into the
  module. The swap now carries the last payload's records across and
  rejoins. Diagnosed from the diag log: a launch line for 572, none for 573.
- **The first run** (`profile-first/greetings` 583, `greetings/set-up` 584,
  `greetings/last-word` 589): welcome-to-the-project, the card, Face ID +
  notifications with **got it** held until both settle, "that's it! hold
  any button", no tour. `greeted` (user var, 0–3). `tick-right`,
  `mission-flash` (587). The smoke gate walks all three pages
  (`tools/smoke.py pass_gate`).
- **Surfaces:** `current-project/title` (578; the chip retires), `frame/hint`
  (583), `install/smaller-logo`, `install/welcome` (580), `install/steps/
  menu-below` (585), `/counter` unticked in the miso product (589 — the
  taps tool is gone for everyone; ash's old switch is an orphan op the
  replay skips).
- **Rig lessons:** `diag/rig/plain-cookie` (a rig strips `Secure` from every
  route now, not only the ones older than `/rig`); the product's own
  `order.md` and symlink are needed under a product-local override dir
  (qr/ has one — `instant` is unticked there); commit with `git -C
  /Users/microserver/fm2` — twice a commit landed in the triage worktree
  after a `cd`; `tweaks.py --since` takes a date, not a time; a deploy
  refuses a dirty tree, so commit everything before chaining deploys, and
  stamp shipped only on the deploy's exit code (two stamps went out wrong
  and were reverted).
- **Residuals:** (a) `mission-flash`'s focus after the toolbar's blur is
  unverified (the probe crashed on its print; the flash itself is proven);
  (b) the rig's `users.json` holds a dozen test rows; (c) the triage
  worktree has two stray commits on a detached head; (d) the audience
  floor: ash's three newest posts were filed at `admin` while a candidate
  cannot see them — by design, ash agreed, but the default floor may want
  a look before Saturday; (e) `co-members` seeds profiles only; posts in a
  project stay `/audience`'s.

- **Builds 590–596, the field asks from Tara's phone** (all stamped
  shipped on her sheet): `tag/with-close` (a ✕ on every card, sending ‹'s
  event), `titled/byline` + `byline/plain` (author · date under a post's
  title; clip length, owner note and map pill hidden on post pages — scoped
  by the page's own `post` class), `browse/flick` (vertical sweep at the
  page's end walks the surface's list), `square-posts/sound` (a speaker on
  an audio-only post's pin), `live-only/everyone` (the people reel is the
  map's pins, live or placed). `kinds/posts` and `cards/page` are both at
  the six-child cap — the next child there forces a regroup.

- **Builds 597–601, the cards budget (evening, Opus workers):** the Soho
  video's poster had been silently refused by `/poster` because ash's
  cards list (176 KB, 94% inline pictures) was over `LIST_CAP`; one more
  pictured card would have jammed his outbox behind a 400 from
  `msg_body_cap`. Retrofit run through the op door (a 4.5 KB frame from the
  server's clip); `wider/room-for-a-team` (LIST_CAP 640 K, wire 1 MB);
  `messaging/past-a-refusal` (a 4xx that cannot succeed on retry is dropped
  with a diag report and a local `misoDropped` ring; `refused(status, msg)`
  is the seam opened in `/messaging`). A third worker is on the foundation:
  pictures beside the card, not inside the list (handover item 4).
  **Named risks:** the broadcast slot is capped by count, re-parsed 5×/s by
  every waiting phone — linear in list size × clients; `fmlink --prove`
  calls a parent-refactor-plus-child "implied" (a node's own files count as
  inside it) — the classifier needs the /confined shape; `flush()` shifts
  the head by position after an await (latent).

- **Builds 602–609, pictures beside the card (`cards/store/pic-beside`,
  Opus worker):** a picture block holds `pic/<24 hex>`; the bytes live in
  `~/.miso-blobs/pics/<id>`, write-once, served only to a logged-in caller
  whose own cards name the id; old inline pictures keep drawing; the
  device keeps its own copy in IndexedDB and uploads on its own queue;
  `tools/pics.py` is the retrofit door (dry by default; `--back` is the
  proven inverse). `/guard` moved under the new `/store` (a regroup; the
  old `guard*/enabled` ticks are orphan ops). **The live retrofit was run
  (`--go`) at session end** — ash's list 180 KB → 10 KB, 24 pictures over
  4 worlds. Risks the worker named: copies keep inline bytes until the
  owner's next real edit; the video road proven at the seam only; the
  local picture store is never pruned; the dressing is a regex over
  `src="pic/…"`. **Outage 22:46:** the worker's rig reset killed the live
  server by a fallback pgrep — misses.md "the rig that killed the server";
  the rule is now in the worker seat and deploy.md.

- **Builds 610–614, from ash's phone late evening:** `browse/flick/on-touch`
  (613 — iOS cancels pointer events on a scroll; the flick reads
  touchstart/touchend through `arm/release/go` seams opened in `/flick`,
  `go` dedupes the two roads at 400 ms), `capture/one-add/video-only`
  (614 — the add records; no kind chooser; audio/photo/write posts are not
  made any more, existing ones keep their kind; the smoke gate's post step
  mints through `/new`'s event under this rule). Field-ask flow ruled: a
  non-admin ask is stamped `proposed` (tools/ask_ack.py); ash accepts by
  word; batch-built; everyone gets it (notes.md "feature flow").

## TOMORROW (2026-09-04): a field walk — posts and transcription

Ash walks around making many video posts and transcribing on the phone and
the server. Watch: `/dictate` (the phone's whisper-tiny and the server road),
`/as-posts`, `/poster` now landing again under the new caps, `/pic-beside`
for the first real pictures beside cards from a phone, the outbox under
many writes (`misoDropped` in the engineer sheet if anything is refused),
and the broadcast slot's cost with bigger lists. Field asks from ash build
at once; anyone else's stamp `proposed` and wait for his word. Re-run
`python3 tools/tweaks.py --since 2026-09-03` at the next session end — the
late evening's eight asks are not distilled yet.

## FOR SATURDAY

Reset any test rows (`tools/reset_user.py --list`). Show one code from 👤
with the project current; every canvasser scans it, types a name, taps
join, lands on the welcome, fills the card, enables Face ID and
notifications, taps done. They see each other at once. Posts they should
see must have a floor at or below their grade.

## TODAY (2026-09-03): builds 531–564 — the taps' real cause, the map reel, the learning loop

*Written at session end. 34 commits, every build proven with a real finger
on the iPhone simulator before it shipped (memory: fm2-prove-the-real-path).
Fable 42% → 49% for the whole day on Opus workers — the switch worked.*

- **The taps (housekeeping #p3, `keep/lands/on-release`, 532):** ash's
  "two or three taps" was never a DOM race. The phone's black box held 89
  presses: every press that clicked was down ≤114 ms, every press with no
  click ≥127 ms — iOS hands a touch held past ~120 ms to another
  recognizer and never synthesises the click. The tap is read on
  `pointerup` now. misses.md has the entry; `scratchpad/taps.py` is the
  one-query reader. **Diagnose a phone tap bug from the black box first.**
- **Video posts:** `poster/player-in-place` (531, the clip above the
  words), `square-crop/clips-too` (536, the player and viewfinder as the
  central square), `poster/face-first` (537, the face until it plays),
  `as-posts/where-taken` (539, a recording placed where it was made, with
  its time — last night's clip had been placed at first opening; moved by
  hand through the op door).
- **The map:** `pins/fan-out` (538; the map regrouped: `square-posts` and
  the new node under `pins`), `with-live` (the live pin joins),
  `black-stem`, `bigger-faces` (+50%); `map/reel` (546; a second regroup:
  squares/boundaries/quiet-credits under `basemap`) with `floating`,
  `current` → `on-the-pin` → `stem-too`, `opens-over-map` (map behind the
  card, tap the map to close), `swipe-away`, `quicker`, `on-people-map` →
  `people-there` → `live-only`; and the reel lists exactly the map's set
  (`data-ids` on `#mapData`) with room at its end (#p22).
- **Reports:** `own-notes` (the writer told the truth: the team's own
  notes, never recordings of the public — ash's ruling, memory updated),
  `viewer` (a sheet with ‹ and share; the first two cuts probed with
  HEAD and fell back — proven on a planted report only after ash's
  second look), `fit-page`, `share-glyph`.
- **Also:** `tag/aligned/centred`, `long-press/further`; the wifi
  watchdog (`tools/wifi_watchdog.sh`, launchd `com.noob.wifiwatchdog`,
  log `~/wifi-watchdog.log`, seven quiet OKs — the mini is on Wi-Fi with an
  empty ethernet port; a cable is still the right answer for Saturday).
- **The learning loop (#p31–#p32):** `tools/tweaks.py` (the digest of
  every ask with its refinements, all history — 169 of 76 at first run)
  and `taste/learned/learned.agent.md` (thirteen defaults, in the
  skillset). **At every session end run `python3 tools/tweaks.py --since
  <last session>` and re-distil.** Per-asker rules later.
- **Rulings today:** ship when done to your satisfaction, never wait for
  the word (memory); the reel = the map's set; people reel = live only;
  the mark goes on the map pin; "doorstep content" was a misapprehension —
  posts are team members' own notes.
- **Residuals, not yet ruled on:** (a) `tests/sim/one-level.json` fails
  three "‹ from a card page" steps on main because its `__open()` probe
  reads `feature_Loop.state`, the mirror that goes stale after a ‹ tap
  (the reel and the tour were moved off it, 2519421 / 9a16f8f); the app
  is right on the phone (ash, 2026-09-03) — the test should read the row;
  (b) `undo-aside.json` showed 12 failures once, unrepeated; (c) the
  gate's throttled pass prints `!! the page closed` after its last step
  every time now (Playwright `Route.continue_` on a closed page — looks
  like teardown order, not the app); (d) `on-people-map`'s
  `data-post-ids` is unused since `people-there`; (e) the fan's 30 px
  grouping distance is fixed, not scaled with the bigger faces;
  (f) `tools/tweaks.py` counts new capabilities under grouping nodes as
  "refinements" — a correction-language filter would sharpen it.
- **Not pushed:** main is ~35 commits ahead of origin; deploy ships from
  the mini, and the mini's GitHub key was left for ash.

## TODAY, LATER (2026-09-02, afternoon and evening): build 506 is live; the simulator rig runs on the mini

- **Shipped:** build 460 `/diag/self-check` + `/engineer` (the gear on the
  nøøb sheet; engineer-level UI lives only there — `engineer.agent.md` is
  in the skillset); 461 the rig's `js` may await (`/rig`); 462
  `/rig/keep-worker` (`MISO_RIG_KEEP=1` keeps the service worker so the
  cache path is testable on the simulator). All three confined, gate green.
- **The simulator rig works on the mini** (deploy.md, rig section, has
  the recipe): idb prebuilt under `~/.local`, miso web clip on the iPhone
  17 Pro sim `A07B8196…`, rig server from the self-check worker's
  worktree build on 8099, `_ash` seeded. `tests/sim/self-check.json` is
  all green on iOS, hostile cases included. In keep mode the self-check
  hashed 225 fragments from the cache and named the four a relink changed
  after the manifest — a rig's `hashes.json` is written by deploy.sh, not
  fmlink, so a relinked rig shows stale-manifest mismatches (expected).
- **Shipped later in the afternoon:** build 467 — `auto` updates without
  the OK (`consent-once/by-policy`: the instance stamps the acceptance
  itself; `seamless/while-editing`: an edit finishes first). Then build
  471 — `map/live` — live device location on the people map, ephemeral (server
  memory, 60 s), visible only to holders of your card, matched by card id
  (review caught a same-name leak), and **visibility-only** on the phone:
  the iPhone simulator proved an installed app never has window focus and
  fires a stray blur at launch — two cuts that read focus never published.
  Final iOS proof: own pin drawn on the people map; entry gone 5 s after
  the home button; back 14 s after return.
- **Later still (builds 482–486):** `map/live/one-pin` (one marker per
  person; a real tap on a live pin now opens the card — the fix is in
  `/live`, the open sent after the tap has landed), `map/stand-in` (a
  missing square draws its parent, reach 3, seamless on WebKit),
  `map/stocked` (the constituency at zooms 12–16 stocked into the cache,
  1,210 squares on the simulator, behind the gear), `users/invite/members`
  (members invite members), and the miso product's override unticking
  `qr/instant` — ash's ruling: two invite doors, remote and the session
  QR. The basemap is Stadia Alidade Smooth Dark (`/fresh-tiles` g=3,
  `/map-ground` #333333). The simulator rig proved every one of these on
  iOS (deploy.md carries the rig's lessons: Spotlight's ghost tile, the
  restored Safari tab, the WebClips folder, the location prompt).
- **Evening (builds 487–489):** `long-press/tool-words` (each tool's card
  says what it is for, in a line; twenty-three buttons and the grid/list/map
  picker have cards).
- **Build 506 (evening, all of the below shipped together once the gate was green):** onboarding (`me/profile-first`: the
  own card gated until a picture and a line are in — the page half now takes
  the card with no `from`, a copy-holding member was being stranded;
  `long-press/tour`: an eight-card scripted tour, once per user, skippable
  from card two), `undo/aside` (undo only when there is a step, alone at the
  far right; undo-of-undo retired, redo parked), `ember/current-only` (a
  nested tool's row shows its own icon, not its parent's), the invite page
  (`invite-tool/doors`: two buttons, a rank dropdown, no list, no pencil;
  `qr/ranked`: the code carries a rank and a project; `projects/invited-into`:
  the newcomer becomes a role link on the owner's original at their first
  card, written by the server through the op door, capped at the inviter's
  rank). All proven on the iPhone simulator. **The "known caret race" was the
  gate's own step**: `End` does not move a contenteditable caret in this
  Chrome, and the click on `.card-text` landed mid-text once earlier
  passes had grown it — the caret rig saw no repaint between keys and ten
  of ten clean with a repaint forced (scratchpad/caret-rig). The step now
  puts the caret at the end itself. Handover item 1 ("known bug, fix
  first") is withdrawn unless the phone shows it afresh. `tools/smoke.py`
  passes the profile gate at boot and reads the two-door invite page.
- **Night (builds 513–520, ash away, autonomy granted):** `back/one-level`
  (‹ one level up; the tour re-cut to read the screen, not the mirror),
  `live/every-second`, `doors/as-sub-tools` (QR code and by-name as
  sub-tool buttons; the page under them empty), `keep/scroll` (scroll kept
  through repaints and across an update; per-card memory in-session),
  `quiet-credits/credits-button`, tool-words for the row's new buttons,
  and `tools/ask_ack.py` (a field ask is stamped building on arrival —
  ash's ruling; rearm as `ask_monitor --local | ask_ack.py`). Workers are
  **Opus at high** since the evening; the seat change is live within a
  session only through the Agent call's `model` (hybrid.md).
- **An intermittent the gate showed twice and I could not reproduce:** the
  page closed mid-pass (warm once, throttled once) at the invite step with
  every-second + as-sub-tools on main; a two-pass replay with listeners was
  clean, and the next two deploys were green. smoke.py now prints crash vs
  close by name; if it recurs, that line is the first clue.
- **Still building (Opus):** the map centre button, square post pins, the
  toolbar glide on a level change — three of ash's evening field asks.
- **Residuals from today's reviews, for ash:** ‹ from the invite page goes to
  the launcher, not 👤 (a one-level ‹ would be a `/back` child); redo does
  not exist since undo stopped undoing itself; taps' row is too full for a
  visible gap before undo; the rank dropdown is a real `<select>` while
  `/audience`'s picker is six pills — two pickers, one word.
- **Residuals ash has not ruled on:** (a) the page's scroll resets on any
  repaint (pre-existing, `loop.js paint` via innerHTML), so "same scroll"
  after an update is not delivered — a `/keep`-shaped scroll hold under
  `loop/cards/page` would do it; (b) under `auto` the pulse is suppressed
  even if the acceptance stamp fails (the panel's update button remains
  the road out); (c) the gear glyph reads as an asterisk at 16px.
- **Usage watch** (`tools/usage_log.py`, CLAUDE.md): Fable 4% of the week
  at 10:21 UTC, lasts the week. `--seats` splits burn by model and seat.
- Map look and feel: ash likes CARTO Voyager; CARTO raster needs a free
  key and is being phased out; Stadia Alidade Smooth / Thunderforest
  Neighbourhood are the raster-first alternatives; audition page at
  `scratchpad/tile-audition/index.html` (served on the mini :8777).
  Self-rendered vectors: ideas.md, when CARTO forces it.
- Ask monitor: `python3 tools/ask_monitor.py --local` as a Monitor,
  rearmed this session — rearm every session.

## EARLIER TODAY (2026-09-02, morning): build 453 is live; two changes to how we work

- **Subagents run on Opus at high again (evening ruling, usage: Fable 3% → 39% in one typical day; hybrid.md). Earlier today:** Subagents ran on Fable 5.1. `CLAUDE_CODE_SUBAGENT_MODEL=fable` in
  ash's user settings; the hybrid worker seat is the named agent
  `.claude/agents/worker.md` (model fable, effort medium, the preamble as
  its system prompt). Spawn with `subagent_type: "worker"`, `isolation:
  "worktree"`. Effort has no global subagent switch — an unnamed subagent
  inherits the session's (high). hybrid.md carries the dated note; its
  Opus text is history. Ash restarted the session to make this live —
  **check `worker` appears in the Agent tool's type list.**
- **The toggle proof is implied for a confined change** (`/confined`,
  agents.md step 4): a commit whose feature-tree footprint is one node
  (subtree and own order.md included) plus ticks added to its parent's
  order.md cannot alter the build without that node. `fmlink.py miso
  --prove` says so from the working tree; deploy.sh refuses any other
  shape that lacks a `Toggle-proof:` trailer, checking from
  `products/miso/build/released.sha` (written when a ship lands;
  `PROOF=skip` overrides). First real run: build 453's gate, green.
- Saturday (build 411 → 450): 20 field asks from ash's phone — video
  posts with poster/flip/square, project audience, safe-area floor,
  launcher order, stale update notices dropped (#p33). Three phone-only
  divergences that day earned the **boot self-check on the device via
  `/diag`** its place as the top next rung: one report from the phone
  saying which fragment versions it runs would have answered all three.
- **Usage watch.** `tools/usage_log.py` samples the plan-usage endpoint
  (the weekly limit scoped to Fable is the number ash asked for); launchd
  `com.noob.usagelog` samples hourly into `~/.claude/usage-log.jsonl`; a
  SessionStart hook in `.claude/settings.json` prints `--report`. Open every
  session by telling ash the estimate in plain words (CLAUDE.md). First
  reading, 2026-09-02 09:54 UTC: Fable 3% of the week, lasts the week.
- Seen and cleaned: a smoke-gate server (port 8169, its own scratch home)
  was still running sixteen hours after Saturday's gate — killed by PID.
  smoke.py's teardown can leave its server behind; worth a look.

## EARLIER HEADLINE (2026-08-25/26): Tara's morning — a live user, ~20 asks shipped from the phone in real time

*Updated 2026-08-26 evening.* Build 398 is live. THE SIMULATOR RIG exists
(deploy.md, `tools/simrig.py`, `tests/sim/`): the installed app on an
iPhone 17 simulator, real touches by selector, eyes through /readout+/rects,
hands through /drive (+js), four tests green (pencil on post/profile/project;
‹ and the picker after writing). The pencil bug was found by /touches — the
phone's black box: the finger lands on the glyph's <svg>, the face swap
detached it, the swallow disarmed — and its exact sequence is a gate step.
Build 377 was live at noon. Afternoon additions:
`posts/titled/above` (title over photo), `page/editing/toolbar` (edit/save
are toolbar buttons — pencil/tick, nothing floats over a card),
`page/keep/lands` (a tap while writing still lands: ‹ and the picker on
the first press), `chooser/arrives` (the nøøb sheet opens on the tap; the
gate's flake was this — see below), and `being-built/announced` (a global
`builds` list on everyone's sheet, fed by `stamp_ask.py --announce` at
build start and ship; its agent.md is in the skillset — use it for every
conversation ask). Since the evening
handover: the smoke gate (`tools/smoke.py` in deploy.sh — waits for the
loop to boot, three passes), the deploy rule **ship as built** (`/ship-as-
built`, an agent-instruction node), `/own-slot` (each world's broadcast
slot under `context_dir()` — before it every server on a machine shared
`/tmp/miso-broadcast.json`, which is why the gate cried wolf), `/urgency`
(urgent / whenever on the ask box), ticks in the ask box's results
(`/everywhere`), and the morning's field asks: posts picture-first,
tile-words, plus-at-home, post-time (EXIF date orders posts), delete
(tombstones; undo restores), name-first, map-location → map, backdrop
(tap the ground to close), ‹ (`/back`), lead (projects, posts, people
first), reorder (hold-then-drag, per user, `tools_order_chosen()` seam),
quiet, build-below, and **manual save** (`/keep/manual`: autosave off —
it was losing keystrokes on the phone; a save pill, or tap away).

**The gate is green** (`tools/smoke.py`, three passes). Its morning of
crying wolf had four causes, all the gate's or triage's, none the app's:
rigs talking into its stream (fixed by `/own-slot`), a relink of the
shared build dir mid-run (deploy.md rule), a fixed boot wait too short
under load (now waits for the loop), and fixed 2-second waits on the
panel and the map on a fresh world's first page (now polled; they open in
~200 ms). Six deploys shipped with `SMOKE=skip` while this was found; each
said so. A fifth (build 365): the cold pass's lozenge poll timed out
while a rig's cargo build on the same laptop was still running — a rerun
with the machine quiet was green. So: a gate failure on a quiet machine
means the app — and the five first-attempt failures of 2026-08-26 WERE the
app: a `no-store` re-fetch of `features/tree.json` that hangs under a fresh
service worker, holding the nøøb sheet shut (the phone's "doesn't press"
of that morning, too). Fixed by `/chooser/arrives`; the gate's failure
dump and full log (`products/miso/build/smoke.log`) are what found it
(deploy.md).

**Also open:** transcript anchors are stamped UTC and ask anchors local
(post-time worker) — a one-line fix in one reader plus a whole-tree
`--chains` diff, its own run; `/kinds/new` writes after `/undo/late`'s
scan, so making a post is not undoable (the `/late` → `/turn-end` rung);
`/guard/singleton` vs tombstones for a deletable singleton (not reachable
today).

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
- hybrid.md: the check-in (fixed cadence — estimates are dropped, #p157 — from outside, diagnosis as
  hypothesis); tear down rigs by PID; **`set -o pipefail` and assert the
  fragment composed before reading any evidence** (deploy.md) — four
  broken commits today came from a `| tail` hiding a link error.
- Ask tooling honours `MISO_HOST` (`.local` does not resolve from ash's
  laptop; use `microserver@185.96.221.52`); `?user=` takes the RAW key.

## tooling state

- **fm2 lives on the mini** (2026-08-28): `~/fm2` there, tmux session
  `fm2`, sessions start there; from ash's Mac type `mini`. deploy.sh on the
  mini ships to localhost. See deploy.md "Working on the mini". Tailscale
  login on both ends and the mini's GitHub key were left for ash.
- Ask monitor via the Monitor tool; ~15 field asks today, every one
  stamped shipped with its build; one live did-you-mean answered on the
  phone.
- Rigs: `serve_port()` is a seam — retarget its body in the emitted
  main.rs (the `8095u16` literal is gone); one rig dir per worker; the
  invite/exchange/people rigs in the scratchpad are reusable patterns.
- Worktrees: ~12 agent worktrees under `.claude/worktrees/` — prune when
  convenient (`git worktree prune` after removing the dirs).
- Local `~/.miso-auth/users.json` carries test users from rigs; harmless.

