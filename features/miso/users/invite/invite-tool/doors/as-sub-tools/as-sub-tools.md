# as-sub-tools
*add-person's two ways in are sub-tool buttons in its row — QR code, by name — not buttons on a page*

> (transcripts/2026-09-02-self-check.md#p88)
> thinking about it, "add user" should have two sub-tool buttons for QR code and "by name" - the page with the two buttons is doing the job of the toolbar. Add a memory so that in future, interface decisions conform to the "tree of tools" format.

## user

Open 👤 and tap the plus. The control row reads ‹, the lit plus, then two
buttons: a **QR code** and a **keyboard**. The page under them is empty.

Tap the QR code and the rank sheet opens — pick a rank, tap **show**, and the
code fills the screen. Tap the keyboard and the same sheet opens with the name,
the phone and the rank. Both roads are exactly what they were; only where you
choose between them has moved.

Long-press either button and its card says what it does.

## spec

`/doors` built the two ways in as two buttons on the invite page. Ash read that
back as the wrong shape (#p88): the interface is a tree of tools, a tool's
actions are its sub-tools, and a page of buttons is a toolbar in disguise. The
same ask made `/tools`' agent instruction, which this node is the first thing
built under.

**The buttons become controls.** `tool_controls` is extended with two
`.tool-button.ctrl` buttons carrying `invite_qr` and `invite_name`, drawn only
while the invite tool is open and only for someone who may invite. They go in
*front* of `/undo`'s button — undo is last in every row, and this node's link is
the newest on the chain, so keeping that invariant is its job. `/aside` has
already taken undo out when there is nothing to undo, and `/current-only` has
already dropped the parent 👤; with no undo marker to find the two controls
simply end the row.

**The page is empty.** `invite_rows_html` is redefined again, with no `existing`
call as `/doors` made none, and returns one holder carrying the selected
project's id and title — the two attributes `/doors` put on its block, under a
name of this node's own so the two readers can never both answer. The
`.invite-page` ground is not drawn (an empty `.card-page` is a small dark box
with nothing in it); attributes are read from an undrawn element as well as a
drawn one, so the sheet still says where the person is going.

**The tap is the page half's.** Opening a sheet is a DOM act, so the two events
never reach the Rust chain: a capture-phase listener claims them and stops the
click, which is also what keeps `/backdrop` from reading the tap as a tap on
bare ground. It is registered last of the capture listeners — this node is the
newest — so `/sub-tool-cards`' long-press suppression has already run, and its
`preventDefault` is the mark this listener reads to know a hold, not a tap, is
ending.

**The glyphs are drawn.** A QR code (three finder squares and two rows of
modules) and a keyboard, both inline SVG in `currentColor`, both tinted with
`/ember`'s colour for "invite" — the tool's own colour, so the pair reads as two
ways into one act beside the lit 👤 of the same hue (`/glyphs`). A person with a
pencil was the other candidate for "by name" and is not used: 👤 already stands
beside it and the pencil is the card page's edit mark.

**The long-press words.** Each button carries a `title`, which is what
`/sub-tool-cards` shows when nothing better is known. The lines proper belong in
`/tool-words`' `BUTTONS` table, keyed by `invite_qr` and `invite_name` — a
second node's business, named here so it can be built.

**Nothing else moves.** The sheet, its two faces, `invite_add`,
`invite_into_ok`, the stamping, `/ranked` and `/invited-into` are `/doors`'
unchanged.

**A coincidence worth naming:** `invite_qr` is also `/qr`'s state key. Events
and state keys are different namespaces and nothing reads across them; the name
is the one the row's grammar gives (`tap_reset`, `posts_new`, `proj_select`).

## hostile cases

- **A long press on a control.** `/sub-tool-cards` shows the card and calls
  `preventDefault`; this node's listener sees `defaultPrevented` and does not
  open the sheet. On release with no hold, `defaultPrevented` is false and the
  sheet opens.
- **A second tap while the sheet is open.** The sheet covers the screen
  (`inset: 0`), so no tap reaches the row; and `/doors`' `busy` flag already
  guards a double send.
- **`/qr` unticked.** The QR control is hidden at load, as `/doors` hid its own
  button, and the by-name road stands alone.
- **`/undo` unticked, or nothing to undo.** No `ctx_undo` marker in the row; the
  two controls end it instead of inserting before it.
- **Someone who may not invite.** They have no plus, so they cannot reach this
  level; and `may` is read here too, so a stale frame that showed the level
  anyway would show no ways in.
- **No project selected.** The holder carries an empty id, the sheet says "no
  project selected" and sends no project — `/doors`' behaviour, unchanged.
- **This node unticked.** `/doors`' block is drawn again, its own `project()`
  reads it, the page is two buttons and the row has no extra controls.

## glossary

- **sub-tool**: an action of the open tool, drawn as a button in its control
  row beside the tool's own icon.

## code description

`as-sub-tools.rs` — `tool_controls` appends the two tinted controls in front of
undo while the invite tool is open and the caller may invite;
`sub_tools_button` builds one; `sub_tools_before_undo` is the insertion;
`invite_rows_html` returns the project holder alone; `sub_tools_qr_svg` and
`sub_tools_name_svg` are the two drawn glyphs.

`as-sub-tools.js` — redefines `feature_Doors.project` to read the holder, claims
the two controls' clicks in the capture phase and opens `/doors`' sheet in the
matching face, and hides the QR control when `/qr` is absent.

`as-sub-tools.css` — the invite page is not drawn.
