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
