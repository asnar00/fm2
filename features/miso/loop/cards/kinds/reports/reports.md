# reports
*ask a question of everything you have collected, and get a PDF back*

> (transcripts/2026-09-01-saturday.md#p2)
> I demoed to Tara - she liked it, and our first actual canvassing session is on Saturday! So we have some work to do to get ready. In no particular order: 1] show constituency and ward boundaries on the map; 2] clean up the "make posts" interface to include video, audio, photo+type, transcription 3] make a quicker QR-code based invite workflow that we can use instantly during canvassing 4] AI interrogation / report generation.

> (the same session, ash answering triage's did-you-mean on ask 4)
> there can be multiple reports (in a new reports tool) and we can make new ones - old ones get updated. to start with they'll just be one query per report. Report will be a PDF including maps that can be shared with others.

## user

Support and above get another glyph in the toolbar: a sheet of paper. Tap it
and you see your reports, newest first, with what each one asked and when it
last answered.

Make one by giving it a name and a question — *what did people say about
parking, by ward?* — and pressing **make**. The report says "working" while it
thinks, for anything from a few seconds to a couple of minutes, and then says
"ready". Tap **open** and the PDF opens: your question, the answer written out,
and a map of where the posts it read came from. From there it is the phone's
own share sheet, so it goes wherever you send things.

Ask it again whenever you like — the refresh button re-runs the same question
over everything you have collected since, and the report you already have is
replaced. It always reads your world and nobody else's, so a report can never
tell you something you could not already see by scrolling.

A member — anyone below support — has no reports glyph at all, and every one
of the reports doors is shut to them.

## spec

A **report** is a card of type `report`, so the list, the page, the title, the
undo and the exchange all come from `/cards` and nothing is invented. Its
`blocks` are the card's ordinary three — a `title` (the report's name) and a
`text` (the **query**, one per report, editable in place) with the picture
block left unused and undrawn — plus one block of kind `report` that carries the
state: `{kind:"report", status, pdf, generated, through, n, note}`. An unknown
block kind renders as nothing (`/cards`), so the state block is invisible to
every other surface — `/location` established that idiom and this follows it
exactly rather than adding a top-level field that `/guard`'s merge and
`/exchange`'s copy would have to learn about.

**`status` is the whole of the state machine**, and it is a string on the card
rather than anything server-side: `new` (never run), `working`, `ready`,
`failed`, `nokey`. That means the phone's answer to "what is happening" is a
card read, it survives a restart of either half, and it travels to the person's
other devices for free.

**Who may.** Reports read the whole of a person's collected world and spend
somebody's money doing it, so the rung is `/authority`'s `may_write_shared` —
`authority_rank >= 2`, support and above, the same rung inviting takes. The
toolbar asks once at load (`GET reports/may`) and the glyph appears only if the
answer is yes, which is `/invite-tool`'s pattern whole; and because a toolbar is
a decoration and not a gate, **each of the three routes checks the rung itself**
before it does anything. This node does not compose without `/authority`, by
design: a gate that could be unticked into nothing is not a gate.

**The query is asked of Claude, over raw HTTP, by curl.** The server is Rust and
there is no official SDK; TLS is curl's problem, which is `/vonage`'s precedent
and `/tiles`' after it, and it is why this node needs no crate.
`POST https://api.anthropic.com/v1/messages` with `x-api-key`,
`anthropic-version: 2023-06-01`, `anthropic-beta: server-side-fallback-2026-07-01`
and a body carrying `model` (`claude-opus-5`), `max_tokens` 16000,
`"fallbacks":"default"`, the system instructions and one user message. The reply
is parsed with serde_json and never with a regex, and **`stop_reason` is read
before `content` is**: `refusal` is a real stop reason on this model family and
arrives with HTTP 200, so a refused generation lands as a `failed` status with
the model's own words on the card, never as a blank PDF and never as a hang.
`max_tokens` is not a failure — the text is kept and the report says it was cut
short.

**The key is never on argv and never in a log.** `/off-argv` is the precedent
and it is followed in both halves: the key is read from
`~/.agent-config.json`'s `anthropic.api_key` — the same file `/vonage` reads its
credentials from — with the `ANTHROPIC_API_KEY` environment variable as an
override for a box that would rather set it there, and it reaches curl inside a
`-K -` config on stdin, where no local `ps` can see it. The request body, which
is large, goes to an owner-only temp file that is removed whether the call
worked or not. Nothing prints the key, its length, or its prefix.

**With no key the tool is still there and says so.** `reports/may` reports
`key: false`; making a report works, and running it stamps `status: "nokey"` and
a line saying the server has no API key. Nothing hangs, nothing spins, and the
node builds, toggles and rigs with no key present at all — which is how it was
proven.

