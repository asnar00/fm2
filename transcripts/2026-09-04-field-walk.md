# transcript: the field walk
*session `b70435b4-6806-4a25-aadb-e68df3da3413`, exported by tools/export_transcript.py — do not edit; anchors `#pN` are stable*

### p1
*2026-09-04 08:36*

> morning claude! It's friday, and tomorrow is our first miso field test with the sevenoaks canvassing team!

Morning ash! First, the usage watch: Fable is at 57% with 3.8 days to the reset, and at the last three days' burn it runs out in about a day and a half, around Saturday evening. That is a squeeze with a field test tomorrow, so I'll keep this session lean. The all-models budget lasts the week.

Let me read the handover and doctrine before anything else.

Handover says today is the walk-and-record day and Saturday is the team. Let me check the live system, the ask queue, and re-arm the ask monitor.

No pending asks, build 614 is live, and the server is up. Now I'll re-arm the ask monitor as a background Monitor and check the live site responds through the tunnel.

### p1a
*2026-09-04 08:37*

> <task-notification>
> <task-id>boxew0c6u</task-id>
> <summary>Monitor event: "field asks from the phone (stamped on arrival)"</summary>
> <event>BACKLOG asked user=phone:+447813943023 t=1788503662808 tool=- at=-
> text: Let’s remove and delete the still image and audio posts
> proposal: -
> watching /Users/microserver/.miso-context (5 worlds) for asked,proposed</event>
> If this event is something the user would act on now, send a PushNotification. Routine or benign output doesn't need one.
> </task-notification>

A field ask just landed from your phone: "remove and delete the still image and audio posts". It is stamped building. First let me see why the live probe is slow.

Live is up through the tunnel, build 614, wifi steady. Now the ask. I'll read the discipline loop, the post kinds in the tree, and count the still and audio posts that exist.

Still working the ask: I'm tallying which posts in each world are still-image or audio before deciding how to delete them.

Next I need the tool's constants, a baseline relink to diff the untick against, and a dry run of the delete. All three go now.

The dry run names the same four posts and their nine copies. Writing the tombstones now, and in parallel materialising the product override to untick the photo kind.

The proof says implied and the photo fragment left the site. Let me check the other differing files are only provenance comments, and why git shows the new directory oddly.

Committed. Now the deploy, which runs the smoke gate and ships build 615.

While that runs, nothing else is needed until it finishes. Here's where things stand so far.

**Usage:** Fable at 57%, and at the last three days' pace it runs dry around Saturday evening, before Tuesday's reset. I'll keep today lean so there is budget left for the field test.

**Live system:** build 614 is up through the tunnel, the server answers locally, the wifi watchdog logged quiet OKs overnight, and the ask monitor is re-armed.

**Your field ask** ("remove and delete the still image and audio posts") is done on the data side and the code side is deploying:

- Four of your posts were not videos: two audio tests from mid-August, "drinkie poos", and a photo-only post from Tuesday. Each is now a tombstone in your world and in the three worlds holding copies, the same shape the bin makes, so your phone's older copy cannot bring them back. The recordings stay on disk and the op log keeps the old lists. The tool is `tools/prune_posts.py`, dry by default.
- The miso product now unticks the photo kind. Audio has no node of its own, since it is the dictate recorder's own post, so it stays in the build but nothing can make one under video-only.

The deploy is running the gate for build 615. I'll stamp your ask shipped when it lands.

### p2
*2026-09-04 08:48*

> <task-notification>
> <task-id>bt1cecazl</task-id>
> <tool-use-id>toolu_01LGe41cvtVo9UX3L7CwPbSi</tool-use-id>
> <output-file>/private/tmp/claude-501/-Users-microserver-fm2/b70435b4-6806-4a25-aadb-e68df3da3413/tasks/bt1cecazl.output</output-file>
> <status>completed</status>
> <summary>Background command "Deploy build 615 through the smoke gate" completed (exit code 0)</summary>
> </task-notification>

Shipped. Build 617 is live, the gate was green on every pass, and your ask is stamped shipped on your phone's sheet.

**What changed**

- The four non-video posts are tombstoned in every world. Your phone will drop them on its next sync, since its older copies lose to the newer tombstones.
- The photo kind is unticked in the miso product, so its script and switch have left the build. Audio stays composed but unreachable, because it has no node of its own to untick.

**For the walk today**

- Every add now records a video, and the posts you see should all be videos.
- If a post you deleted reappears on the phone, tell me, because that would mean the phone's copy carried a newer edit stamp than the tombstone.

**Budget note:** this run cost little. Fable is still the constraint, so I'll wait for asks rather than poke at residuals.

### p3
*2026-09-04 08:53*

> what are the residuals?

From the handover, the things left open. None blocks tomorrow, but a few touch it.

**Worth a look before Saturday**

- **The audience floor.** Your newest posts were filed at the admin grade, so a candidate or canvasser cannot see them. That was by design, but the default floor for a post may want to be team, or the canvassers will see an empty map.
- **Test rows on the live guest list.** `_alice`, `_bob` and Tara's placeholder-number row are still there. Reset before the session, or leave them as the demo people.
- **The mini is on wifi.** The watchdog has been quiet, but a cable is still the right answer for a day the whole team depends on it.
- **Joining a project seeds profiles only.** A newcomer gets everyone's card but a project's posts still depend on each post's floor. That is the same issue as the first bullet, seen from the other side.

**Known and parked, no field effect**

- The people page ranks a co-member as unknown, and leaving a project keeps the copies.
- Copies of cards keep their old inline pictures until the owner next edits; the phone's picture store is never pruned.
- The broadcast slot is re-parsed five times a second by every waiting phone, so its cost grows with list size times people. Twenty phones with big lists is the first real test of that.
- The flush routine shifts the outbox head by position after an await, a latent bug nobody has hit.
- The mission flash's focus after the toolbar blur was never verified, though the flash itself was.

