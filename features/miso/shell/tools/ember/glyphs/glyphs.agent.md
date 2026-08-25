# glyphs — how to make a toolbar icon

*You are adding or changing an icon in miso's toolbar — a tool button or
a control. This instruction toggles with the `/glyphs` node.*

A toolbar icon is ink, never a picture. Two forms are allowed:

1. **A filtered emoji** — a single emoji in the `.icon` span; `/ember`'s
   filter renders it black on the button's colour. Use this only for
   emoji that are true silhouettes (👤, 🎙); check the glyph on a phone,
   not just a desktop.
2. **A drawn inline SVG** — `<svg class="icon-svg" viewBox="0 0 24 24">`
   with rounded strokes (`stroke-width` ~2.6, `stroke-linecap: round`)
   in `currentColor`, so it is black on a tint and white on plain. Use
   this for anything that is a *shape* — arrows, plus, marks.

Never a character with an **emoji presentation** — ↩︎ ✅ ⚠️ ➕ and their
kin. iOS draws these as colour bitmaps that no CSS rule can recolour: the
shape changes and the house style is lost, silently (accounts #p44, the
undo arrow that shipped as a bitmap). If you are unsure whether a
character has one, draw it.

Every button in a control row is black on a palette colour (`/tinted`,
`/plus-tinted` are the precedents), and `/undo`'s button stays **last**
in every row — a newer node inserts in front of it (`before_undo`),
never after.

A tool's **new** button (the one that makes a thing) wears the tool's own
colour, not undo's blue — so the two control buttons read as two things
(`/posts` and `/projects` both arrived at this independently, 2026-08-25).
