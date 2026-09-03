# square-posts
*a post's pin on the map is a rounded square; a person's stays round*

> (asks#1788371145253)
> posts on the map should show as rounded square icons instead of circles

## user

Open posts and switch to the map: every pin wears its picture in a rounded square. On 👤's map the faces stay round, and so does a live pin. One glance tells a post from a person.

## spec

`/map` draws one pin for every located card and draws them all alike — a round face on a grey stem, whatever the card is. The kind is known on the server and was thrown away on the way to the screen, so the page half had nothing to tell a post from a person with. This node carries the kind across and spends it on the corners.

The server half redefines `/map`'s `map_surface_html`. It does not rebuild the rows: `/map`'s own builder runs first, and its one `#mapData` element is opened, one `kind` field added per row from the card's `type`, and closed again. Rows are matched to cards by `id`, never by position, so `/map`'s own rule about which cards become rows is neither restated nor able to drift. Every other field the row carries survives, including fields `/map` may grow later.

The page half wraps `feature_Map.pinHtml` at load — the property-replacement idiom `/boundaries` and `/one-pin` use, never a timer — and writes `data-kind` onto the outer `.map-pin`. The markup stays `/map`'s; this only writes a word on it.

The stylesheet spends that word once: `.map-pin[data-kind="post"] .map-pin-face` gets a 22% radius. Same 34px face, same ring, same dark halo, same stem — the stem meets the rounded square's flat bottom edge more squarely than it met a circle. Nothing else about a pin changes, and nothing about a pin that is not a post changes at all.

`/live` writes its own pin markup (`<div class="map-pin map-live"…`) and never passes through `feature_Map.pinHtml`, so a live pin gets no `data-kind` and stays round. The page half also refuses any markup whose opening tag is not exactly `/map`'s, which is the same guarantee held a second time.

## hostile cases

- **`/map` changes the shape of `#mapData`** (a different attribute, a different element): every read here is a `match` that hands the html back untouched on the first surprise, so the pins are circles and nothing is broken.
- **The rows are not valid JSON, or not an array:** same — the untouched html.
- **A row whose id matches no card**, or a card with no `type`: no `kind` is written, and that pin is a circle.
- **A post with no picture:** the initial is drawn instead, in the rounded square — the same substitution `/map` already makes.
- **`/live` is unticked:** nothing changes; there were no live pins to keep round.
- **`/one-pin`'s index alignment:** it filters the rows by `lat`/`lon` and matches them to `/map`'s markers by position. This node adds a field to a row and removes none, and changes neither the count nor the order, so the alignment is exactly as it was.
- **A title or a picture path carrying `&`, `<`, `>` or `"`:** the unescape is the exact inverse of `/cards`' `card_esc`, ampersand last, and the rows are re-escaped by `card_esc` itself.
- **Node unticked:** no `kind` on the rows, no `data-kind` on the pins, no rule in the stylesheet — circles everywhere, as before.

## parked

- **A shape per kind** — projects as another shape, reports as another. The `data-kind` attribute already carries every kind the server knows; a child node adds one rule per shape and touches nothing else.
- **A kind-coloured stem** — the same selector, one more property, if a kind ever earns a colour (`/taste` 3: only if it earns a meaning).

## glossary

(no new terms)

## code description

`square-posts.rs` — `map_surface_html` calls `existing` and hands the result to `square_posts_kinded`, which finds the `data-pins` attribute, unescapes it, adds `kind` to each row from the card of the same `id`, and re-escapes; `square_posts_kind_of` is the id lookup; `square_posts_unesc` is `card_esc` backwards.

`square-posts.js` — wraps `feature_Map.pinHtml` at load; `mark` writes `data-kind` onto `/map`'s exact opening tag and returns the html unchanged for anything else.

`square-posts.css` — one rule: a rounded square face for `[data-kind="post"]`.
