# instance
*every device says which one it is*

> (transcripts/2026-08-16-fm-spec.md#p24a)
> And yeah, we need device IDs

## user

Nothing to operate. Each place you use miso — this phone, that laptop —
now signs the reports it sends home with a short name of its own, so a
problem on one device can be told apart from a problem on another.

## spec

Today the same question was asked twice and could not be answered: *which
device did this come from?* A transcription failed and the report named
no sender; the stopgap was to attach the browser's user-agent string
(build 193), which is verbose in every line and cannot tell two iPhones
apart at all.

The tree already calls them **instances** (`/mirror`: "your recordings
appear on all your instances"; `/scope`'s user scope is a set of them),
so they get instance ids: a short random name minted once per install and
kept in local storage, attached to every report this device sends. Losing
it when storage is cleared is honest rather than unfortunate — a cleared
instance genuinely is a new one, and pretending otherwise would mean
fingerprinting the device, which is the opposite of what miso is for.

The id is deliberately not a secret and not an identity: it says *which
of your instances*, never *who*. Identity is `/users`' business and is
already carried by the cookie.

Scope kept small on purpose: this node mints the id and puts it on
`/diag`'s reports, which is where the question was actually asked.
Everything else that will want it — `/journal`'s log lines, `/blackbox`'s
batches — reads it from here rather than minting its own.

## glossary

- **instance id**: a short random name for one installation of miso on
  one device, stable until its storage is cleared.

## code description

`instance.page.js` targets every page (as `/diag`'s own report helper
does, so the login and install pages can identify themselves too).
`feature_Instance.id` reads `localStorage.misoInstance`, minting and
storing a short base-36 name on first use; storage being walled off
yields a per-session id instead of an error, since an unattributed report
is still better than none.

It then wraps `feature_Diag.report` so every report carries `inst`,
leaving each caller's payload untouched — the same one-line-of-context
move `/engine-receipts` makes for `running`.
