# login
*how you prove it's you: a texted code, then Face ID*

> (transcripts/2026-08-25-accounts.md#p13)
> we'll do that using the invite process

## user

Browse the children: the SMS code flow (`/pin`) and Face ID (`/passkey`).

## spec

Grouping node, created under the 4–6 children rule: `users` stood at six
children (pin, gate, passkey, whole-number, harden, authority) and the invite
work — Tara joins tomorrow through it — needs a seventh. This regroup takes
`users` to five. Everything about *proving* an identity lives here: `/pin`
(the texted code) and `/passkey` (Face ID). The guest list, the login wall,
identity shape, hardening and authority stay beside it at `users`. Since
linearisation is provenance-ordered, the grouping changes no behaviour —
verified by an fmlink `--chains` diff before and after. Contributes no code.

## glossary

(no new terms)

## code description

No implementation files — a grouping node; `order.md` orders the children.
