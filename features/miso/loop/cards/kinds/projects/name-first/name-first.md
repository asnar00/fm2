# name-first
*in a project's people list, the name leads and the role follows*

> (asks#1787702954479)
> Project people list should show name then role
> *(filed from the field on 2026-08-26 by ash, birthplace `projects @ miso/loop/cards/kinds/projects`)*

## user

Under a project, each person's row reads their name first, bold, and their role after it.

## spec

`/projects` drew each people row role-first, the `.crow` way (the word where the number sits). Ash asked for name then role (`asks#1787702954479`). One reading, so it builds: the row is a flex row, so this node swaps the two cells' `order` and lets the name take its own width — the name is the thing itself, the role is what they do. Nothing else on the row moves; ✕ stays at the end. Untick and the role leads again.

## glossary

(no new terms)

## code description

`name-first.css` — `order` on `.proj-role` and `.ctext` inside `.proj-rolerow`.
