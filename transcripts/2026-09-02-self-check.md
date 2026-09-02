# transcript: the boot self-check worker
*session `6004deb9-d2ef-4b9a-8b15-ef1e406b9cb8`, exported by tools/export_transcript.py — do not edit; anchors `#pN` are stable*

### p1
*2026-09-02 09:52*

> ok claude! we're back, and hopefully running a new agent workflow configuration that lets you manage multiple in-flight workflow items on fm2, speeding up our overall progress rate. One safety check I'd like to add: could you please monitor our "percentage of fable 5 usage" over time, and let me know if it looks like we're going to hit the limit in the next 2 days? thanks

### p2
*2026-09-02 09:52*

> ok claude! we're back, and hopefully running a new agent workflow configuration that lets you manage multiple in-flight workflow items on fm2, speeding up our overall progress rate. One safety check I'd like to add: could you please monitor our "percentage of fable 5 usage" over time, and let me know at the start of each session how many estimated days remain, based on our recent use pattern?

I'll start by reading the handover, then work out how I can actually read the usage numbers from a script so the check can run at session start.

Script written. Running it now to seed the log and check the report reads right.

Now the launchd job, the session-start hook, the CLAUDE.md line, and a memory note, all independent of each other.

Everything is installed and the launchd job fired its first sample. Last step: a handover line and the commit.

Done. The usage watch is built, running, and committed.

**Today's reading, 09:54 UTC:**

| Limit | Used | Window |
|---|---|---|
| Fable (weekly) | 3% | resets Tue 8 Sep 04:00 UTC, 5.8 days away |
| All models (weekly) | 2% | same reset |
| Session (5-hour) | 4% | |

At the burn so far this week the Fable budget lasts the week, ending near 17%. That estimate is thin because the window is only a day old. It will firm up as the hourly samples accumulate.

**What I set up:**

- `tools/usage_log.py` reads the same endpoint the /usage screen uses, with the token Claude Code keeps on the mini. It logs the session, weekly and Fable percentages to a per-machine log. Nothing from the token is written anywhere.
- A launchd job samples every hour, so the burn rate comes from a real time series rather than from session starts alone. Samples fail when Claude Code is closed and the token has expired. That is harmless, since usage does not move while nothing runs.
- A session-start hook prints the report into my context. CLAUDE.md and a memory note tell me to open every session with the estimate in plain words, so you get it whether or not you ask.
- The report compares the projected exhaustion date with the reset date. It says either "lasts the week" or "runs out in N days, around such-and-such date". The estimate uses the higher of the window average and the last three days, so it errs on the cautious side.

**One thing to confirm:** the worker agent type does appear in my Agent tool list, so the Fable 5.1 subagent setup from the last session is live. I have not spawned one yet this session.

### p3
*2026-09-02 09:58*

> ok. the main thing I want to check is whether using fable 5.1 on high effort is the correct setting - I'd been getting good results on fable 5 at medium, and only hit my limits when multiple fable 5 subagents kicked off. My experiment now is fable 5.1 high for the "workflow manager / designer" (this session) and medium for the workers.

Let me check what's actually in force and whether the session transcripts let us split the burn by model and by seat, so the experiment can be measured rather than guessed.

Transcripts carry the model and token counts per turn, and subagent transcripts sit in their own folders. Let me total the recent sessions by model and seat.

Adding a `--seats` mode to the sampler so this breakdown, per session and per model and seat, is one command from now on. Then the assessment.

Here is what the transcripts on the mini say, and what I make of it.

**The setting in force is what you intended.** This session runs Fable 5.1 at high effort. The worker seat is Fable 5.1 at medium. So the experiment is set up correctly.

**What actually burned on Saturday** (the 20-ask day, one session, five hours):

| Seat | Model | Calls | Output tokens | Context read per call |
|---|---|---|---|---|
| main | Fable 5 | 708 | 581k | 280k |
| 10 workers | Opus 5 | 1,773 | 189k | 135k |

Two things stand out. First, Saturday's workers were Opus 5, not Fable 5. So they did not touch the Fable bucket at all. If you hit a limit that day it was the five-hour session limit or the all-models weekly one, driven by ten workers making 1,800 calls. The Fable-subagent limit hits you remember must be from before the move to the mini on 28 Aug. Those transcripts are not here, so I cannot check them.

Second, the bill is context re-reads, not effort. Effort only changes the output column. Output was under 1k tokens per call on average. Context was 150k to 280k per call, re-read on every call. Cached reads are cheap per token, but at that volume they still cost several times what the output does. Doubling thinking on the manager would raise the burn by maybe a tenth. Doubling the manager's context, or its call count, doubles the main cost.

