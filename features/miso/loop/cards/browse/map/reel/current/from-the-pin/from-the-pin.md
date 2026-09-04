# from-the-pin
*tap a post on the map and the band scrolls to it*

> (asks#1788536169816)
> tapping a post in the map should scroll to its reel lozenge

## user

Tap a post's pin on the map and that post becomes the one you are on: the pin is ringed and the band below slides along to its lozenge. The post does not open — tapping the lozenge is what opens it, and it is right there under your thumb.

## spec

A tap on a pin opened the post. Ash asked for it to move the band instead (the ask). The reading, which is the one this node builds: **a tap on a pin makes that post current, and nothing more.** Current is a thing this app already means — `/current` outlines the lozenge at the band's left edge and `/on-the-pin` rings that post's pin on the map — so making a post current is scrolling the band to its lozenge and letting those two follow. Opening stays the lozenge's tap, one finger away and now within reach; and `/back-to-the-lozenge` already scrolls the band the other way when a post is closed, so the two directions agree.

**The band is scrolled, not the map.** The lozenge is put at the left edge, which is exactly what `/current` calls current, and `/reel`'s own rule then applies as it does for a finger on the band: `/quicker`'s scroll listener pans the map to that post a beat later. That is the app's own grammar rather than this node's decision — a scroll of the band moves the map, whoever scrolled it. The mark is also set by hand, because a lozenge that is already at the edge moves nothing and fires no scroll event, and the ring must still be right.

**`/map` was refactored to open the seam.** The pin's tap was a closure inside `draw`, sending `browse_open:<id>` by hand because Leaflet stops the DOM event on its own markers. It is now `pinTap(p)`, a named function whose default is that same send, and this node redefines it. A post the band does not list — one the current project sifts out, a pin from another surface — has no lozenge to go to, so its tap still opens the post, which is what every pin did before.

Untick and a pin opens its post again.

**What was measured.** On the rig, on the pin's own click path — Leaflet's
marker handler into `pinTap` — a tap on the last post's pin moved the band's
scroll from 12 to **2664**, made that post current, ringed its pin, and left
**no card page open**; the map then panned to that post as `/reel`'s own rule
says. A tap on the lozenge that had scrolled into view opened the post as
before. With this node unticked the same tap opened the post at once, which is
the behaviour the `/map` refactor had to leave untouched.

## hostile cases

- **A pin whose post the band does not list.** No lozenge; the tap opens the post as before.
- **A pin whose lozenge is already current.** The band does not move; the mark is set anyway, so the ring is right either way.
- **A fanned group** (`/fan-out`). Fanning is drawing, not tapping — each fanned pin is still its own pin and each tap still means its own post.
- **The band not drawn yet, or `/reel` unticked.** No list to scroll; the tap opens the post.
- **A card already open.** The map is behind the page and a tap on it closes the page (`/opens-over-map`); a pin cannot be reached, so nothing here runs.
- **The live pin** (`/live/one-pin`). It is not a post and carries no post id, so `claim` finds no lozenge and the tap does what it did.
- **A tap during a scroll the finger started.** The scroll is replaced by this one; the last thing asked for wins, which is what a scroll always does.

## glossary

(no new terms)

## code description

`from-the-pin.js` — `feature_FromThePin`.

`lozenge(id)` finds the band's lozenge for a post, by the `browse_open:<id>`
event it carries.

`claim(id)` scrolls the band so that lozenge sits at the left edge — smoothly
where the browser offers it — sets `/current`'s mark by hand, and answers
whether it did anything.

The redefinition of `feature_Map.pinTap` is the whole of the wiring: claimed,
the tap ends there; unclaimed, it falls through to `/map`'s own send.
