# pressed
*a button answers the finger while it is held*

> (transcripts/2026-08-26-session.md#p154)
> let's give the user some feedback when a button is pressed - maybe it gets a little smaller and brighter while it's being pressed

## user

Press a toolbar button, or one of the view buttons at the top, and it dips — a little smaller, a little brighter — until you let go.

## spec

A press had no answer until its effect arrived, which on a slow turn is a beat of doubt (#p154). One reading, so it builds: the `:active` state of every toolbar button and every view button scales it to 0.9 and brightens it by 30%, reached in 0.08 s and released in 0.18 s. iOS only gives `:active` to touches when a touch listener exists, so this node registers one empty passive listener. `/tools`' mount slide is an animation and owns `transform` while it runs; a press after it owns it. Untick and buttons are still under the finger again.

## hostile cases

- A tinted button: `brightness` on the button lightens the tint; the black glyph stays black (its own filter is separate).
- A long-press: the button stays dipped for the hold, which is right — it is being held.
- Undo while dim: dips like the rest; its inertness is `/undo`'s.

## glossary

(no new terms)

## code description

`pressed.css` — the transition and the `:active` rule. `pressed.js` — the passive touch listener iOS needs.
