# transcript: housekeeping: wifi, taps, video
*session `8c2a4676-8164-433a-8e54-10fdede97b18`, exported by tools/export_transcript.py — do not edit; anchors `#pN` are stable*

### p1
*2026-09-03 10:02*

> morning Mr Claude. How you feeling today? Enjoying the 5.1-ness?

Morning ash. Usage first, as ordered:

- **Fable weekly: 42% used**, 2.3 days into the window. Burn over the last three days is roughly double the window average.
- **At that pace it runs out Friday evening**, about 3.3 days before the Tuesday reset. All-models is healthier, lasting to Sunday night.

So this week is not a "lasts the week" week. If the heavy days continue, Saturday and Monday will be dry unless we ease off or lean more on the worker seat.

As for the 5.1-ness: feeling sharp, thanks. Hard to compare from the inside, but the loop feels quick and I have plenty of context to spend. I have not read handover.md yet this session. Say the word, or just hand me the first ask, and I will pick up from where yesterday left off.

### p2
*2026-09-03 10:03*

> cool cool. just checking that we've now selected opus high for subagents - that should ease our issue. We just have to get through Saturday (our first field test). Today our priorities are 1) invite workflow and 2) I'm going to walk around and make posts. What does handover say?

I'll check the worker agent config for the Opus-high setting and read handover.md.

**Opus at high is confirmed** for the worker seat. The agent file says model opus, effort high, and the worker type shows in my list. That should pull most of the burn off Fable.

Handover, in plain words. It was written yesterday morning and updated through the night, so it is current up to the last commit on main.

**Where the build stands**
- Everything from yesterday's night shift shipped and the three Opus asks that were still building have since landed on main: the map centre button, square post pins for posts on the map, and the toolbar glide when you go up a level. Released sha matches the tip of main, so the live build is current.
- Workers moved to Opus at high yesterday evening. Same ruling you just made.

**Invite workflow, what exists today**
- Two doors, as sub-tool buttons in the add-person row: a QR code and by-name. The page under them is empty.
- The QR carries a rank and a project. Members can invite members. The newcomer becomes a role link on the inviter's project at their first card, capped at the inviter's rank.
- Onboarding: the newcomer's own card is gated until a picture and a line are in, then an eight-card tour runs once.
- **Named residuals on invite:** ‹ from the invite page goes to the launcher rather than to 👤. The rank picker is a real dropdown while audience uses six pills, so two pickers for one idea. Both are yours to rule on or mine to fix as part of today's invite work.

**Posts, what to expect walking around**
- Live location on the people map is in, visibility-based, one pin per person. The map draws a parent square when the exact one is missing, and the constituency is pre-stocked at zooms 12 to 16 behind the gear.
- Posts are picture-first, ordered by EXIF date, deletable with undo, with a manual save pill. Making a post is still not undoable.
- Scroll is kept through repaints and updates now.

**Things to watch on Saturday**
- The gate twice saw the page close mid-pass at the invite step and I could not reproduce it. smoke.py now names crash versus close. If the phone does anything odd on invite, that is the first clue.
- A new user's update policy default is unverified. If a dead control ever traps someone on an old build, the panel's update button is the road out.

**Housekeeping I will do before the first ask:** rearm the ask monitor piped through the acknowledger, so your field posts get instant "building" stamps.

Give me the shape of the invite workflow you want and I will start there. Otherwise I will begin with the two invite residuals while you walk.

### p3
*2026-09-03 10:09*

> That looks good. Couple of housekeeping things before we start: 1) my home wifi (which the mini lives on) dropped twice yesterday evening after 7pm, so I lost contact. That's not going to be a great look if it happens on Saturday, so I'd like to sanity-check to make sure that it didn't drop because of something we did. 2) There are two long-running bugs that I still see: a) screen taps still don't reliably press buttons, often requiring 2 or 3 taps to hit the button. b) I made a video post last night and that's showing the text above the media, which is supposed to have been fixed.

Three things, in order: the wifi drop, the taps, the video post. I'll read the discipline doc first, then dig into each.

Still on the wifi check. The mini's log has 1.2M lines for the evening, so I'm filtering for the actual join/drop events, and in parallel pulling the server logs and the tap and video code.

Wifi verdict is in hand. Now reading the video post layout and the tap handling code.

Still digging. I'm now reading the tap-handling code and pulling ash's stored video post to see if it got a poster.

Quick update: the black box gives a crisp signature for the lost taps, so I'm about to prove it on the simulator, and I'm writing the video-post fix in parallel.

Build is up and the rig restarted. Checking the new fragments actually composed, then reloading the app on the simulator.

Both taps proofs pass on the simulator. Now running the existing tap-heavy simulator tests for regressions and checking the video rule composed.

Deploy is running through the gate in the background. Meanwhile I'm rearming the ask monitor and checking the announce tool.

### p3a
*2026-09-03 10:41*

