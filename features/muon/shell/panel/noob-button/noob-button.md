# noob-button
*the nøøb button is the meta-button: tapping it opens the system panel again*

> (transcripts/2026-08-14-fm-spec-3.md#p58)
> OK: so let's move the "user" thing back to the noob button - that's the meta-button, basically. Keep the "user" button there, but let it be empty for now - we'll fill that in later with an editable user profile page.

## spec

The #p54 doctrine lands: the **nøøb button** (ash's name for the corner logo lozenge) is the meta-surface — it controls how muon works; the rest is muon. Its tap, parked by `/account`, is un-parked: tapping the lozenge opens the system panel (who you are, updates and the queue, log out), exactly as before `/account` moved it. The 👤 tool keeps its toolbar place and its teal, but opening it shows an **empty display surface** — the placeholder for the editable user profile page (`/account`'s social future, notes #p55). The agent prompt (#p53) will join the meta-surface later; the panel is its first tenant. Untick this and `/account`'s arrangement returns whole: tool opens panel, lozenge parked.

## user

The little logo at the top right is the system button again: tap it for updates, the feature list, and log out. The 👤 button in the toolbar is your page — empty for now, profile coming.

## glossary

- **nøøb button**: the corner logo lozenge, muon's meta-button — it steers muon (features, updates, the agent to come); the toolbar uses muon.

## code description

`noob-button.js` composes after `/account` (provenance order) and re-points two of its decisions at init: `feature_Panel.buttonTap` returns to `feature_Panel.open` (the seam's original default), and `feature_Account.watch` becomes a no-op so opening the 👤 tool no longer drives the panel — the tool opens onto the empty surface. `/account`'s shade-close wrap stays live and harmless: dismissing the panel while the empty tool is open also returns to the launcher. The init uses the same poll-until-loop-ready pattern; both fragments gate on the same condition, and composition order runs `/account` first, so the re-pointing always lands on the parked state.