**The data is what the asker can already see.** `exchange_cards_of(who)` is
their own world, which is the only world this node ever reads: their posts (the
words in every text block, the author, the time, the place) and their projects
(name and mission, as context for a question that mentions one). Nothing is
gathered from anybody else's world, so a report cannot exceed its reader's own
visibility — the guarantee is structural rather than a filter, because there is
no other world in scope.

**The payload is capped by recency, out loud.** `reports_post_cap()` is 300
posts, newest first, and `reports_text_cap()` is 160000 characters; whichever
bites first, bites, and the footer of the PDF says exactly which posts were read
(`the newest 300 of 812 posts`) and the date the data runs through. Silent
truncation is the failure this cap exists to avoid.

**The PDF is composed here and printed by Chrome.** The server writes a
print-styled HTML file — the model's answer, run through this node's own small
markdown renderer, plus a map section and a footer — and runs headless Chrome
with `--print-to-pdf` over a `file://` URL, in a throwaway user-data directory
so it can never touch anybody's profile. Chrome is already on the box for the
smoke gate; `MISO_CHROME` points this elsewhere, and a missing Chrome is a
`failed` status with a sentence, not a crash.

**Chrome does not reliably exit when it has finished printing**, and that is the
ordinary case here rather than a rare one: on this box it writes the file in
about four seconds and then sits there indefinitely. The first cut waited for
the process, so the generation thread never returned, nothing was ever stamped,
and every report said `working` for ever with a finished PDF sitting beside it —
found on the rig, and the exact shape of "what happens when it hangs". So the
finished **file** is the signal and not the exit status: the printer is polled
for an output whose trailer (`%%EOF`) is written, given a moment to close the
handle, and then killed. A printer that has produced nothing by
`reports_print_ms` (two minutes) is killed too, and that is a `failed` with a
sentence on the card.

**The map is drawn by the server, not by Leaflet.** The brief's route was
Leaflet over `/tiles`, and it was not taken: a headless Chrome loading our own
page would arrive without a session cookie (`/tiles` is behind the gate), would
have to be handed a credential to get past it, and would then have to be raced
for "has the map finished drawing" before the print. Fetching the tiles
server-side removes all three problems and the whole of the third party from the
print step: the mosaic is computed in Rust (web-mercator pixel coordinates, a
zoom chosen to fit the pins), the PNGs are inlined as `data:` URIs, and the pins
are absolutely-positioned marks. The picture of the world is still ours and
still cached on our own disk, which is the doctrine the Leaflet route was
serving.

The **print basemap is a different source from the screen's**, and that is
`/taste` 9 rather than an oversight: the app's map is a dark basemap because the
app is dark, and a dark map on a page that prints on paper is either a great
deal of ink or a filter working to correct an asset — the standing "no". So
`reports_map_source()` is a light one, cached under `<context>/tiles-print/`,
with `MISO_PRINT_TILE_URL` to point it anywhere. It is this node's own twenty
lines rather than a call into `/tiles`, which is the other half of the reason:
with its own source and its own cache, the reports node has no compile-time
dependency on `/tiles` at all and toggles on its own. The credit line
OpenStreetMap's licence asks for is printed under the map.

*Which* light source was found on the rig rather than chosen from a list. The
first cut used CARTO's `light_all`, the sibling of the dark basemap the app
already draws; the printed map came back with **API KEY REQUIRED** written
diagonally across every tile in grey. `basemaps.cartocdn.com` now answers 200
with a watermarked tile instead of refusing, so nothing in the fetch path could
have noticed. A watermark is not something to filter off — it is the wrong
source — so the default is OpenStreetMap's own standard rendering
(`tile.openstreetmap.org`), which is open, needs no key, and is drawn light. Its
tile policy asks for a User-Agent naming the application and a way to reach
whoever runs it, which `reports_agent` sends, and for modest use, which a cached
mosaic per report is.

**This is also a live defect in the app's own map, and it is not this node's to
fix**: `/tiles` defaults to `basemaps.cartocdn.com/dark_all`, and a tile fetched
from there today carries the same watermark (checked directly, 2026-09-01). Only
tiles the mini has already cached are clean, so the map degrades as canvassers
walk into ground it has not seen. `MISO_TILE_URL` is the one-line mitigation and
`/tiles`' own default is the real fix; both are named here rather than done,
because neither is this node's file.

`reports_map_overlay()` is the named seam for a ward-boundary overlay: it
returns markup absolutely positioned in the mosaic's own pixel space and
defaults to nothing, so the boundaries node being built in parallel can join by
redefining one function and drawing into the coordinates it is handed. No file
of that node's is touched or assumed here.

