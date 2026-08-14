# pwa
*being an installed app: icons, the install wall, standalone chrome*

> (transcripts/2026-08-14-fm-spec-2.md#p14)
> go for it :-)

## spec

Grouping node, created under the 4–6 children rule: shell stood at six
children and this regroup (proposed at fm-spec-2 #p13) takes it to four.
Everything about muon *as an installed PWA* lives here: `/icon` (home-screen
identity), `/install` (the add-to-home-screen wall browsers see), `/pinned`
(standalone-app chrome). The three were contiguous in shell's order, so the
grouping preserves linearisation exactly — verified by an fmlink `--chains`
diff before and after. Contributes no code.

## user

Browse the children: home-screen icons (`/icon`), the install wall
(`/install`), and standalone chrome (`/pinned`).

## glossary

(no new terms)

## code description

No implementation files — a grouping node; `order.md` orders the children.
