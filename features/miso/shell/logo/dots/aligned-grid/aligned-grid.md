# aligned-grid
*the graph paper agrees with the toolbar, and is centred on the screen*

> (asks#1786892582635)
> NEW ASK [proposed] … :: 'set the background grid size to match the tool icon size, and center it around screen center'
> *(a field ask, filed from the phone on 2026-08-16, miso build 198)*

## user

The background dots now line up with the tool buttons — one square of graph
paper per button — and the grid is centred, so a dot sits at the middle of
the screen with the pattern even on all sides. If the buttons ever change
size, the paper follows by itself.

## spec

`/dots` laid down graph paper at 32px, chosen when it was written; the
buttons are 50px, chosen when `/bigger-buttons` was written. Two numbers
picked independently, so nothing lined up. The ask is for one number.

**Size.** The grid cell becomes the tool button's size — not a copy of it.
`/tools` was refactored to name its two sizes (`--tool-size`,
`--tool-icon`) and derive its own rules from them, and `/bigger-buttons`
now *sets* those names rather than restating the rules. Both refactors are
behaviour-preserving; what changes is that the number has one home. This
node reads that name, so the paper follows the buttons through any future
change in either — including `/bigger-buttons` being unticked, which
returns 40px buttons on 40px paper rather than 40px buttons on a stale
50px grid.

**Centring.** `background-position: center` puts the tile's centre at the
container's centre, and `/dots` draws its dot at the centre of the tile, so
a dot lands exactly at the middle of the screen and the grid is symmetric
about it. The positioning box is `body`, which measures exactly the
viewport, so the screen's centre is what "centre" means here.

**The promotion rule's first case** (notes.md #p18: *a parameter earns its
variable on the second ask that touches it*). `/bigger-buttons` was the
standing first case, and `ideas.md` named the next size-shaped ask as its
trigger. This is that ask, and it is a stronger trigger than a plain
resize would have been: two features now have to agree on one number, and
a constant copied into a second stylesheet is the exact failure the rule
exists to prevent. What is promoted here is the *name* — the number has one
declaration and its consumers derive. Binding that name to a per-user
variable, so the size can be tuned without a build, is the rung after this
one and is not built in advance of an ask for it.

**Provenance, decided here (the flywheel's open ruling).** This ask reached
the builder through the ask store rather than the session log, so no
`transcripts/…#pN` anchor exists for it, and the linker refused to place a
node that cited nothing. Judgement taken: **a field ask is provenance in
its own right** — it is the human's actual request, timestamped to the
millisecond it was filed, carrying its own recorded OK, which is a better
record than a chat message quoting it. Specs may therefore cite
`asks#<t>`, and `tools/fmlink.py` resolves it by reading the position
straight from the id (no lookup: the id *is* the timestamp). Known gap:
`tools/audit_prompts.py` still only inverts transcript citations, so
ask-cited nodes read as uncited there until it learns the second form.

## glossary

(no new terms)

## code description

`aligned-grid.css`, two declarations on `body`, composing after `/dots`
(newest wins): `background-size` set to `var(--tool-size)` on both axes,
and `background-position: center`. The dot itself, its colour and its
radius stay `/dots`' business — this node changes only the ruling of the
paper and where it is anchored.

The extension point it uses was created by refactoring two parents, each
behaviour-preserving: `tools.css` now declares `:root { --tool-size;
--tool-icon }` and derives `.tool-button`'s width, height and font-size
from them; `bigger-buttons.css` now sets those two values instead of
restating the rules, and keeps only its own back-chevron sizing.