**The PDF never rides a card.** `misses.md`'s picture cap is the reason stated
in one line: the whole cards list travels as one `/msg` op with a 64KB body, and
a PDF is hundreds of kilobytes. The bytes go to `/mirror`'s blob store —
`~/.miso-blobs/<user>/report.<card id>.pdf`, the same per-user directory the
recordings use — and the card carries only the file's name. It is served by this
node's own route rather than `/blob/`, for one reason worth the route: the
response's content type is `application/pdf` and the URL's last path segment is a
slug of the report's title, so what the phone hands to the share sheet is
*parking-by-ward.pdf* and not an octet-stream blob.

**Generation answers immediately and finishes later.** `POST reports/run`
stamps `working` into the caller's world, spawns a thread and returns at once,
because a route that took two minutes would be a phone that looked broken. The
thread's result is written back through the same door `/exchange` gives a card
by — a `CtxOp` on the `cards` var handed to `handle_msg` with the context set to
that user — so `/guard` merges it (nothing of theirs can be displaced),
`/remember` logs it, and `/converge` relays a `CtxUpdate` to their open pages.
The report page therefore updates itself when the answer lands, with no polling
and no clock: there is no clock to have. `misses.md`'s "clock in wasm" is
obeyed — every time in this node is either `now_ms()` on the native side or
`Date.now()` riding the event from the page.

**Refresh is the same run again.** The refresh control posts the same
`reports/run` for the same card; the query is whatever the text block says now,
so editing the question and refreshing is how a report is re-aimed. The PDF is
written to the same name and replaces the old one; `generated` and `through`
move.

## hostile cases

- **A member (rank < 2) asks.** No glyph, and `reports/may`, `reports/run` and
  the PDF route each answer 403 on their own account. Proven in the rig.
- **No API key anywhere.** The tool is present, the list works, making a report
  works, running it stamps `nokey` and the page says *no API key on the server*.
  Nothing hangs.
- **The model refuses** (`stop_reason: "refusal"`, HTTP 200). `failed`, with the
  refusal's own words on the card and no PDF written — the previous PDF, if
  there was one, is left alone rather than replaced by a blank.
- **The API errors, or curl cannot reach it.** `failed` with the error's message
  (or *the report service could not be reached*); the same rule about the old
  PDF applies.
- **The answer is cut at `max_tokens`.** Kept, printed, and the footer says
  *the answer was cut short*.
- **Chrome is missing or fails.** `failed`, *could not print the report*. The
  HTML is left beside the blob so the failure is inspectable.
- **Refresh pressed twice.** A run whose card says `working` and whose stamp is
  under ten minutes old is refused with *already working*; older than that the
  card is stale — a server restarted mid-generation leaves exactly this — and
  the run is allowed. So a wedged report always has a way back, and a racing
  double-tap does not spend twice.
- **The server restarts mid-generation.** The card stays `working`; after ten
  minutes the refresh works again and the page says so. Nothing is lost but the
  run.
- **A world with no posts.** The report is written anyway and says there was
  nothing to read; the map section is left out rather than printed empty.
- **A world with 5000 posts.** The newest 300, or 160000 characters, whichever
  comes first, with the footer naming the cut.
- **A card id from another user, or with a slash in it.** The run and the PDF
  routes both check the card is in the caller's own world and that the id is
  ordinary card characters, so no path this node accepts can name a file outside
  the caller's own blob directory.
- **The model wraps its lines.** It does, always, and the first cut broke every
  wrapped bullet in two — half the sentence in the list, half orphaned in a
  paragraph under it (rig-found, and visible only by looking at the printed
  page). A plain line under a list item is that item continuing, which is
  markdown's lazy continuation and is what the renderer does now.
- **A report card meets `/location`'s pill.** Every card page carries a **map
  location** offer; on a report it is a control about nothing, so this node —
  which is newer, and therefore outside `/location`'s link — takes it out of the
  page it is handed, exactly as it takes out the unused picture block.
- **A title full of punctuation.** The slug in the URL is alphanumerics and
  hyphens only, capped at 40 characters, and is used for the *filename* — the
  file itself is found by the `id` parameter, so the slug can be anything at all
  without reaching the disk.

## parked

Named because the Saturday session will ask for them, and each is a new node
extending a seam this one leaves, not a change to what is built here: more than
one query per report; scheduled or automatic refresh; a report about one ward or
one project (`reports_corpus` is the seam — narrow what it gathers); comparing
two date ranges (the same seam, with a window); an outward-facing share link (an
ash ruling, deliberately not built — sharing today is the phone's own share
sheet on a downloaded file); interrogation as a conversation in the app.

## glossary

- **report**: a card of type `report` — a name, one query, and the PDF its last
  run produced.
- **query**: the one question a report asks, held in the card's text block.
- **corpus**: the digest of the asker's own world that the query is asked
  against — their posts and projects, capped by recency.
- **the state block**: the `report` block on the card carrying status, the PDF's
  name and the times.

## code description

