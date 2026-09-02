# engineer
*a small gear on the nøøb sheet opens the engineer section — the only place engineer-level UI lives*

> (transcripts/2026-09-02-self-check.md#p7)
> let's keep the UI user-focused, with any engineer-level stuff displayed only on the noob popup page, and even then behind an "engineer" / "settings gear" button. Aside from this: yeah, I understand, let's go for it

*(The self-check itself is `/diag/self-check`, the earlier prompt #p4 of the same session; this node is where its report — and any engineer-level thing after it — is allowed to show.)*

## user

Nothing changes on the app itself. On the nøøb sheet, next to the build line, there is a small gear. Tap it and an engineer section unfolds — plain text for whoever is debugging the app with you; tap again and it folds away. It starts folded every time the sheet opens, and it is empty until a feature has something engineer-level to say.

## spec

The rule ash set with the self-check ask: the user surface stays user-focused, and anything engineer-level is shown only on the nøøb sheet, and even there only behind a gear. This node is that gear and the section it opens. It joins the build row (`/build-row`: the build line and the features button) as a third, quieter control — a drawn gear in `currentColor`, dim until it is open. Below the row sits `#engineer`, collapsed by default, folded again on every open of the sheet, empty until a tenant fills it.

The section is an **extensible function**: `feature_Engineer.fill(box)`. Its default holds the first tenant — `/self-check`'s report as plain text, when that node is present (it predates the gear, so the gear reaches for it, not the other way round) — and a later feature with engineer-level content replaces the property at load, calling the one it captured first and then appending its own block (service-worker state and storage usage are the anticipated next ones). A tenant may call `feature_Engineer.refresh()` when its content changes while the section is open; for the self-check this node does it itself, by wrapping `run()` at load so a finished check redraws an open section. No timer-installed wrappers: load-time replacement of a named function cannot race (notes.md, "the apply-wrapper race").

`engineer.agent.md` carries the standing rule into the composed skillset: engineer-level UI lives here and nowhere else.

## hostile cases

- `/build-row` unticked: no build row to join; the gear gets its own row at the foot of the sheet and the section follows it.
- `/self-check` unticked, no other tenant: the section opens on "nothing here yet" — the empty state is a sentence, not a blank.
- `/self-check` present but not yet run when the gear is tapped: "self-check: running…" (or "not run yet"), and the section redraws itself when the run finishes.
- The sheet closes and reopens: the section is folded again, whatever it was; the gear's `on` state follows.
- A tenant's `fill` throws: caught; the section shows the others' content and the error line, so one broken tenant cannot hide the rest.

## parked

- Other engineer tenants (service-worker registration state, storage estimate): each a node extending `fill`.

## glossary

- **engineer section**: the folded block under the build row that holds engineer-level readouts; opened by the gear, never shown otherwise.

## code description

`engineer.index.js` — `feature_Engineer`: `fill(box)` the extensible function (its default appends `#selfCheck` with `feature_SelfCheck.text()` when that node is present, else nothing); `toggle()`, `render()` (folds/unfolds `#engineer`, runs the tenants, writes the empty state), `refresh()`. At load: makes the gear button (an inline SVG, `currentColor`) and appends it to `#buildRow` when present, else to a row of its own; makes `#engineer` after that row; wraps `feature_Panel.open` so every open of the sheet starts folded; wraps `feature_SelfCheck.run` so a finished check refreshes an open section.

`engineer.index.css` — the gear (30px, dim `#77777e`, `#c9c9d2` when open) and the section (monospace 12px, `#121215` on a `#202026` border, scrolls past 40dvh).

`engineer.agent.md` — the instruction: engineer-level UI only behind the gear.