> <task-notification>
> <task-id>bsa3dosk1</task-id>
> <summary>Monitor event: "field asks arriving (stamped building on sight)"</summary>
> <event>watching /Users/microserver/.miso-context (4 worlds) for asked,proposed</event>
> If this event is something the user would act on now, send a PushNotification. Routine or benign output doesn't need one.
> </task-notification>

Release build finished; the smoke gate is running now. While it runs I'm checking the ledger format so the taps finding gets its entry.

### p3b
*2026-09-03 10:44*

> <task-notification>
> <task-id>bs5bnwyg8</task-id>
> <tool-use-id>toolu_01KrchztVP5WSJQuo4YrshA8</tool-use-id>
> <output-file>/private/tmp/claude-501/-Users-microserver-fm2/8c2a4676-8164-433a-8e54-10fdede97b18/tasks/bs5bnwyg8.output</output-file>
> <status>completed</status>
> <summary>Background command "Deploy both nodes through the gate" completed (exit code 0)</summary>
> </task-notification>

Build 533 is live and both announcements are stamped shipped. Writing the handover entry, then reporting.

Done. Build 533 is live, both fixes shipped through the gate, and the wifi question has an answer.

**Wifi: not us.** The mini is on wifi, with its ethernet port empty. The mini's own log shows its link dropping four times: 19:51 (down 67 minutes), 20:59, 22:47 and 01:50. During the long outage it tried to re-associate 327 times and macOS logged "broken backhaul" faults, which means the access point itself had lost its uplink. Nothing of ours was running at 22:47 or 01:50, and the 19:51 drop came two minutes after a deploy finished, with the mini idle. For Saturday I would plug a cable into the mini's ethernet port. If you want belt and braces too, say the word and I will add a small watchdog that cycles wifi when the gateway is unreachable for a minute.

**Taps: the real cause, fixed (build 532).** I read your phone's black box rather than guessing. Of 89 real presses on toolbar buttons, 13 produced no click at all, and the cutoff was crisp: every press that worked was held 114 ms or less, every press that failed 127 ms or more. iOS hands a touch held past about 120 ms to a different recognizer and never fires the click. A firm, deliberate press is exactly the one it eats, which is why the second and third tries felt worse. The simulator reproduced it in ten taps. The tap is now sent when the finger lifts, whatever the press length. Proven on the simulator: one send per press from 50 ms to 450 ms, a long hold still shows the tool card and sends nothing, and the invite-doors and glide tests stay green.

**Video post: fixed (build 531).** Tapping the poster swapped its class to the player's, and only the poster had the rule placing it above the words. One rule added, checked on the simulator's stylesheet. Your post from last night will sit right once the phone updates.

**Things to check on the phone when you update:**
- Open a person's card under 👤, press ‹. A simulator test says it may go to the launcher instead of the people list. That failure is on main with my change unticked too, so it is older. One tap tells me whether it is real.
- The gate's throttled pass printed "the page closed" after its last step again. All steps passed, so it shipped, but that intermittent is still unexplained.

The ask monitor is rearmed with instant "building" stamps, and the handover and misses ledger carry today's findings. Ready for the invite workflow whenever you are.

### p4
*2026-09-03 10:56*

> great stuff. let's add the wifi watchdog and monitor the situatio

Adding it now. First a look at the existing launchd job as a template, and whether this user may cycle wifi without sudo.

The watchdog is installed and running, and I'm watching its log.

**What it does.** Every 30 seconds a launchd job pings the router, then 1.1.1.1 as a fallback. After two misses in a row, about a minute, it switches the wifi radio off and on once. It will not cycle again within five minutes, so a router that is genuinely down gets one nudge every five minutes rather than a thrash. Every change of state is a line in the log at `~/wifi-watchdog.log`, plus one OK line an hour so we can see the watchdog itself is alive.

**What was tested.** The detection path was exercised with unreachable addresses in dry mode: the log shows DOWN, then CYCLE after the second miss, then UP and OK once the real gateway was back. The radio cycle itself was not run for real, because it would have cut this session. The first genuine DOWN, CYCLE, UP in the log is the proof, and I will report it the moment it lands.

**Monitoring.** A watch on the log is armed for this session. I will hear about every DOWN, CYCLE, UP and hourly OK and will tell you if the wifi wobbles while you are out.

The cable into the mini's ethernet port is still the better answer for Saturday. The watchdog covers the case where the mini's own association goes stale, which is what last night's 67 minutes looked like. It cannot fix a router that has lost its uplink.

Ready for the invite workflow.

### p5
*2026-09-03 11:04*

> great. one note: the video post should have a square media, rather than the portrait clip. All visual media should be cropped to a central square, not just stills.

Placing it in the tree first: the stills' square crop is `capture/square-crop`, so this reads as its refinement for video. Reading that node.

The posts tool opened in map view, so no tiles were on screen. Switching to the grid and opening the test clip.

The player is a square above the words. The frame is black until it plays, so I'll play it to see the crop.

The crop is right: the square shows the clip's middle, with the top and bottom bands just visible at the edges. Shipping it.

### p5a
*2026-09-03 11:09*

> the frame shouldn't be black until it plays - it should show the poster frame
