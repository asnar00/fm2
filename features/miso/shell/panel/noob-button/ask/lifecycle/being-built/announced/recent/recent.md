# recent
*the work queue as a plain feature list: the last eight, newest first, each wearing the stage it is at*

> (transcripts/2026-09-04-field-walk.md#p208)
> I had an idea for the "building" list of features showing the work queue. why not keep those features there, and change the label; eg. "building", "testing", "deploying", "installed" - showing the most recent N features, whatever their status? That way we don't have to care about version numbers or whatever. It's just a feature list, most recent first.

## user

The list on your sheet stops emptying itself. The last eight things built for
you stay there, newest first, and each one says where it has got to:
**building** while it is being made, **testing** while the release it is in is
being checked, **deploying** while that release is going out, and
**installed** once *your* phone is running it. No build numbers, no version to
work out — the word is the answer. A colleague's phone that has not updated
yet still reads *deploying* on the same row, because that is true of their
phone.

## spec

`/announced` put the builder's own announcements on everyone's sheet and took
them off again the moment they shipped, so the list was only ever "what is
unfinished" and went empty between builds. `/by-the-ship` then made the deploy
close them by itself, which is what makes the rest of this possible: the
release already knows which announcements it carries. Ash (#p208): keep the
features there and change the label — building, testing, deploying, installed
— *"That way we don't have to care about version numbers or whatever. It's
just a feature list, most recent first."*

**Eight.** The block sits in one panel above the requests and the feature list
itself, and eight rows is what fits before it starts pushing the list it
belongs to off the screen. It is also about a working day of announcements, so
a person opening the sheet in the evening sees the day. Older entries fold
away; nothing is deleted, and `builds` keeps its own forty (`/announced`).

**Three of the four words are stamped by the deploy**, because `deploy.sh` is
what knows when its gate starts and when its bytes move. `testing` goes on
before the first gate, on exactly the set `--ship` would later close — the
announcements naming a node the release touches. `deploying` goes on once
every gate has passed, immediately before the binary and the site move.
`shipped` is the ship stamp `/by-the-ship` already wrote.

**The fourth is decided on the phone.** *Installed* means "this phone is
running it", which no server can answer for somebody else's device. The page
compares the build it launched from (`/update`'s `running`, the store's
`misoVersion` behind it) with the build the announcement shipped in: at or
past it reads **installed**, behind it still reads **deploying**. The number
does the work and is never shown, which is the whole of "we don't have to care
about version numbers".

**A deploy that stops puts its entries back.** An `EXIT` trap in `deploy.sh`
returns them to `building` with the reason on the entry (`why`), shown as a
dim line under the row when it is opened. A ship retires the reason. Without
this, a failed gate would leave the sheet saying *testing* until the next
release, which is a lie the sheet used to be incapable of telling.

**Field asks keep their own rows, unchanged.** An ask has no node, so no
deploy can move it through testing and deploying; the stage words would be a
promise the machinery cannot keep. Their rows still say `building` while they
are being built and still leave the block when they ship, exactly as before —
and `building` happens to be the same word, so the block reads as one list.
The two kinds are already distinguished by the row's own text.

## hostile cases

- **An announcement with no node.** Nothing stamps it, so it sits at
  `building` — visible, honest, and named by `/by-the-ship`'s reminder at
  every deploy once it is a day old.
- **A phone that is offline or has not updated.** It reads *deploying* on a
  shipped row until it takes the build; the word turns to *installed* on its
  next open, with no message from anywhere.
- **A phone that cannot say what it runs** (`first-run`, a cleared store):
  every shipped row reads *deploying* rather than claiming an install.
- **More than eight.** The older ones fold away and stay in `builds`; a
  shipped entry that has scrolled off is simply an old feature.
- **A deploy that dies without running its trap** (a kill -9, a power cut).
  Entries stay at `testing` or `deploying`; the next deploy touching their
  node moves them on, and `/by-the-ship`'s reminder names them after a day.
- **Two deploys at once.** Not a thing on one box (one checkout, one script),
  and the second's `testing` stamp would simply re-stamp the same set.
- **This node unticked.** The block goes back to the announcements still
  `building` only; the deploy's stage stamps are scaffolding and keep
  writing a status the sheet then reads as it always did — `building` for
  anything not shipped.

## glossary

- **stage**: where a build has got to on its way to a phone — building,
  testing, deploying, installed. The first three are the release's, the
  fourth is the phone's.

## code description

`recent.index.js` — `feature_Recent.list()` is what `/announced` contributes
to the block now: every announcement, newest first, the most recent `N` (8),
each with its `status` replaced by `stage()`'s word. `mine()` is the build
this phone launched from and the only input `installed` needs. `paint()` runs
after `/lifecycle`'s render — this wrap is the outermost, so the rows are in
the DOM — and does two things the row builder is not asked to do, because
siblings share it: it marks an `installed` pill for the stylesheet, and it
puts a stopped deploy's reason under the row it belongs to.

`recent.index.css` — the installed pill in the same shape and weight as the
amber one, in a colour that says finished; the reason line dim under the row.

`tools/stamp_ship.py` (scaffolding), `--stage testing|deploying|building`:
moves the same set `--ship` closes, never writes a build number, never touches
an entry that has shipped, and takes `--why` when putting entries back.

`tools/deploy.sh` (scaffolding): `testing` before the first gate, `deploying`
after the last one and before the rsync, and an `EXIT` trap that puts the
entries back with the exit status if anything in between stops the ship.

## risks

**The stage words are only as true as the deploy's own reading.** They are
stamped on the announcements naming a node the release touches, so an
announcement whose node is misspelt never moves, and one whose node a release
touches incidentally moves early — the same exposure `/by-the-ship` named,
now visible as a word on the sheet rather than only in a stamp.

**A build number is still doing the work behind `installed`.** The ask was to
stop *caring* about version numbers, not to stop having one: the comparison is
numeric and would go wrong the day builds stop being an increasing count.

**The trap is `sh`.** It fires on every exit path the shell controls, and not
on a killed process; the sheet can therefore hold a stale `testing` after a
hard kill until the next deploy.
