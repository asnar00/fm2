# one-add

*one add button, and a separate control that says what it will make*

> (transcripts/2026-09-01-saturday.md#p13a)
> OK, I'd request one small change: there should just be one "add" button, and
> the addition mode should be set using a separate tool. That way you don't
> have to think about your capture mode - you set it once at the beginning of
> a session (or indeed when you first use the app) and then it uses the same
> every time you hit "add".

## user

The posts row has one add button. Beside it a smaller control wears the glyph
of the kind of thing add will make — a camera, a video camera, a dot, or a
pencil. Tap that control and it opens into the kinds; tap a kind and it closes,
wearing that one. Add makes that kind from then on, on this device, until you
change it. Undo stays last.

## spec

`/capture` put four ways to make a post in one row and made them read as one
set. Ash asked for the row to stop asking the question: *"you don't have to
think about your capture mode - you set it once at the beginning of a session
… and then it uses the same every time you hit add"* (`#p13a`). So the four
controls become one control plus one setting.

**The add button is the kind's own button, wearing the plus.** Nothing new
runs when you tap add. The row this node is handed already carries every
kind's control, each with the event its own feature listens for — `/photo`'s
`capture_photo`, `/video`'s `vid_rec`, `/dictate`'s `dict_rec`, `/posts`'
`posts_new`. This node keeps the plus and gives it the chosen kind's event.
Every capture path — the hidden file input taken in the capture phase, the
recording edges the page halves watch — is reached exactly as before, and no
page half was touched.

**The choices are the controls the row hands us.** A kind is offered only if
its control was drawn, so the picker obeys every toggle beneath it for free:
with `/video` unticked there are three choices, with `/as-posts` unticked
there are three others, and their glyphs are lifted out of the buttons
themselves rather than drawn twice. Only `write` is this node's own, because
`/posts`' plus is the add button and cannot also be the mode's face — so this
node draws a pencil for it.

**`write` is the default**, and the mode a device has never set. It is the
one kind that asks the phone for nothing — no camera, no microphone, no
permission prompt — so a first tap of add on a new phone does what the plus
has always done. A mode whose kind is no longer in the row falls back to
`write` too, rather than pointing add at a button that is not there.

**The mode is device-scoped**, `/browse`'s `view` exactly: where you are and
how you work, not what you own. `/world-cache` keeps it, so it survives a
reload and an app restart; the write queues no op and nothing reaches the
wire. A per-user mode that follows you between phones is a different var and
is parked.

**While a recording runs, add is the stop it turned into.** `/as-posts` and
`/video` already answer their own recording flag with a stop face; this node
lifts that face into the add slot and leaves the mode control beside it, so
the row does not change shape mid-recording.

**The picker is open-on-tap, not always-on.** A permanent strip of four
choices is the row ash asked to be rid of. Open is a flag on the turn's state,
not a var: it is closed by choosing a kind and by any other tap, so it never
comes back on a later visit.

Untick this node and `/capture`'s four-button row returns exactly as it
shipped — this node only ever re-lays controls it was handed.

## glossary

- **add mode**: the capture kind — photo, video, audio or write — that the
  add button will make. Chosen once, remembered on this device.

## code description

`one-add.vars` declares `mode` (`"write"`), device-scoped, `own` — the same
declaration `/browse` gives `view`, and the same consequence: the write
queues no op.

`one_add_read` / `one_add_write` read and write it against the live context
rather than the bridged loop state, for `/browse`'s reason: `/payload`
republishes part-way down the update chain, so a render following this node's
own write would otherwise be one turn stale.

`tool_controls` redefines `/capture`'s. Under the same gate — the posts tool
open with no card open — it grabs each kind's control out of the row with
`/capture`'s own `capture_grab`, cuts them all, and puts back one of three
sets before `/undo` through `/posts`' `posts_before_undo`: the mode control
and the stop face while a recording runs, the choices while the picker is
open, or the mode control and the add button. With no plus in the row there
is nothing to fold and the row is returned untouched.

`one_add_mode` resolves the stored mode against what is actually drawn,
answering `write` for a kind whose control is absent. `one_add_ev` maps a mode
to the event its kind already listens for. `one_add_glyph` lifts the inner
markup out of a grabbed control — a button in a control row nests no div, so
the first `>` and the last `</div>` bound its contents.

`one_add_add_button` is the grabbed plus with its event and its title
rewritten. `one_add_mode_button` is the quiet face: untinted, so the pink add
beside it is the one lit thing (`/taste` 2). `one_add_choice` draws one kind
in the picker, wearing `/capture`'s button shape and, for the chosen one,
`oneadd-on`.

`update` writes the mode on `oneadd_mode:<kind>`, opens the picker on
`oneadd_pick`, and closes it on anything else.

`one-add.css` gives the chosen kind the accent that already means chosen, and
the picker keeps `/capture`'s tighter gap because its buttons keep the
`capture` class.