**So my assessment of the experiment:**

- **5.1 at high for the manager is fine.** The manager makes few calls and its extra thinking is a small slice of the burn. Keep it.
- **Medium for the workers is the right dial, but the model change is the bigger move.** Workers now count against the Fable bucket. Ten workers of Saturday's shape would add about the same context volume as the manager itself, all in Fable. Expect the Fable percentage to climb faster than it did with Opus workers. The hourly log will show it within a day of real work.
- **The lever that matters most is the manager's context per call.** Saturday's manager averaged 280k tokens per call, against 146k in the overnight session and 50k in this one. Keeping this session lean, by delegating reading to workers and by compacting or restarting when it grows, saves more than any effort setting.
- **The five-hour session limit is about how many workers run at once,** not their effort. If that is the limit you hit, throttle the number in flight rather than the dial.

**To measure it going forward:** `tools/usage_log.py --seats` prints the table above for any recent session, so each day's experiment can be read off directly beside the percentage log. I committed it.

### p4
*2026-09-02 10:00*

> ok, let's kick off the next rung: the /diag boot self-check - could you explain in simple terms (I'm a "simple mind" ;-)

Let me read what the handover and the tree say about `/diag` before I explain it.

Here is the rung in plain words.

**The problem it solves.** On Saturday your phone showed three things the build did not produce: the safe-area gap, the blank projects page mid-update, and the video post with text above the picture. Each time I could only guess. My best guess was the phone's cached copy of the app was a mix of old and new files, because you took three updates in quick succession. But I had no way to see what the phone was actually running. I could only ask you to force-quit and try again.

**What the app already does.** Every time the app starts it sends one line home to the mini, saying "I am build such-and-such, the server is build such-and-such, I am logged in". That is the existing `/diag` feature. It is a postcard, not a check-up.

**What the rung adds.** At boot the app would look at itself and report what it finds. Three kinds of thing:

- **Which pieces it is really running.** The app is built from many small files, one per feature. Each file carries a version. The self-check lists them, so a phone running a mixed cache says so in its first report, with the exact stale file named.
- **Whether the basics work.** Can a tap reach the page. Is the start-up veil lifted. Is there a leftover wrapper from a half-finished update. These are the cheap checks that would have caught the blank projects page.
- **Whether the cached app matches the server.** If the phone's copy is behind or mixed, the report says so, and the app can fix itself by fetching the missing pieces rather than waiting for you to notice.

**Why it matters more than the smoke gate.** The gate on the mini tests the app in a simulator. It cannot see your phone. The self-check is the only test that runs on the real device, on the real network, with the real cache.

**How I would build it, in the tree's way:**

1. A new node under `diag`, roughly `diag/self-check`. Its spec says what the report contains and what the phone does about a mismatch.
2. Client side: after paint, gather the fragment versions and the three basic checks, post them as a second report line.
3. Server side: accept the richer report, keep the last one per device, and show it somewhere I can read from the mini. The log file already exists.
4. Prove it in the simulator rig. Then ship it, and the next report from your phone is the proof on the real device.

It is one node, so the toggle proof is implied by the confined rule, and it can go out as one build. I would give the build to a worker on the medium seat and review it here. Say go and I will start, or tell me if you want the report to also show on the phone screen, not just in the log.

### p5
*2026-09-02 10:13*

*(edited and resent — superseded by the next prompt; do not cite)*

> let's keep the UI user-focused, with any engineer-level stuff displayed only on the noob popup page, and even then behind an "engineer" / "settings gear" button.

### p6
*2026-09-02 10:13*

*(edited and resent — superseded by the next prompt; do not cite)*

> let's keep the UI user-focused, with any engineer-level stuff displayed only on the noob popup page, and even then behind an "engineer" / "settings gear" button. Aside from this: yeah, I understand, let's go for it.

### p7
*2026-09-02 10:14*

> let's keep the UI user-focused, with any engineer-level stuff displayed only on the noob popup page, and even then behind an "engineer" / "settings gear" button. Aside from this: yeah, I understand, let's go for it. Kick that off in a subagent as per protocol (design/delegate/review) - I want to work on map look and feel in parallel with that

On it. First I'll read the brief protocol and the code the worker needs to be pointed at, then spawn the worker.

Brief is ready in my head. Spawning the worker now.