**Tooling only**

- One simulator test reads a stale mirror after a back tap and fails three steps that the phone gets right.
- The gate prints "the page closed" after its last step every run, a teardown-order thing in Playwright, not the app.
- The proof checker calls a parent refactor plus child "implied" when it should not.
- The tweaks digest counts new capabilities as refinements and wants a correction-language filter. Last night's eight asks are not yet distilled, and today's and tomorrow's will join them.
- Main is around 40 commits ahead of origin. Deploys ship from the mini so nothing depends on the push, but a GitHub key on the mini would close it.

If you want one thing fixed today, I'd pick the floor. Tell me which grade a new post should default to and it is a small node.

### p4
*2026-09-04 08:55*

> audience floor: I want all my test posts to be admin-only - I want volunteers, once they sign up, to only see actual campaign related posts made by Tara, since she's the leader; we'll look at the promote workflow today. We'll reset users before we go out for real. I will buy an ethernet cable today! A newcomer should only see posts that have been promoted to their level.

Noted. Let me check what the audience node does today so I can say whether the current defaults already match your rulings, and write them into notes.md.

Good news: the app already does exactly what you ruled, so nothing needs building.

**How the floor works today**

- A new post is stamped at its author's grade. Yours land at admin, so they are admin-only. Tara's land at candidate.
- Promote lowers the floor one rung at a time, and only the author can press it. Tara promotes a post down to volunteer and every volunteer sees it at once.
- A reader sees only posts whose floor is at or below their grade, so a newcomer sees nothing until something has been promoted to their level.

**What that means for today's walk**

- Tara's posts start at candidate, so a volunteer sees none of them until she promotes each one. If she wants everything visible to the team by default, that is a one-node change: stamp her posts at team instead of her own grade. Tell me if you want it.
- Promote goes one rung per tap. From candidate to volunteer is three taps: team, then volunteer. If that is too slow in the field, a "promote to" picker would be the ask.

I've written the rulings into notes.md and committed. Users get reset before the real session, and the cable is on you.

### p5
*2026-09-04 08:59*

> I'm thinking that in the posts toolbar, we need an "options" button that pops up a general options page: select front or back camera (default selfie), post level (default same-as-me, but select any level from an option list).

That's a field ask, so it builds now. First I place it in the tree: I need the posts tool's children (it is at the six-child cap), the sub-tool button pattern, the camera flip node, and the audience default seam.

A worker is building it now on Opus, in its own worktree and rig, so it costs no Fable budget while it runs.

**What it is building**

- A new node under capture, beside the add button, video and flip. It is called options.
- A sub-tool button in the posts row that opens the options page one level down. The back chevron returns to posts.
- Two rows of pills on that page. Camera: front or back, with front now the default. Post level: same as me, or any grade from admin down to public. A chosen level is the floor a new post is stamped at, and promote still lowers it afterwards.
- A volunteer cannot pick a level above their own rank. I told the worker to clamp it and to say so.

**Proof it must bring back:** the smoke gate green, and the real tap path on the iPhone simulator: posts, options, pick back camera, back, add, the viewfinder asks for the rear camera; then pick a level, make a post, and read its floor from the rig. Plus a screenshot judged against the taste standard.

When it returns I review the diff myself, ship it, and stamp the build shipped on your sheet. Your posts and the walk are unaffected meanwhile, since the live server is untouched until then.

### p6
*2026-09-04 09:18*

> while that's running: the posts interface (map view) has a number of issues that make it feel janky. 1) when you tap a reel lozenge, I'd like the post to animate open from the lozenge to the full card, so it's clear that it's the same thing; 2) the post opens, but then the video flashes a couple of times and auto-plays, which it shouldn't. It should just come up without flashing and wait for you to hit play. 3) swiping up and down should scroll the card physically upwards/downwards and the next/prev card should scroll properly in; 4) closing the post should animate the card area down to its reel lozenge (which should have scrolled left/right if you switched which post you were looking at, and focused on the map appropriately).

Four distinct refinements, so four nodes. Let me check the child caps on the reel and flick nodes before I place them.

A second worker is on the four map-view fixes, in its own worktree, while the first keeps building the options page. Both are on Opus, so Fable spend is only my reviews.

**How I placed the four**

- **Opening animation:** a child of the card-over-map node. The card grows from the tapped lozenge's rectangle to full size, quick and quiet.
- **No flash, no autoplay:** a child of the face-first poster node. The worker diagnoses the flashes first on the rig, since each repaint on arrival of the clip, poster or location currently swaps the player element, and there is an explicit play call on open. The cure is a stable poster and play only on your tap.
- **Physical swipe:** a sibling of the touch flick node. The card tracks the finger, springs back short of the threshold, and the next card scrolls in past it. The existing flick still does the switch underneath.
- **Closing animation:** a second child of the card-over-map node. The card shrinks to the lozenge of whichever post is current after your swiping, with the reel scrolled to it and the map panned to its pin.

**Order:** the flash bug first, then open, swipe, close. Each ships as its own commit with its own proof on the simulator, including rect readouts through the transitions and a count of play calls after opening, which must be zero until the tap. All four are announced as building on your sheet.

I'll review each worker's diff myself when it returns, then deploy and stamp shipped.

### p7
*2026-09-04 09:25*

