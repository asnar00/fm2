# scroll
*where you were reading is kept — through every repaint, and through an update*

> (transcripts/2026-09-02-self-check.md#p88)
> scroll position needs to be kept.

## user

You are halfway down a long post. Someone's live pin moves, a card arrives from
another device, the app updates itself in the background — and the page stays
exactly where it was. After an update that reloads the app you come back to the
same post at the same place. Open a different card and you are at its top, where
a page you have not read yet belongs; come back to the list and the list is
where you left it.

Nothing is added to the screen. The whole feature is that nothing moves.

## spec

Every loop event redraws `#app` wholesale (`/loop`'s `paint`, one `innerHTML`),
and `.card-page` scrolls as an element rather than as the document, so the new
element is a new element and starts at the top. The measured behaviour before
this node: a card page set to 400 was at 0 after a no-op state change — not an
update, any repaint. `/keep` already holds the caret across that same moment;
this node holds the place.

**The two moments around a paint.** `/loop.paint` is taken by property
replacement at load, the idiom `/keep` uses on the same seam (`/me`'s precedent,
never a timer-installed wrapper — notes.md, "the apply-wrapper race"). Before
the html goes in, every scroller on the screen is recorded. After it, each
scroller found in the fresh DOM gets the position remembered under *its own*
name back. This fragment is the newest under the seam, so it loads last and
wraps outermost: the scroll is put back after `/keep` has restored the words,
the caret and the focus, and `focus()`'s own scroll-into-view cannot leave the
page somewhere else.

**A place is named, so it is never given to a different page.** A card page
answers to `card:<id>`, taken from the `data-card` the renderer already writes;
the browse list answers to `list:browse` and the grid to `grid:browse`. A
`.card-page` with no card id — the waiting card, the invite page, a deleted
post — has no name, is never recorded, and is never restored: it opens at the
top, which is right. A card you have not scrolled has no entry, so a first visit
opens at the top too.

**The record is bounded and evicts the least recently seen.** A session that
opens hundreds of cards must not grow a record without end, so `at` holds fifty
places, insertion-ordered, oldest discarded first. The page on the screen is
re-recorded before every paint, which moves it to the newest end, so the page
you are actually reading can only be evicted after fifty *other* pages have been
scrolled while it is away. Positions of zero are not recorded at all — a page at
its top is the default and would only spend a slot.

**A page that is not tall enough yet is given a moment.** Pictures decode after
the paint, so a `scrollTop` written into a still-short page is clamped. The
restore re-applies on each animation frame until the page fits, for at most
700ms. It stops early on any of: the position achieved, a newer paint (a
generation counter — an abandoned settle can never touch the new screen), or the
reader touching the screen (wheel, touch, pointer or key, watched in the capture
phase). A restore must never fight a finger, and a page that genuinely got
shorter simply ends up as far down as it goes.

**And a clamp is never mistaken for a choice.** This is the way the mechanism
would have destroyed the thing it exists to keep: a repaint arriving inside that
settle window would record the clamped `scrollTop` — the browser's answer, not
the reader's — over the real place, and the place would be gone for good. So a
key with a restore still in flight is skipped by `record` until its settle ends,
and an element with nothing to scroll is skipped always: it reports 0 because it
is short, not because anyone went to the top. The boot after an update is
exactly where this bites, because the page is drawn before its pictures are.

**Across the reload an update makes.** `/review`'s `apply` is the version stamp,
the cache eviction and the reload; wrapping it puts the record in front of all
three. The record travels in `localStorage.misoScroll`, in the shape `/seamless`
uses for the state: the build it belongs to, and the places. At load it is read
once and deleted — matching or not — and kept only if its build is the build now
running, which is `/seamless`' own test. The places go into the ordinary record,
so the resumed page is restored by the ordinary paint path, whichever paint
after boot happens to draw it (`/seamless` rehydrates on the first paint and
repaints with `seamless_resume`, so it is the second one).

The stash also carries the moment it was written, and is kept only if that
moment is within two minutes. The build number alone is not enough: `/patch`
can take an update in place — new wasm, no reload — and it stamps `misoVersion`
all the same (through `/delta`'s `quiet`), so a stash written for a reload that
never happened would otherwise match at a boot days later and put a stale place
back under a card. Two minutes is longer than any reload and shorter than any
session. The scroll survives a patch anyway, on the ordinary paint path,
because the record never left memory.

It is its own key rather than a field inside `misoStash` because `/seamless`
writes that key whole from inside its own wrapper, after this node's wrapper has
run; a newer node cannot add a field to it without editing `/seamless`. Same
stash discipline, its own key.

**Parked, named:** remembering a scroll per card across sessions (the record
lives for the life of the page, plus one update reload). "Jump to the newest" is
a control, not this. Other scrollers with their own identity — the transcript,
the queue, a project's roles — join by adding a line to `keyOf`.

`scroll.js` — `feature_Scroll`. `keyOf`/`each` name and find the scrollers;
`remember` is the bounded record; `record`/`restore` are the two moments;
`settle` is the retry for a page still growing; `stash`/`resume` carry the
record across an update's reload. The load block takes the two seams and
watches for a finger.
