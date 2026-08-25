# me
*the 👤 tool opens your own card: name, picture, mission — editable in place*

> (transcripts/2026-08-25-accounts.md#p10)
> instead of subclassing profile as a type of card, let's give card a "type" field - one of those could be "profile".

> (transcripts/2026-08-25-accounts.md#p7, the ask this node serves)
> ok. so I propose to build in this order: 1) the user page - name, picture, mission statement paragraph, all editable.

*(The brief asked for `#p7` as this node's anchor. #p7 is 12:27 and `/cards`'
own anchor #p9 is 12:31, so citing it puts a child before its parent in
provenance order and the linker refuses — correctly. The anchor is #p10
instead, the prompt at which "under the existing 👤 tool, `me`" was actually
named; #p7 is the ask, quoted where it belongs.)*

## user

Tap 👤 and you get your own page: your name at the top, a picture, and a
paragraph about what you are here to do. Tap any of them to change them; tap
away and they are kept. Everything you write is yours — it is on all your
devices and nobody else's until you hand it over.

## spec

The 👤 tool has been a placeholder since `/account` gave it a button
(#p46 there: *"a profile page is coming"*). This node is that page. Opening 👤
now shows **your card of type `profile`**, rendered by `/cards` as a page of
blocks and editable in place; the system panel does not open, and everything
administrative stays where `/noob-button` put it — behind the lozenge.

**The card is made on first open, not before.** The page half sends
`CardEnsure` with the name `auth/whoami` gives, because that name lives behind
the cookie and not in anyone's world; `/cards` makes a card of that type and
owner if there is not one already, and does nothing if there is. So the very
first tap on 👤 shows a card with your name in the title, an empty picture and
an empty mission, rather than an error or a blank screen. A second tap, or a
second device, finds the same card.

**And not before the world arrives.** "Do I already have one?" is a question
about a world that reaches a fresh instance by join, not by boot. An ensure
asked before the join lands reads an empty world, makes a *second* card, and
last-write sends it over the first — which is exactly what this node did on its
first rig run, a duplicate profile per page load. So the ensure waits for
`/veil`'s `fm-joined` mark, which the join sets and the join's own two-second
timeout also sets, so an offline instance still gets a card. With `/veil`
unticked nothing can answer the question and the ensure goes at once —
correct, and the reason the offline duplicate is possible at all: two
instances that both create while unable to reach the server will both have
made a card, and the later write wins. Per-card identity is the rung that
closes it.

**Which surface 👤 opens is a seam, not a rewrite.** `/account` decides what
opening its tool does through `openTool` / `closeTool` on its own object, which
default to the system panel exactly as before. This node replaces them with
nothing at all — the card page is drawn by the render chain, so the tool needs
no sheet. Untick `me` and the pair falls back to `/account`'s own, so 👤 opens
the panel again, as it did yesterday.

**Who "me" is, is a question this node does not ask twice.** The render chain
draws the first card of type `profile` in your world. Your world only holds
your own cards today; when exchange arrives and other people's profile cards
land beside yours, this node grows the owner test rather than the store.

## hostile cases

- **No name.** `auth/whoami` unavailable or logged out: the ensure still fires
  with the empty name, `/cards` seeds the owner as `you`, and the title is
  editable — a card you can fix beats a page that refuses to appear.
- **The tool opens before the loop has booted.** The ensure waits for
  `feature_Loop.state`, so it is sent once the loop can carry it, never lost.
- **`/cards` unticked.** It is this node's parent, so unticking it takes this
  node with it and 👤 goes back to the panel — the one door, not two.

## what the toggle proof found, and could not show

Unticking `me` removes it cleanly: `feature_Me` is gone from the composed page,
`card_page_html` stays with `/cards` where it belongs, the render chain loses
exactly this node's link, and no card page is drawn.

What could not be shown is the other half of the sentence — 👤 opening the
system panel again — because **it does not do that on this build with or
without this node**. `/account`'s `watch` is orphaned by a race between the
page fragments that extend `feature_Loop.apply` from a timer, and the failure
reproduces at the tree baseline with `/cards` and `/me` both unticked and
`account.js` untouched. The seam's default is intact and unchanged; the thing
that would call it is not being called. The finding is written up in `notes.md`
("the apply-wrapper race"); it is not this node's to fix, and this node is not
affected by it — being the newest index fragment, it installs last and is
outermost by construction.

## code description

`me.rs` extends `render`: with `open_tool` at `account`, it appends
`card_page_html` of the first `profile`-type card in the world, or a quiet
"making your card…" line in the moment before the first ensure lands.

`me.js` is the page half. It takes `/account`'s open seam — `openTool` and
`closeTool` become no-ops, so the panel sheet stays shut — and it wraps
`feature_Loop.apply` to watch `open_tool`, sending one `CardEnsure` on the
transition into the tool. The name comes from `feature_Panel.lastWho`, which the
shell's loader already holds, and falls back to fetching `auth/whoami` itself so
the node does not depend on the panel being composed.

`ready` is the join test the ensure waits on: `/veil`'s `joined` flag or the
`fm-joined` class it puts on the body. It is a read of another node's mark
rather than a second listener for `VarJoin`, because the mark already means
"the world is as current as it is going to get" — including the timeout case,
which a `VarJoin` listener would wait through forever.