> next: upload and transcription. The quality of the on-device transcription is really poor, so let's not do that. Instead, let's do the following: 1) stream audio and video to the server as we're recording it (or to a local cache with an upload queue if we're on a slow/nonexistent connection) 2) do transcription using the best available on the mac mini [look at the fieldnote project - that method got decent results; and maybe look online for other options also]; 3) seed the transcription with words taken from a context document based on our location (streetnames) and maybe a later a briefing document; 3) as soon as the post is complete, it should appear on other users' grid/list/maps, as appropriate to the publishing level.

A big one. Before placing it I need to know what the dictate node does today, what the server road already is, what fieldnote used, and what the mini has installed.

### p8
*2026-09-04 09:25*

> dictate is deprecated - we're just recording video notes now.

Understood: video notes only, and the on-device road goes. Let me see

### p9
*2026-09-04 09:25*

> dictate is deprecated - we're just recording video notes now. let's make a design, I'll approve it, then we can delegate building

Design first, then. I'll gather the facts the design rests on: how a clip reaches the server today, what transcribes it, what fieldnote used, and what the mini has installed.

I have what I need. Writing the design into notes.md now, then the summary for you to approve.

The design is written into notes.md under "video notes to the mini" and committed. Here is the short form for your approval.

**What exists already**

- The phone records into one blob at stop and uploads it whole afterwards, retried when the connection returns. That is the upload queue, just coarse.
- The dictate node has three transcription rungs as empty slots: local, server, api. Only local is ticked. The server slot is waiting for a node.
- A landed transcript is written into the post's words as an edit, so it already travels to everyone at the post's floor. The post itself travels at the mint. So part 4 is mostly there and needs proof, not a build.
- The mini is an M2 with 8.6 GB of memory. Whisper large-v3-turbo for MLX, fieldnote's road, is already in its model cache. No whisper package is installed. ffmpeg is.

**The method**

- **Recommended: mlx-whisper large-v3-turbo on the mini**, one resident worker with the model warm, one clip at a time. It is on disk, it takes a seeding prompt, and it runs well over real time on an M2.
- **Parakeet** is more accurate on clean English and lighter, but it cannot be seeded with words, which kills your street-name idea. **WhisperKit** is the fastest whisper runtime on Apple silicon and a drop-in upgrade later behind the same seam. **Cloud APIs** are better on accents and noise but send the team's notes off the mini, so they stay as the api rung, not the default.

**The nodes**

1. **Streamed recording.** The recorder cuts a chunk every two seconds and posts each as it arrives. The chunks also stay on the phone, so offline or slow they wait and upload oldest first. The server joins them. The companion audio recorder the phone runs today goes, since the server pulls the audio track itself.
2. **The mini rung.** A resident transcriber watches a queue, extracts audio, runs turbo with silence trimming so it does not invent words, and lands the text through the server. Every instance sees the words arrive.
3. **Seeding.** The prompt is "Canvassing in Sevenoaks. Streets: the thirty nearest. Names: the project's members." Street names come from one Overpass pull inside the constituency boundary, stored on the mini, offline afterwards. A briefing block on the project card is a later child.
4. **The phone's own whisper is unticked.** That takes 133 MB out of the app and a model off the battery.

**Tripwires named for the builders:** iOS may hand over chunks only at stop, which degrades cleanly to today's upload; a joined video that will not decode gets rewrapped; memory on the mini is measured, never two models; a transcript for a deleted post is dropped.

