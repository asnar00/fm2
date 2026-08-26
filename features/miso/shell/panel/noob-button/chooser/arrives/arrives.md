# arrives
*the panel opens on the tap; what fills it arrives after*

> (transcripts/2026-08-26-session.md#p95)
> The nøøb button doesn't press any more

> (transcripts/2026-08-26-session.md#p95a)
> It would be good to think of ways to catch this kind of error before we ship. In fact essential.

## user

Tap the nøøb button and the sheet is there. If the feature list is slow to arrive it says so, and the next tap has it.

## spec

`/panel`'s `open` awaited `/chooser`'s `mount`, and the mount awaited `features/tree.json` — so one stuck fetch was a button that "doesn't press" (#p95). The fetch does stick, and it is not a slow network: under a freshly installed service worker with new content, a `cache: 'no-store'` re-fetch of a URL the worker has just stored never returns, while the same URL in default cache mode answers in 15 ms. The deploy gate (`tools/smoke.py`, #p95a) met it five times on 2026-08-26 — every first attempt after a real change, never on a relink — before its failure dump named the fetch. Three rules follow. A `tree.json` fetch never carries `no-store` (`/fresh` is net-first with `/deadline`'s 1.2 s, so freshness is the worker's job) and never waits past 2.5 s. A list that did not arrive is not the list: the sheet says "still arriving", and the chooser forgets it so the next open tries again. And the sheet is on screen before anything is awaited — `open` shows it first, then fills it. Untick and the tap waits for the list again.

## hostile cases

- The fetch stalls for good: the sheet opens in one frame with the who-line; at 2.5 s the list row says still arriving; the next open retries with a fresh request.
- `/review`'s own `tree.json` fetch: the same rule covers it — one budget, no `no-store`.
- Offline with a cached tree: the worker serves the copy inside its deadline; the budget is longer than the deadline, so the copy arrives in time.
- `vectors.json` (`/semantic-find`) is fetched `no-store` too and is not covered; it is not on the tap's path.

## glossary

(no new terms)

## code description

`arrives.index.js` — the `fetch` wrapper for `features/tree.json` (drops `cache`, races the budget, marks late); a `mount` wrapper that reports a late list and forgets it; an `open` wrapper that shows the sheet first.