`reports.md` is this file; the node's code is `reports.rs` (server and loop),
`reports.js` (the page half) and `reports.css` (the in-app surface). The PDF's
own stylesheet is written into the print HTML by `reports.rs` and is deliberately
not in `reports.css`: the app is dark and the artifact that leaves it is light,
and keeping the two apart is what stops one being edited into the other.

`reports.rs` extends `tools_list` with the paper glyph, gated on
`s["reports"]["may"]` — the toolbar reads what the server said — and `render`
draws the tool's surface when `open_tool_read()` is `reports`. The surface is
this node's own rather than `/browse`'s: reports have no pictures, so a grid
view and a view picker would both be furniture for nothing, and the list is the
`.crow` grammar directly (`/taste` 6).

`reports.rs` extends `update` with three events. `ReportsMay {may, key}` is the
server's answer landed in the loop state, `/invite`'s `InviteList` exactly.
`ReportNew {owner, title, query, t}` makes the card — `card_new`, the title and
the query written into their blocks, a `new` state block appended — and opens
it, so the id is `<owner>.<t>` and the page half knows what it just made without
waiting for a paint. The tool's own back-tap is the navigation, `/browse`'s
grammar.

`reports.rs` extends `route` with three doors, each of which reads the cookie,
resolves the caller and checks `authority_rank >= 2` for itself.
`GET reports/may` answers `{ok, may, key}`. `POST reports/run {id}` finds the
card in the caller's own world, refuses a fresh `working`, stamps `working`,
spawns the generation thread and returns. `GET reports/<slug>.pdf?id=<card id>`
answers the stored bytes as `application/pdf`.

`reports_generate(who, id)` is the thread. It sets the context to that user,
reads the card, gathers `reports_corpus`, calls `reports_ask`, renders
`reports_html`, prints it with `reports_pdf`, and stamps the outcome through
`reports_stamp`. Every early return stamps something: there is no path out of
this function that leaves a card saying `working`.

`reports_corpus(who)` is the data seam and returns one JSON object — `text` (the
digest the model reads), `n` and `total` (what was read, of what), `through`
(the newest post's time) and `points` (the located posts, for the map). It reads
`exchange_cards_of(who)` and nothing else. A narrower report — one ward, one
project, one week — is this function's redefinition and nothing else's.

`reports_ask(query, corpus)` is the model seam: it builds the request, writes the
body to an owner-only temp file, runs `curl -K -` with the key on stdin, removes
the temp file, and returns `{ok, text, why, cut}`. `reports_key()`,
`reports_model()` and `reports_system()` are the three things about the call that
a later node would want to change.

`reports_html(...)` composes the print page: the header (the miso wordmark, the
report's name, the query), the answer through `reports_md` — a small strict
markdown renderer covering headings, bullets, numbered lists, pipe tables, bold
and italic, over HTML-escaped text, so nothing the model writes can put markup on
the page — the map section, and the footer with the generated date, the
data-through date and the cap note.

`reports_map_html(points)` is the mosaic: `reports_zoom_for` picks the largest
zoom whose bounding box fits the print frame, `reports_px_x` and `reports_px_y`
are the web-mercator pixel coordinates, `reports_tile` reads the print-tile cache
and fetches on a miss, and each tile is inlined as a `data:` URI. The mosaic is
built at twice the width it is printed at and scaled by half, so the tiles land
on paper near 190dpi rather than 96 — the difference between readable street
names and a smear. `reports_map_overlay` is the ward-boundary seam, drawn over
the tiles and under the pins. The map section avoids being split across a page
but does not force a break, so a short report keeps its map on page one.

`reports_pdf(html, out, work)` runs and watches the printer: `reports_pdf_whole`
is the "is this a finished document" test the watch turns on, and
`reports_print_ms` is how long a printer may take before it is abandoned.

`reports_stamp(who, card)` writes one card into a named world through
`/exchange`'s door: a `CtxOp` set carrying that card alone, handed to
`handle_msg` with the context user set, which `/guard` merges by id. The card's
`edited` moves on every stamp, because `/guard` keeps the newer edit and a stamp
that did not move it would be discarded.

`reports.js` is the page half: `pull()` asks `reports/may` once at load and sends
`ReportsMay`; the name and question fields are held in a `draft` object outside
the DOM, because `#app` is repainted wholesale (`/invite`'s reason); `make()`
looks the owner's name up the way `/posts` does, sends `ReportNew` and then posts
`reports/run` for the id it just minted; `run(id)` is the refresh. Every control
carries `data-rep` rather than `data-ev`, so the loop never repaints the page out
from under a half-typed question.

`reports.css` styles the tool's surface against `/taste`: the list on the
`#161619` ground the browse list uses, the make box as one card with two fields,
the status line dim, and the one accent — the dusty blue that already means
*chosen* — on the focused field and nowhere else.
