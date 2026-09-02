# ranked
*the QR code carries a rank and a project: everyone who scans it comes in at that rank, into that project*

> (transcripts/2026-09-02-self-check.md#p68)
> for the invite workflow, couple of things: by default, the person should be invited to the current active project (i.e. sevenoaks) - so the new person needs to automatically be added. Also, when inviting by name/phone, we should also be able to assign a role from a dropdown. The page should show two buttons: "show QR code" and "invite by name"; the former should let you choose the role (same for all invitees) and then display the QR code; the latter should pop up a single "name/phone/role" chooser and an invite/cancel buton.

## user

Pick a rank on `/doors`' sheet and tap **show**: the code that fills the
screen says, under *join miso*, where it leads — "into sevenoaks as
volunteer". Everyone who scans it and types their name and number is invited
into that project at that rank; they join it the moment they have a card of
their own. **new code** keeps the rank and the project. The count under the
code is as before.

## spec

`/qr`'s token is a row in a file — `{token, by, made, expires, uses, cap,
last}` — and `/doors` is the page that asks for a rank before showing the
code. This node puts the rank and the project on the row and carries them
across the claim onto the guest-list entry, so the QR road records exactly
what the name road records (`/doors`' two fields, `project` and `rank`), and
`/invited-into` needs no second reading.

**Mint carries them.** `qr_mint` is wrapped: with a `project` in the body the
inviter is checked through `/doors`' `invite_into_ok` — holds the project,
stands in it, not above their own rank — and after the inner mint the caller's
row is stamped `project` and `rank`. With neither in the body (**new code**
sends `fresh` alone) the row's previous values are carried onto the new row,
which is what makes **new code** keep the rank. The answer gains both fields,
so the sheet can say where the code leads.

**Claim carries them on.** `qr_claim` is wrapped: after a successful claim the
token's row names the project and rank, and the guest-list entry for the
number that just claimed is stamped through `/doors`' `invite_into_stamp` —
which skips an entry that has already joined, so a member re-scanning a code
(answered as a success by `/qr`, on purpose) is not re-invited anywhere.

**The sheet says where it leads.** One quiet line under *join miso* —
"into sevenoaks as volunteer", or "as volunteer" when the code carries no
project — spliced into `/qr`'s sheet by a `render` wrap. A code with no rank
shows no line, exactly as today.

## hostile cases

- **A rank above the canvasser's own.** Refused at mint with the sentence;
  no code is shown.
- **A code minted before the rank was chosen** (or by `/qr`'s own open, with
  this node's page unticked): no fields, no line, claims land as before.
- **The canvasser's rank in the project drops after minting.** Not re-checked
  at claim: the code gives what it was minted with until **new code** or
  expiry. Named, not closed — a day's window, the canvasser's own code.
- **The project is deleted while the code lives.** The entry still carries
  the id; `/invited-into` finds it gone at join and drops the fields.
- **`/doors` unticked.** This node calls its two functions; untick the pair
  together (the product's `order.md` is the place).

## glossary

(no new terms — the rank and project are `/doors`' words)

## code description

`ranked.rs` wraps `qr_mint` (check, then stamp the row, carrying the old
row's fields forward when the body has none; the answer gains `rank` and
`project`), wraps `qr_claim` (after a 200, the row's fields onto the entry
through `invite_into_stamp`), and wraps `render` to splice the quiet line
under *join miso*. `ranked_row_of` and `ranked_stamp_row` are the token
store's two small helpers, under `/qr`'s own lock and save.

`ranked.css` is the one line's dimness.