The worker is running in its own worktree on the medium seat. I'll review its diff and evidence when it reports back, then integrate and ship.

**What the brief asks it to build:**

- **Two nodes.** A gear on the nøøb sheet, collapsed by default, with an extension point other engineer-level things can plug into. It carries an agent instruction that engineer UI lives only there, so the rule outlives this rung. Then the self-check itself under `diag`. Nothing appears on the user surface.
- **The check.** At boot the phone hashes every cached fragment against the server's list and names each mismatch. It also confirms a tap reaches the page, the veil is lifted, and no half-finished update wrapper is left. The result posts home as a second diag line and shows behind the gear.
- **Evidence it must return.** A simulator rig run on a clean install, a hostile run where one cached fragment is deliberately corrupted and the report must name it, the server-side receipt, a readout proving both screens stay clean with the gear closed, and the toggle proof both ways since two nodes are involved.
- **Parked.** Self-healing, where the phone refetches a stale fragment on its own. That is the next rung after this one and is named in the spec, not built.

Map look and feel is yours whenever you are ready. Tell me what you see on the phone and what you want it to look like, and I'll take it from there.

### p8
*2026-09-02 10:19*

> first, we're using openstreetmap as our current tile provider: what other options are there? include paid/free options, and can we audition different looks / rendering styles so I can evaluate each of them?

Let me check how the map is wired now, so the options fit what we have.

No response requested.

### p9
*2026-09-02 10:21*

> <task-notification>
> <task-id>a67b06922c8f01479</task-id>
> <output-file>/private/tmp/claude-501/-Users-microserver-fm2/6004deb9-d2ef-4b9a-8b15-ef1e406b9cb8/tasks/a67b06922c8f01479.output</output-file>
> <status>stopped</status>
> <summary>No completion record was found for background agent "Build /diag boot self-check" from the previous session. It may have been stopped, or it may have been running when the previous Claude Code process exited — either way its transcript is saved, so its progress is not lost. Resume it by sending it a message with SendMessage, or check its worktree/output for partial work before assuming the task landed.</summary>
> </task-notification>

Session resumed. Usage first: Fable is at 4% of the week, burning about 3 points a day, so it lasts the week with room to spare.

The self-check worker was cut off when the previous process exited. Let me see what it left in its worktree before resuming it.

The worktree is empty, so the worker got nothing done before the cut. Resuming it with its context intact, and building the tile audition page in parallel.

The audition page is up. Open it at `http://192.168.1.76:8777/` from the Mac or your phone on the home network, or use the file I just sent. Every map pans and zooms together over Sevenoaks. Tap a caption to mark a favourite, and the exact setting for the mini appears at the bottom.

**One correction first.** We are not on raw OpenStreetMap. The data is OpenStreetMap, but the drawing is CARTO's Dark Matter style, which our `/tiles` proxy fetches by default. That matters because switching provider is one environment variable on the mini, not a code change. And because the proxy holds any key server-side, a paid provider's key never reaches the phone.

**The options, in four groups:**

- **Free, no key, visible on the page now.** CARTO Dark, Dark no-labels, Positron and Voyager. OpenStreetMap standard, Humanitarian, CyclOSM, OpenTopoMap. Esri satellite, dark canvas and streets on their legacy endpoints. All fine at our traffic. OpenStreetMap's own servers forbid heavy use, so it is the one I would not ship.
- **Free tier with a key, paste it in the header and the cells fill in.** Stadia hosts the Stamen styles: Toner, Watercolor, Terrain, plus their own Alidade Dark. MapTiler has a good dark data-viz style and satellite hybrid. Thunderforest has a Neighbourhood style made for town scale and a sepia Pioneer. Jawg has clean dark and light. Geoapify has several. Free tiers range from a few thousand tiles a day to two hundred thousand a month, all far above a campaign team's use.
- **Ordnance Survey, key, free OpenData plan.** Light, Road and Outdoor. This is the map Sevenoaks residents grew up with, and it is the one I would most want you to see beside the others.
- **Metered.** Mapbox serves raster tiles from any style you design in their studio, so a fully branded look is possible. Google's map tiles are not usable through our proxy under their terms, so they are out.

**Two paths beyond the page.** Any raster style above ships by setting the URL and attribution in the launchd plist and restarting the server. The other path is self-hosting vector tiles for Kent from a single file on the mini, which gives us a map that works offline and a style we can edit ourselves. That is a real rung, not a setting, so I have left it as a mention.

The self-check worker had lost its work when the previous process died, so I have restarted it with its full context. I'll review when it reports.