Sources: [Parakeet vs Whisper](https://spokenly.app/blog/parakeet-vs-whisper), [why turbo over Parakeet in production](https://www.arunbaby.com/speech-tech/0073-whisper-vs-parakeet-asr-decision/), [Whisper and Parakeet on Apple silicon](https://macparakeet.com/blog/whisper-to-parakeet-neural-engine/).

Say yes, or change anything, and I brief the workers. The two earlier workers are still building.

### p10
*2026-09-04 09:30*

> yeah that sounds like a decent start. Parity with fieldnote would be good since I know that works OK in the field.

Approved. Fieldnote's code is not on the mini as far as I found, so let me look once more for it before I brief, since parity means copying its exact settings.

### p11
*2026-09-04 09:31*

> continue

Fieldnote had two pipelines, and the one that ran in the field was Speechmatics, not local whisper. Let me confirm the key and the geocoding it used, then brief.

A third worker is on the transcription work. Three are now building in parallel, all on Opus.

**What I found about fieldnote, which changed the design**

- The pipeline that ran in the field was **Speechmatics**, cloud batch, enhanced model, speaker diarization, with a vocabulary list built from the reverse-geocoded address, ward, district and constituency. The local whisper road was its earlier version.
- Its key is in fieldnote's env file, and its venv on the mini still runs mlx whisper and pyannote. So both roads work today.

**The amended design, now building**

- **Speechmatics is the top rung**, fieldnote's exact configuration. That is the parity you asked for.
- **Local whisper turbo is the fallback rung**, taking over when the key or the network is absent. The scheduler already grades rungs and upgrades a transcript in place when a better one lands.
- **One context node feeds both** with the same phrases: fieldnote's geocoded address plus the nearest thirty streets from a one-time Overpass pull of the constituency.
- **Streamed upload** cuts two-second chunks and posts each as it is made, with the phone keeping them for the queue. The worker must report what iOS actually does, since Safari may only hand over one chunk at stop.
- **The phone's own whisper is unticked**, taking 133 MB out of the app.
- **The two-user proof** is required: a post on the second user's world within a second of the stop, then the words following.

The key will move to the mini's agent config file, never the repo. The transcriber's launchd job gets written but not loaded until I ship it.

I'll review each worker's return myself, then deploy in the order they finish. Nothing on the live server changes until then, so keep recording.

### p12
*2026-09-04 10:13*

*(edited and resent — superseded by the next prompt; do not cite)*

> ok, superb. next batch of work: I want to lose the grid/list views and standardise on map view for everything - the "reel" feature, coupled with smooth open/close/scroll, does everything we need. So let's remove the grid/list/map switch, and replace it instead with a time-domain filter: options are "today", "week", "month", "all". Today just shows posts made today, week shows this week (from monday as day 1 of the week), month shows all this month's, all shows all.

### p13
*2026-09-04 10:13*

> ok, superb. next batch of work: I want to lose the grid/list views and standardise on map view for everything - the "reel" feature, coupled with smooth open/close/scroll, does everything we need. So let's remove the grid/list/map switch, and replace it instead with a time-domain filter: options are "today", "week", "month", "all". Today just shows posts made today, week shows this week (from monday as day 1 of the week), month shows all this month's, all shows all. This applies to all the other views as well (users, projects).

Two nodes: one that makes the map the only view and drops the switch, one that puts the time filter in its place. Let me see the browse node's shape before I place them.

A fourth worker is on it. Four are building now.

**How I placed this batch**

- **Map only** is a sixth child of the browse tool. Every browse tool opens on the map, the grid and list become unreachable, and the picker draws nothing. The grid and list nodes stay in the tree so the untick still works. I told the worker to check what happens to a card with no place, since those may only have been reachable through the grid, and to give them the reel if so.
- **The time filter** is a child of map-only, filling the slot the picker vacated. Four pills, today, week, month, all. Posts filter by their taken time, people and projects by creation. Week starts Monday, boundaries are the phone's local midnight.

**One call I made for you:** the default is "all". A newcomer on a quiet day should not open an empty map. Say the word if you want "today" as the default for the field.

The worker is told to keep clear of the reel and flick files the other worker is changing, and to rebase on main before it returns. I'll review and ship each as they land.

### p14
*2026-09-04 10:19*

> next group of work: for the "add post" button, the "+" button shouldn't start recording right away. Instead, it should open a toolbar that contains buttons "rec" [create new post and start recording], "stop" [finish and file post], "camera-flip" [change camera], "publish level" [pops up publish options]. Those settings are persistent, so you set them as often as you like, but you get a bit of time do that.

This reshapes what the first worker is building, so rather than letting it finish a row button you will then replace, I'm redirecting it now.

Redirected. The first worker now builds the recording row instead of a row button, keeping the settings it already has.

**What the + does now**

- Tapping + opens a recording row one level down. The back chevron returns without minting anything.
- The row has four buttons: rec mints the post and starts recording, stop finishes and files it, camera flip switches front and back, publish level pops up the level picker.
- Camera and level persist, so you set them whenever you like and they hold across posts.
- While recording, rec is out and stop is live. Stop with nothing recording files nothing.

The proof is the real tap path on the simulator: plus, flip, level, rec, stop, then the post exists at the chosen floor. And plus then back leaves no post behind.

Four workers are still building. Nothing changes on the live server until I have reviewed each and shipped it.

### p15
*2026-09-04 10:22*

> what was the audition link for map tiles again?

It is still up on the mini's port 8777:

```
http://192.168.1.76:8777/
```

or `http://microservers-Mac-mini.local:8777/` from a machine on the same wifi. The page is `scratchpad/tile-audition/index.html`. It is local network only, not through the tunnel.

### p16
*2026-09-04 10:23*

> can you print the stadia API key here? or better still, plug it into the audition system automatically

Done. The audition page now has your Stadia key pre-filled in its key field, and the served copy on port 8777 carries it, so the Stadia and Stamen styles will load on a reload. The page lives in the scratchpad, which is gitignored, so the key stays off the repo.

Same link as before:

```
http://192.168.1.76:8777/
```

### p17
*2026-09-04 10:27*

> OK. next batch of work (hopefully parallel with the first): I'd like to display the constituency / ward boundaries slightly differently. (if possible): to have a selected map region (either the whole constituency, or just a single ward - selectable under the posts toolbar maybe- show using "stadia outdoors", and outside the region show in the current look. So we'd need to "mask" the current region where the boundary intersects a tile. Also, let's not print the ward labels for all wards, as they make the map look messy when zoomed out; instead, we'll show ward boundaries as black outlines.

Let me look at the boundaries node and how the map draws tiles, then place and brief.

A fifth worker is on the boundaries. Five run in parallel now.

**How I placed it**

- **Outlined:** a child of the boundaries node. Ward names go, wards become black outlines, the constituency edge keeps its own weight. Built first, since it is quick and visible.
- **Region:** a second child. A second ground, Stadia Outdoors, served through the app's own tile cache under its own route with the key from the environment, never the repo. A second tile layer sits above the current ground, clipped to the chosen polygon, recomputed on zoom. The worker must show a crisp edge on a tile the boundary crosses, on WebKit, or stop and report rather than ship a soft one.
- **The choice:** a region sub-tool button in the posts row, opening a page of pills, the constituency first then each ward by name. It persists per user, constituency by default.

The worker is told the Stadia free tier is metered and to stock only what the proof needs. It keeps clear of the reel, flick, time filter and recording-row files the others are changing.

I'll review each of the five returns myself and ship them in the order they land.

### p18
*2026-09-04 10:30*

> this feature modular thing is pretty good for parallel workers, right?

Yes, and today is the clearest proof so far. Five workers on one codebase, and the only coordination I had to write was "don't touch these files", because each ask maps to one node and a node owns its files.

What makes it work:

- **One node per ask** means one worker per node with no shared edit surface. The tree does the partitioning that a normal codebase needs a lead to do by hand.
- **Extension by redefinition** means a worker adds behaviour without opening a parent's file. When it must open an extension point, the refactor is a named, proven step rather than a merge conflict waiting to happen.
- **The toggle proof is per node.** Each worker proves its own node in isolation on its own rig, so I can review and ship them in any order they land.
- **The linker catches overlap.** Two nodes redefining the same function without a chain fails at link time, not in the field. That was the "siblings at one anchor" miss, and it is now a rule in every brief.

Where it still strains:

- **Shared order files.** Two workers ticking siblings in the same product override will conflict at merge. Today I told them to add their lines minimally, and I will resolve by hand.
- **Child caps force regroups**, and a regroup moves addresses for everyone. I placed three of today's nodes one level lower than they naturally sit to avoid one.
- **Cross-cutting asks** like "map for everything" still touch one node's base file, so that worker gets the parent and the rest get told to stay out.

The real test is the next hour, when five branches land on main and the gate runs on the union rather than on each alone.

### p19
*2026-09-04 10:31*

> can't wait ;-)

Nor me. I'll ship each one as it lands and tell you what changed on the phone in plain words, with the first union gate result when the branches meet. Keep recording.

### p20
*2026-09-04 10:31*

> I'll do a test walkabout once we've landed everything

Good plan. I'll message you here when the last of the five is live and the gate is green on the union, with a short list of what to try on the walk. Until then nothing on the phone changes.

### p21
*2026-09-04 10:36*

> <task-notification>
> <task-id>a42bf4bae1c8c7811</task-id>
> <tool-use-id>toolu_01RnF55yn2fynEX3kRUdYgSf</tool-use-id>
> <output-file>/private/tmp/claude-501/-Users-microserver-fm2/b70435b4-6806-4a25-aadb-e68df3da3413/tasks/a42bf4bae1c8c7811.output</output-file>
> <status>completed</status>
> <summary>Agent "Build video notes to the mini" finished</summary>
> <note>A task-notification fires each time this agent stops with no live background children of its own. The user can send it another message and resume it, so the same task-id may notify more than once.</note>
> <result>All six nodes are built, proven and committed. Here is the outcome.
> 
> ## What shipped (branch `worktree-agent-a42bf4bae1c8c7811`, 6 commits on `d5fb143`)
> 
> | commit | node |
> |---|---|
> | `8e8aadf` | `features/miso/loop/dictate/transcribed` — the plumbing |
> | `50771a3` | `transcribed/vocabulary` + `tools/streets.py` |
> | `e1f6d87` | `transcribed/api` — Speechmatics, grade 3 |
> | `466597d` | `transcribed/mini` + `tools/com.noob.transcriber.plist` |
> | `40f4c3a` | `capture/video/streams` |
> | `247deb8` | `/phone` unticked for miso + a deploy.sh guard |
> 
> **Two deviations from the brief, both forced:** the node the brief called `transcribed/context` is `transcribed/vocabulary` — `context` is a tree-global name already held by `miso/loop/context` (misses.md, "the second `ground`"). And the parity approval is `field-walk#p10`, not `#p8`: `#p8` is "dictate is deprecated". notes.md's amendment cites `#p8` and is now wrong by one — worth a one-line fix by triage. `transcripts/2026-09-04-field-walk.md` was uncommitted and stopped at `#p6`, so I re-exported and committed it; triage's own export at session end will extend the same file.
> 
> **One design change I made:** the rung ladder is walked by `/transcribed` (`transcribe_rung(job)` carries a `want` grade and a rung answers only for its own) rather than by each rung standing aside. Same-anchor siblings load in name order, so no rung may assume it is outermost; this way each is tried exactly once, in order, with an explicit fall-through.
> 
> ## Proven
> 
> - **Full pipeline, twice, on my rig (port 8112, own scratch HOME):** four parts posted to `blob/&lt;id&gt;/part/&lt;n&gt;` (HTTP 200 each) → joined **byte-identical** to the source (89,031 bytes) → seeded with a **494-character** prompt built from the live geocode + streets → transcribed → words on the author's card in **18 s / 7 s** → on the second user's copy at the next read. Script: `/private/tmp/claude-501/-Users-microserver-fm2/b70435b4-6806-4a25-aadb-e68df3da3413/scratchpad/a42/server_e2e.py`.
> - **A real defect, found and fixed:** `/exchange` hands a changed card on from its **route** link, watching the caller's cards across one `POST /msg`. A transcript landed on a background thread has no request and no cookie, so **the words stopped in the author's world** — the second user watched an empty note. Observed on the rig (`_tara words after 122s: ''`), fixed by taking the same two reads and calling `exchange_share` in `transcribed_land`, re-proven green.
> - **Speechmatics vs whisper, one real call:** with the seeded vocabulary Speechmatics wrote **"Bligh's Road"**; mlx turbo wrote **"Bly's Road"** both seeded and unseeded. That is the whole argument for grade 3. One job, submitted and deleted, ~6 s wall clock on 9.9 s of audio. I did not query billing, so I have no cost figure — only the duration.
> - **Memory, measured:** 1,543 MB held in MLX + 1,332 MB peak resident while loading. `ps` reports 37 MB because Metal's unified memory is not counted as resident — so the heartbeat carries both numbers and the spec says why.
> - **Smoke gate green** (all three passes) at the tree state of node 1 (8142), the rungs (8144), streams (8145) and the phone untick (8146). Nodes 2 and 3 were not gated individually; node 4's state contains them.
> - **Toggle proofs:** node 1 and node 5 are full untick/relink/diff proofs (node 5's shows `index.html` byte-identical apart from `streams.js`, i.e. the seams opened in `video.js` and `mirror.js` change nothing alone); node 6 *is* an untick; nodes 2–4 are `--prove`-implied.
> - **4a:** `/private/tmp/claude-501/-Users-microserver-fm2/b70435b4-6806-4a25-aadb-e68df3da3413/scratchpad/a42/evidence/post-transcribed.png` — the transcript sits in the post as ordinary words, no rung badge, no machine-wrote-this chrome. Good enough; I would change nothing inside this footprint.
> 
> ## Not proven — blockers, not skips
> 
> - **MediaRecorder was never exercised.** This box's headless Chrome refuses the fake audio device (`NotReadableError: Could not start audio source`) with a correct 16-bit 48 kHz wav, so `getUserMedia({video, audio:true})` fails and no recording happens. The **iOS timeslice question is therefore unanswered** — I cannot tell you what iOS actually did. The simulator is the instrument and **another worker held it all session** (`/tmp/miso-readout.json` live, iPhone 17 Pro on port 8098; readout and drive are single-tenant).
> - **The joined MP4 was never played back.** The join is proven byte-identical to a valid source file; that it decodes as a container assembled from real MediaRecorder chunks is a hypothesis.
> 
> ## Open risks, by name
> 
> 1. `tools/deploy.sh` is outside the brief's footprint. Without the change it would refuse every deploy demanding 133 MB of model nothing composes; the guard runs the check only while `/phone` is ticked for the product.
> 2. The launchd plist is **written and not loaded** — putting a 1.5 GB resident model beside the live server is triage's call. Until it is loaded the mini rung is unreachable, `transcribe_best_grade()` is 0 (no key in the live server's HOME either), and **nothing is queued and nothing is transcribed** — that is the correct silent state, not a bug, but it means shipping this changes nothing until the worker runs.
> 3. `tools/streets.py --go` has not been run against the live `~/.miso-context` (I only wrote my rig's copy). Without it, the vocabulary is the geocode alone.
> 4. The api rung's script is served at `site/tools/transcribe_api.py` — publicly readable. It holds no secret; the key is in `~/.agent-config.json` (mode 600, `speechmatics.api_key`, copied from fieldnote's `.env`, not in the repo).
> 5. `/transcribed` now calls `exchange_share` and `exchange_cards_of`, so it will not link with `/exchange` unticked. Deliberate — a landing that cannot travel is not a landing — but it is a new coupling.
> 6. `transcribed_stamp` re-enters `handle_msg` from a drain thread; that path is `/reports`' and is proven, but it is a thread doing what a request normally does.
> 7. The `!! the page closed` teardown noise made one gate run look wedged; it had in fact passed. Pre-existing (handover residual c).</result>
> <usage><subagent_tokens>482052</subagent_tokens><tool_uses>461</tool_uses><duration_ms>3829244</duration_ms></usage>
> <worktree><worktreePath>/Users/microserver/fm2/.claude/worktrees/agent-a42bf4bae1c8c7811</worktreePath><worktreeBranch>worktree-agent-a42bf4bae1c8c7811</worktreeBranch></worktree>
> </task-notification>

The transcription worker is back with all six nodes. I'm reviewing its diff myself before shipping anything.

### p21a
*2026-09-04 10:36*

> <task-notification>
> <task-id>a42bf4bae1c8c7811</task-id>
> <output-file>/private/tmp/claude-501/-Users-microserver-fm2/b70435b4-6806-4a25-aadb-e68df3da3413/tasks/a42bf4bae1c8c7811.output</output-file>
> <status>completed</status>
> <summary>Agent "Build video notes to the mini" finished</summary>
> <note>A task-notification fires each time this agent stops with no live background children of its own. The user can send it another message and resume it, so the same task-id may notify more than once.</note>
> <result>That was a stale background wait from before I retook the screenshot — its `page text: no card page` is the *first*, failed attempt, superseded by the successful retake I already reported. It adds one confirmation and nothing else: the node-6 gate line reads `smoke: all passes green`.
> 
> Nothing in my report changes. Final state:
> 
> - Working tree clean, HEAD at `247deb8` — all six commits in.
> - Both my processes are down (pid files gone; they were stopped by their own PIDs after `ps` confirmed the command line was my worktree's).
> - Live server on 8095 answering 200; the other workers' rigs (8098, 8125, 8143, 8151) untouched.
> 
> The report above stands as written — including the two things I could not prove and am not claiming: **MediaRecorder was never exercised** (this box's headless Chrome refuses the fake audio device, so the iOS timeslice question is still open), and **the joined MP4 was never played back**. Both need the simulator, which another worker held for the whole session.</result>
> <usage><subagent_tokens>470323</subagent_tokens><tool_uses>463</tool_uses><duration_ms>3850634</duration_ms></usage>
> <worktree><worktreePath>/Users/microserver/fm2/.claude/worktrees/agent-a42bf4bae1c8c7811</worktreePath><worktreeBranch>worktree-agent-a42bf4bae1c8c7811</worktreeBranch></worktree>
> </task-notification>

### p21b
*2026-09-04 10:37*

> <task-notification>
> <task-id>a42bf4bae1c8c7811</task-id>
> <output-file>/private/tmp/claude-501/-Users-microserver-fm2/b70435b4-6806-4a25-aadb-e68df3da3413/tasks/a42bf4bae1c8c7811.output</output-file>
> <status>completed</status>
> <summary>Agent "Build video notes to the mini" finished</summary>
> <note>A task-notification fires each time this agent stops with no live background children of its own. The user can send it another message and resume it, so the same task-id may notify more than once.</note>
> <result>Another stale wait timer from earlier, draining now. It reports `smoke: all passes green` for the node-5 gate (port 8145) — the same verdict I already folded into that commit message after correcting my earlier hedge about it hanging.
> 
> No new information, no change to the delivery. Work is complete and the report above stands: six commits on `247deb8`, clean tree, my rig and transcriber stopped by their own PIDs, live server on 8095 healthy, and the two named evidence gaps (MediaRecorder never exercised, joined MP4 never played back) still open pending the simulator.</result>
> <usage><subagent_tokens>471154</subagent_tokens><tool_uses>464</tool_uses><duration_ms>3869441</duration_ms></usage>
> <worktree><worktreePath>/Users/microserver/fm2/.claude/worktrees/agent-a42bf4bae1c8c7811</worktreePath><worktreeBranch>worktree-agent-a42bf4bae1c8c7811</worktreeBranch></worktree>
> </task-notification>

### p21c
*2026-09-04 10:37*

> <task-notification>
> <task-id>a481d67a8bff6672b</task-id>
> <tool-use-id>toolu_01EcA2W1uoHFCJGuPqMBwLw7</tool-use-id>
> <output-file>/private/tmp/claude-501/-Users-microserver-fm2/b70435b4-6806-4a25-aadb-e68df3da3413/tasks/a481d67a8bff6672b.output</output-file>
> <status>completed</status>
> <summary>Agent "Build four map-reel refinements" finished</summary>
> <note>A task-notification fires each time this agent stops with no live background children of its own. The user can send it another message and resume it, so the same task-id may notify more than once.</note>
> <result>All four nodes are built, proven and committed. Report follows.
> 
> ## What shipped
> 
> Branch `worktree-agent-a481d67a8bff6672b` in `/Users/microserver/fm2/.claude/worktrees/agent-a481d67a8bff6672b`, rebased onto main's tip (`d5fb143`). Four commits, four nodes, 1140 insertions. Not deployed.
> 
> **`ffe326e` — `/until-play`** (`features/miso/loop/dictate/as-posts/capture/video/poster/face-first/until-play/`). Answers sentence 2. Diagnosed first: opening a video post costs **4 paints and 4 image loads** (the blinking); returning to a post played once earlier gives **2 `play()` calls and 1 `loadstart` with no finger near the play mark**; three paints under an open clip cost **4 `play()` calls, 3 `loadstart`s and 3 discarded players**. Cause of the autoplay: `/poster.opened` never forgot, so `restore()` re-opened the clip inside the tap that opened the *post*. Cure: `/poster` refactored to open `start(h)` + `replaying` (behaviour unchanged); the node suppresses the play on the repaint road, forgets a clip whose holder has left the screen, and carries the live `&lt;img&gt;` and `&lt;video&gt;` across the DOM swap. Only commit that is **not confined** — full `Toggle-proof:` trailer in the message.
> 
> **`fdd737c` — `/from-the-lozenge`**, **`cab6815` — `/carries-the-card`**, **`209205f` — `/back-to-the-lozenge`**: all confined (`fmlink --prove` says implied).
> 
> ## Evidence
> 
> Smoke gate green on my own rig (port 8125, own scratch home) after each node — `smoke: all passes green`, three passes. The trailing `!! the page closed` is handover residual (c), not the app.
> 
> Desktop rig (Chrome, real CDP touch): with `/until-play` **1 image load, 0 element swaps, 0 plays without a finger**; with it unticked, **4 image loads, 2 plays**. Opening: first painted frame at the lozenge (`0,674 232x88` → card at `-7,660` 241 wide), full size `358x724` at `16,48` by 292 ms, with three repaints forced into the first 180 ms. Carry: 40 px sweep tracks `0/-16/-24/-32/-40` and springs back; 140 px sweep tracks to `-140`, leaves at `-841`, next card arrives from `+432`.
> 
> **iPhone 17 simulator, installed web clip, real fingers** (`/private/tmp/claude-501/-Users-microserver-fm2/b70435b4-6806-4a25-aadb-e68df3da3413/scratchpad/a481-mapview/evidence/`): drag tracks the finger `0/-36/-84/-108/-132/-144/-168/-180`, card leaves at `-807`, next post arrives from `+396` and lands — the iOS touch road works. Close after two sweeps: card `16,48 370w` → `5,595 243w` (the lozenge is at `5,608,239`), and afterwards `reelLeft 506`, current lozenge and ringed pin both the third post, map centre on its pin — before the close the reel was still on the first post, which is exactly ash's complaint. Video post on the phone: **6 paints, 0 poster-element swaps, 0 `play()` calls**, poster up and waiting (`poster-sim.png`).
> 
> ## Named risks and open items
> 
> - **`/swipe-away` does not get the shrink.** It has already taken the card off-screen at opacity 0 when the send arrives; shrinking would haul it back into view. That road gets the reel scroll and map pan only. The ask named it — **this is my judgement and needs ash's or triage's word.**
> - **Three of us shared one scratchpad root.** My first rig scripts landed on `scratchpad/rig/`, overwriting another worker's `rig.pid`, `start.sh`, `stop.sh` and truncating their `server.log`. I restored their PID (16348) within a minute and moved everything to `scratchpad/a481-mapview/`. Nothing was killed. Worth a misses.md entry: the scratchpad is per-session, not per-agent.
> - **`/tmp/miso-readout.json` and `/tmp/miso-drive.json` are global**, so two simulator rigs corrupt each other. I temporarily pointed `readout_file()`/`drive_file()` at my own scratch for the sim run and reverted before committing (tree verified clean). A per-rig path belongs in `/diag/rig`.
> - The opening's first frame is ~7 px off the lozenge when a repaint changes the card's width between keyframe-building and the first composited frame.
> - `/carries-the-card` at either end of the list flies the card off and brings the same one back (~300 ms). Correct, but ash may read it as odd.
> - The resume-after-move in `/until-play` re-plays on any `pause` within 300 ms of a repaint; a reader who pauses inside that window gets one unwanted resume.
> - The web clip left on the iPhone 17 points at my (now stopped) port 8125; removing it needs the simulator shut down, which would disturb the other workers.
> - `transcripts/2026-09-04-field-walk.md` is committed at 6 prompts; triage's session-end re-export will extend it (anchors are append-only, `#p6` is stable).
> 
> My rig was stopped by its own PID (`48100`), checked against `ps -o command=` first. The live server and the other three workers' rigs are untouched.</result>
> <usage><subagent_tokens>428983</subagent_tokens><tool_uses>285</tool_uses><duration_ms>4637290</duration_ms></usage>
> <worktree><worktreePath>/Users/microserver/fm2/.claude/worktrees/agent-a481d67a8bff6672b</worktreePath><worktreeBranch>worktree-agent-a481d67a8bff6672b</worktreeBranch></worktree>
> </task-notification>

### p21d
*2026-09-04 10:37*

> <task-notification>
> <task-id>a42bf4bae1c8c7811</task-id>
> <output-file>/private/tmp/claude-501/-Users-microserver-fm2/b70435b4-6806-4a25-aadb-e68df3da3413/tasks/a42bf4bae1c8c7811.output</output-file>
> <status>completed</status>
> <summary>Agent "Build video notes to the mini" finished</summary>
> <note>A task-notification fires each time this agent stops with no live background children of its own. The user can send it another message and resume it, so the same task-id may notify more than once.</note>
> <result>Empty — another sleep-timer I started while polling the slow gates, draining after the fact. Several more of these may still fire; none can carry new information, since every gate and proof they were watching has already completed and been reported.
> 
> Nothing to add. The delivery is unchanged: six commits ending at `247deb8`, clean tree, my processes down, live server healthy, and the simulator-dependent evidence still outstanding.</result>
> <usage><subagent_tokens>471948</subagent_tokens><tool_uses>465</tool_uses><duration_ms>3889423</duration_ms></usage>
> <worktree><worktreePath>/Users/microserver/fm2/.claude/worktrees/agent-a42bf4bae1c8c7811</worktreePath><worktreeBranch>worktree-agent-a42bf4bae1c8c7811</worktreeBranch></worktree>
> </task-notification>

Only the transcript file conflicts, and that one is regenerated anyway. The deploy is still in its gate. I'll merge the reel branch as soon as it finishes, then deploy again.

### p22
*2026-09-04 10:37*

> <task-notification>
> <task-id>a42bf4bae1c8c7811</task-id>
> <output-file>/private/tmp/claude-501/-Users-microserver-fm2/b70435b4-6806-4a25-aadb-e68df3da3413/tasks/a42bf4bae1c8c7811.output</output-file>
> <status>completed</status>
> <summary>Agent "Build video notes to the mini" finished</summary>
> <note>A task-notification fires each time this agent stops with no live background children of its own. The user can send it another message and resume it, so the same task-id may notify more than once.</note>
> <result>Another drained sleep-timer from the polling loops. No new information; the work is complete and reported.</result>
> <usage><subagent_tokens>472348</subagent_tokens><tool_uses>465</tool_uses><duration_ms>3916058</duration_ms></usage>
> <worktree><worktreePath>/Users/microserver/fm2/.claude/worktrees/agent-a42bf4bae1c8c7811</worktreePath><worktreeBranch>worktree-agent-a42bf4bae1c8c7811</worktreeBranch></worktree>
> </task-notification>

Still waiting on the deploy gate. Nothing new from that notice.

### p23
*2026-09-04 10:38*

> <task-notification>
> <task-id>a42bf4bae1c8c7811</task-id>
> <output-file>/private/tmp/claude-501/-Users-microserver-fm2/b70435b4-6806-4a25-aadb-e68df3da3413/tasks/a42bf4bae1c8c7811.output</output-file>
> <status>completed</status>
> <summary>Agent "Build video notes to the mini" finished</summary>
> <note>A task-notification fires each time this agent stops with no live background children of its own. The user can send it another message and resume it, so the same task-id may notify more than once.</note>
> <result>Another drained sleep-timer. Nothing new; the delivery stands as reported.</result>
> <usage><subagent_tokens>472740</subagent_tokens><tool_uses>465</tool_uses><duration_ms>3962956</duration_ms></usage>
> <worktree><worktreePath>/Users/microserver/fm2/.claude/worktrees/agent-a42bf4bae1c8c7811</worktreePath><worktreeBranch>worktree-agent-a42bf4bae1c8c7811</worktreeBranch></worktree>
> </task-notification>

Still the same stale timer. Waiting on the deploy.

### p24
*2026-09-04 10:42*

> <task-notification>
> <task-id>by7364gd6</task-id>
> <tool-use-id>toolu_011dKscnjXhrB8Xv6JnEwGwG</tool-use-id>
> <output-file>/private/tmp/claude-501/-Users-microserver-fm2/b70435b4-6806-4a25-aadb-e68df3da3413/tasks/by7364gd6.output</output-file>
> <status>completed</status>
> <summary>Background command "Deploy the transcription build through the gate" completed (exit code 0)</summary>
> </task-notification>

Build 621 is live with transcription, and the reel build is deploying. Meanwhile the street list for seeding needs its one-time pull on the live box.

### p24a
*2026-09-04 10:43*

> <task-notification>
> <task-id>bak9556p6</task-id>
> <tool-use-id>toolu_01SkdzYygetwPm1FtMiB3vS7</tool-use-id>
> <output-file>/private/tmp/claude-501/-Users-microserver-fm2/b70435b4-6806-4a25-aadb-e68df3da3413/tasks/bak9556p6.output</output-file>
> <status>completed</status>
> <summary>Background command "Deploy the map-reel build through the gate" completed (exit code 0)</summary>
> </task-notification>
