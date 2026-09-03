# transcript: invite tests
*session `745f3d4c-9e6f-40ae-b167-5902245543f9`, exported by tools/export_transcript.py — do not edit; anchors `#pN` are stable*

### p1
*2026-09-03 16:26*

> hi claude! welcome back to miso, a little app we're building to be used by Tara Matthews in her quest to become the next MP for Sevenoaks. Take a look around and let's crack on? Our first field test is two days from now.

Hi ash. First, the usage watch: Fable weekly is at 50% with 4.5 days to the reset, and at the last-3-days burn it runs out in about 1.4 days, around Saturday early morning. So it does not last the week at current pace. The all-models budget lasts almost to the reset.

Now I'll read the handover and doctrine to get oriented.

Orientation done: live build 564, tunnel up, ash's phone self-check green on 564, wifi watchdog quiet. The ask monitor is not running, so I'll rearm it now.

### p1a
*2026-09-03 16:27*

> <task-notification>
> <task-id>b2n9e20ex</task-id>
> <summary>Monitor event: "field asks from miso devices (acked building on sight)"</summary>
> <event>watching /Users/microserver/.miso-context (4 worlds) for asked,proposed</event>
> If this event is something the user would act on now, send a PushNotification. Routine or benign output doesn't need one.
> </task-notification>

Bringing up the simulator rig to reproduce the ‹ failures.

Rig built at main's tip. Starting the rig server on 8098 and running the one-level test on the simulator.

Reproduced the three ‹ failures on the simulator. Now reading the handlers that a ‹ tap is rewritten into.

### p2
*2026-09-03 16:31*

> Seems like we burned a lot of fable credits today - do you think that's because you did a lot of the work yourself rather than delegating?

Short answer: no, not this session and not today. The log says where it went.

**What the log shows.**

| when | Fable weekly |
|---|---|
| Tue 1 Sep, window opens | 0% |
| Wed 2 Sep, 16:47 | 39% |
| Thu 3 Sep, 10:00 | 42% |
| Thu 3 Sep, 16:30 (now) | 50% |

So about 42 points went in the first two days, and 8 points went in today's whole session of 34 builds. This session so far has moved it by nothing measurable.

**The likely cause of the big spend.** The worker seat was switched to Fable on 1 September, so every worker in Saturday's 20-ask day and Tuesday's builds ran on Fable, in parallel, at high effort. On the evening of 2 September the worker file went back to Opus, and today's session ran the same kind of day for 8 points instead of roughly 20. Delegating only saves Fable when the delegate is not Fable.

**Doing it myself vs delegating.** Me reading and reasoning is far cheaper than a worker doing a full five-step loop with a rig and a build, whichever model it runs on. The expensive thing is heavy tool-heavy builds, not triage. So the rule I'll keep is the one that worked today: triage, diagnosis and review here, builds on Opus workers.

**What that means for Saturday.** The estimate of running out early Saturday comes from the 3-day average, which is dominated by the Fable-worker days. At today's pace of about 8 points a day you reach Saturday at around 65% and the reset with room to spare. I'll keep this session light anyway, since Saturday is the day you need it.

Want me to carry on with the ‹ from a card page fix?

### p3
*2026-09-03 16:33*

> the < thing is fixed, has been for a while. maybe the handover is out of date

Handover corrected and committed. The rig is up on 8098 from the triage worktree if we want it, and the ask monitor is armed.

What do you want to crack on with for Saturday? My suggestion is the walk through your own checklist from the handover on the simulator, end to end: map pins for you, alice and bob, the flag tool making the "miso" project, + for a post from where you stand, and the invite flow Tara will go through. I'd fix whatever stumbles as I find it. Or if there's something on your mind for the field test, that comes first.

### p4
*2026-09-03 16:37*

> I'd like to test out the invite workflows. I have a second iphone and an android phone - we'll start with the iphone. I'll want to be able to quickly "reset" after a test - remove the test user and all of their posts/other data.

This is two things: the invite walk itself, and a reset tool to strip a test user out cleanly. Let me look at how a user's data is laid out on the mini so the reset removes all of it and nothing else.

Writing the reset tool now.

The op door cannot drop a card, by design. Reading how posts and projects delete today so the reset uses the same road.

The reset tool is in and proven. You can start the invite walk whenever you like.

**To reset after a test**, on the mini:

```
python3 tools/reset_user.py --list          # who is on the guest list, invited by whom
python3 tools/reset_user.py <name>          # take them out
python3 tools/reset_user.py <name> --dry-run
```

**What it does.** Their profile and posts held in your world become tombstones, so your phone drops them the way a deleted post is dropped. Any role they held in your projects is unlinked. Their guest-list row, PIN, passkeys and push subscriptions go, their world log moves to a removed folder, and the server restarts by handover so it forgets them. Nothing is deleted outright, and the same number can be invited again straight away. It refuses to touch an admin or support user without a flag.

**Proof.** On the rig it tombstoned a planted copy in your world and removed the user. On the live server I ran it on the stale `_test` row, which had no world. The restart took under two seconds and the phone-facing build is unchanged at 564. alice and bob are untouched.

**While you test** I am watching the server and device logs for invite, login, launch and error lines, and the ask monitor is armed. Go ahead with the second iPhone. Tell me the name you invite them as and I'll follow it through.

### p5
*2026-09-03 16:50*

> <task-notification>
> <task-id>b6fr9i4fh</task-id>
> <summary>Monitor event: "invite/login/launch lines from the live server and device logs"</summary>
> <event>qr: a claim on 3b668b… was already on the list</event>
> If this event is something the user would act on now, send a PushNotification. Routine or benign output doesn't need one.
> </task-notification>

### p5a
*2026-09-03 16:50*

> <task-notification>
> <task-id>b6fr9i4fh</task-id>
> <summary>Monitor event: "invite/login/launch lines from the live server and device logs"</summary>
> <event>doors: …3023 is invited into asnaroo.1787704828987 as candidate</event>
> If this event is something the user would act on now, send a PushNotification. Routine or benign output doesn't need one.
> </task-notification>

Seen in the logs just now: a scan of your live invite code came from your own number, so the server said you were already on the list and then gave you the role of candidate in your own project "asnaroo.1787704828987". If that was the second iPhone logged in as you, that's expected. If you meant to come in as a new person, log that phone out first, or give me the number you'll use and the name, and I'll watch for it.

### p6
*2026-09-03 16:51*

> OK. The QR code shouldn't require an SMS challenge - having the QR code within the time limit is "proof" that you're authorised.

Understood: the code itself is the proof, so a scan should land you logged in with no PIN. Reading the current QR path to see where the challenge sits.
