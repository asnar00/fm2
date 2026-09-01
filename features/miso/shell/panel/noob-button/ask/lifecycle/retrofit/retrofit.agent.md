# retrofit — old posts come along, and changes can be taken back

*You are briefing or building a feature that changes what posts (or any
user-held cards) render or hold. This instruction toggles with the
`/retrofit` node.*

Every such brief names its retrofit before the build starts: either a
backfill that brings existing posts up to the new shape (through the op
door, the same road as any edit), or an explicit ruling — ash's, recorded
in the spec — that old posts keep the old behaviour and why. Silence is
neither: a feature that quietly strands the posts a user already made
fails review even when the new posts are perfect (2026-09-01: posters left
old videos faceless; project links hid every pre-existing post behind the
selection).

Retrofits are additive and revertible. Write new fields beside old data,
never over it; the op log must be able to restore every prior value
(`/revert`'s precedent — the log holds the history, one op restores one
value). A retrofit that cannot be undone is not a retrofit, it is a
migration, and migrations go to ash before they run. Test the revert the
way you test the change.
