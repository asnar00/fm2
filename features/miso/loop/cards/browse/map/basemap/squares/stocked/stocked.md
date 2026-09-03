# stocked
*the map keeps the project's area in the phone's cache at low zoom, so it still draws with no signal*

> (transcripts/2026-09-02-self-check.md#p49)
> ok do the fallback first, then the pre-load

> (transcripts/2026-09-02-self-check.md#p47b, the question this answers)
> a note about caching: if we lose connection to the server while wandering
> about, does the app use locally cached tiles? Is there some way of
> pre-loading tiles to the local cache so in case of loss of signal, we still
> get at least a low-res version?

*(The pre-load. The fallback — a blurry parent square where a sharp one is
missing — is `/stand-in`, briefed in parallel.)*

## user

Nothing to see. Open the map with a connection and, quietly, the squares
for the whole patch at zooms 12 to 15 — and the town centre at 16 — are
fetched into the phone. Later, in a stairwell with no signal, the map still
draws: sharp when zoomed out over the district, and (with `/stand-in`)
blurry-but-there when zoomed into a street nobody looked at. The gear on the
nøøb sheet says what is stocked.

## spec

**The problem.** Offline, the map could only draw squares somebody had
already looked at: the service worker's `/fresh` policy stores what the page
fetches and nothing else. A canvasser who never zoomed out over the whole
district had nothing when the signal went — the old app's lift-queue lesson
(`notes.md`, memory), met again on a map.

**The premise that changed.** `map.js` sets `keepBuffer: 1` because *"OSM's
policy forbids bulk prefetch and our proxy pays for every miss"*. Since
2026-09-02 the ground is Stadia's Alidade Smooth Dark on a key (free tier,
200k credits a month), and `/tiles` on the mini caches every square on disk —
so the district is fetched from Stadia **once, for every phone**, and a
phone's prefetch costs the mini a disk read per square. `map.js` is not
edited; its comment is history and this node is the reason.

**The area.** The current project's area is the patch `/boundaries` draws:
the `constituency` feature of its file, whose bounding box this node takes
(`feature_Boundaries.data` — the parsed file, held after the map's first
paint). Failing that (`/boundaries` unticked, or its file refused), the box
round the pins on the map, padded to a couple of kilometres; failing pins,
the Sevenoaks District box. `/current-project` names the project the user is
in, but no project carries a boundary yet — a project card with its own
boundary is the named next reading, and it lands in `area()`.

**The plan.** Zooms 12–15 over the whole area's box, then zoom 16 over a
3 × 3 km box at its centre; a cap of 1,500 squares per run, cut from the
end, so the low zooms are always whole. For the Sevenoaks constituency
(0.042–0.300 E, 51.218–51.434 N) the count is 20 + 63 + 221 + 825 at zooms
12–15 and 81 at 16: **1,210 squares**, ~15 MB on the dark ground (a square
is ~12 KB). The district box would be 1,851 and stops at the cap with
zoom 15 partial.

**The stocking.** Only while the page is `visible`, only while the map view
is up (`#mapData`, `/map`'s sign, as `/live` reads it), only while
`navigator.onLine`. Four squares in flight, a 300 ms pause between batches
— a full run is about two minutes. Each square is a plain `fetch` of the
same url the map itself asks for: `tiles/{z}/{x}/{y}.png?g=N` with
`/fresh-tiles`' ground tag, read from `feature_FreshTiles.TAG` so the two
can never drift (bare without that node, as the map is). The service
worker's `/fresh` policy — network first, `cache.put` on every ok answer —
makes the fetch the cache write; on a page the worker does not control yet
(its very first load) the answer is put into the same cache, `miso`, by
hand. Verified in the rig with `caches.match`, not assumed.

**Once per ground and area.** A record in `localStorage` (`miso.stocked`)
holds the key — `g=3|patch:E14001465` — and the counts: done, total,
missed, when last full. Keys only, never a position: the pins fallback keys
by its zoom-12 square range (10 km squares). A new ground tag or a new area
starts a fresh run; the same key resumes where it stopped. A run stopped by
hidden, offline or leaving the map view aborts the batch in flight and
keeps its place.

**The wire.** Where the platform says (`navigator.connection`: `saveData`,
or `type === 'cellular'`), zooms 12–14 go on any wire and 15 and the centre
wait for wifi. iOS has no `navigator.connection`, so on the phone this
never fires and the cap is the protection.

**The engineer line.** The gear's section (`/engineer`) gets one tenant
line: `stocked: 1210 of 1210 squares, zooms 12–16, ground g=3, last full at
<time>`. Nothing on the user surface.

## hostile cases

- **The wire goes mid-run.** The batch's fetches fail, the run stops with its
  place kept; the `online` event or the next paint resumes it.
- **Hidden mid-run.** `halt()` aborts the batch; visible again, `kick()`
  resumes from the record.
- **Leaving the map view.** `/map`'s `sync` finds no `#mapData`; the wrapper
  halts. Grid and list views make no requests.
- **A square the proxy cannot get** (upstream missing): a 404, counted done
  and `missed`, not retried in this run. The engineer line says how many.
- **The proxy is down entirely.** Every square 404s; the run walks the plan
  and records it full with every square missed — the count says so. The
  next ground tag starts over.
- **No Cache API** (a browser without it): nowhere to stock; nothing runs.
- **The service worker is not yet controlling** (first load): the page puts
  each answer into cache `miso` itself, so the first visit stocks too.
- **`/boundaries` still loading at the first paint.** `area()` answers null
  up to twelve times, a second apart, then falls through to the pins.
- **A repaint every few seconds** (live pins arriving): `kick()` returns at
  once while a run is up.
- **The cache is evicted** (storage pressure; Safari's seven-day rule for a
  tab, not for an installed app): the record says stocked and the cache is
  empty. Named risk; a "clear it" action behind the gear is the parked
  repair, and a new ground tag restocks.
- **`/fresh-tiles` unticked.** Bare urls, the same the map asks for.
- **`/fresh` unticked** (no service-worker caching): the page's own
  `cache.put` still stores each square, but nothing serves from the cache
  — stocking is inert, and harmless.
- **`/engineer` unticked.** No section, no tenant; the stocking runs.
- **This node unticked.** No stocking, no record read, the map as today.

## parked, and named

- A "keep this area" button — a deliberate run on demand. The quiet run is
  the ask.
- High-zoom stocking of the whole district (16 everywhere is ~5,000 squares).
- "Stock my ward at street level": a per-area entry in `zoomsFor()`.
- "How much is stored": extend the engineer line with a `storage.estimate`.
- "Clear it": a tenant action behind the gear that deletes the `tiles/`
  entries from cache `miso` and drops the record.

## glossary

- **stocked**: a square fetched on purpose into the service worker's cache,
  ahead of anyone looking at it.
- **run**: one walk of the plan for one (ground tag, area) key.

## code description

`stocked.js` — `feature_Stocked`. At load it takes `feature_Map.sync` by
property replacement (`/boundaries`' idiom, typeof-guarded): on the map,
`kick()`; off it, `halt()`. `visibilitychange`, `online` and `offline` do
the same.

`kick()` checks `may()` (visible, online, `#mapData` and a mounted map) and
`canStore()`, asks `area()` — the constituency's bbox from
`feature_Boundaries.data`, else the padded pins box, else `BOX` — builds the
plan with `build()` (`squares()` per zoom over the box, the centre box at
`CENTRE_ZOOM`, cut at `CAP`), loads or starts the `localStorage` record under
the key `<tag>|<area key>`, and starts `run()`.

`run()` walks the plan from `record.done` in batches of `LANES`, an
`AbortController` per batch, `PAUSE_MS` between; a metered link caps the
walk before zoom 15; a `net` result ends the run with its place kept; the
end of the plan stamps `full` and refreshes the engineer section.

`one(t)` fetches `url(t)` — `tiles/z/x/y.png` plus `?g=N` from
`feature_FreshTiles.TAG` — and, when no service worker controls the page,
puts the answer into cache `miso`. `tile()`, `squares()` and `bboxOf()` are
the slippy-map arithmetic. `text()` is the engineer line.

`stocked.index.js` — the `/engineer` tenant: captures `feature_Engineer.fill`,
calls it, appends `#stocked` with `feature_Stocked.text()`. The stocking
object is reached at fill time, since this fragment is composed before it.
