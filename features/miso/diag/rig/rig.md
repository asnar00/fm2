# rig
*a rig server's pages are observable and driveable from their first paint*

> (transcripts/2026-08-26-session.md#p164a)
> the ability for you to run user-level interaction tests on the ios simulator, at high speed (i.e. not waiting for screenshots to tell you what's on there) - we've built a bunch of features allowing control and observation, take a look through them and suggest improvements, then let's build some real tests.

## user

Nothing to see on a real install. On the builder's test rig, the app reports its screen and takes commands from the moment it opens, so a test can drive an installed app on an iPhone simulator and read what happened in milliseconds.

## spec

`/readout` and `/drive` switch on with `?readout=1&drive=1` — and a home-screen app opens with no query string, so an installed app on a simulator could only be observed by screenshot (#p164a). A **rig** is a server started with `MISO_RIG=1`; it answers `diag/rig` with `{rig:true}` on localhost (never through the tunnel — a rig is a laptop talking to itself). Every page asks once at load, and on a rig arms itself: the readout's observer and its first post, the drive poll, and a one-second `/blackbox` flush so the finger (`/touches`) and the loop are readable at once. The login page is a page too, so a rig can type a `_` user in through `/drive`. A rig is plain `http://localhost`, and WebKit drops a `Secure` cookie there (Chrome keeps it, which is why no desktop rig ever saw it), so on a rig a localhost response's login cookie loses the flag. Untick and the rig is a plain dev server.

## hostile cases

- A rig server reached through the tunnel: `rig:false`; the endpoints stay cookie-gated as before.
- `MISO_RIG` unset (the mini, a dev server): `rig:false`; the query-string flags still work as they did.
- Two rigs on one laptop share `/tmp/miso-readout.json`; one rig at a time, named in `tools/simrig.py`.

## glossary

- **rig**: a miso server started with `MISO_RIG=1`, whose pages report and obey without flags.

## code description

`rig.rs` — the `diag/rig` route. `rig.page.js` — the ask at load and the arming.
