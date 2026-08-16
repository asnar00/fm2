# build-row
*the build readout and the features button share one line*

> (transcripts/2026-08-15-fm-spec.md#p48)
> PROPOSAL: put the build readout and feature button on one line
> *(a field ask, filed 2026-08-15 on muon build 157)*

## user

put the build readout and feature button on one line

## spec

The build line and the **features** button were two rows saying small
things; now they share one, directly under the ask box — the readout
on the left, the button its natural size on the right. The features
button keeps every behaviour it had (folding, unfolding, the tucked
updates picker riding along); the readout keeps its writer
(`/less-busy`'s refresh finds it moved, not recreated). If either
half is absent from the composition, the other keeps the line alone.

## glossary

- **build row**: the one line under the ask box holding the build
  status and the features button.

## code description

`build-row.index.js` runs once at load (composing after `/less-busy`
and `/features-button`): it wraps `#buildLine` in a flex `#buildRow`
placed where the line stood, moves `#featuresBtn` in on the right, and
removes the emptied `#featuresRow`. Elements are moved, never
recreated — every standing handler and writer still finds them.

`build-row.index.css` lays the row out (readout flexes, button
natural width), `#panel`-scoped to outrank the panel's button
stretching — the standing cascade lesson.
