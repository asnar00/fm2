# vocabulary
*the words a clip is likely to contain, taken from where it was made*

> (transcripts/2026-09-04-field-walk.md#p7)
> 3) seed the transcription with words taken from a context document based on our location (streetnames) and maybe a later a briefing document

> (transcripts/2026-09-04-field-walk.md#p10, the amendment approved)
> yeah that sounds like a decent start. Parity with fieldnote would be good since I know that works OK in the field.

## user

Nothing to do. A note recorded on Bayham Road comes back saying "Bayham
Road", not "Beacon Road" — because the machine that transcribes it was told
what streets are nearby before it listened.

## spec

A speech model guesses proper nouns badly and street names worst of all. Both
of ours will take a list of words to expect — whisper as an `initial_prompt`,
Speechmatics as `additional_vocab` — so the list is built once, here, and
both rungs read it.

**The list is the place, from coarse to fine.** The constituency, the
district and the ward the post sits in; the road, quarter, suburb, city and
county the geocoder gives back for its exact fix; then the nearest named
streets and places, nearest first. Fieldnote built the first half of that
list and it worked in the field (`reverseGeocode` and `lookupWard` in its
server), so the two calls are the same two calls: **Nominatim** for the
address and **postcodes.io** for ward, district and constituency, with a
real user-agent, one request each and never in a loop.

**The street list is offline.** `tools/streets.py` pulls every named highway
and place inside the constituency boundary — the boundary being
`/boundaries`' own committed geojson, so nothing here knows the word
"Sevenoaks" — into `~/.miso-context/streets.json`, once. 1,986 of them, for
Sevenoaks. After that the mini needs no network to seed a clip, which matters:
a field day is exactly when the network is worst.

**A geocode is cached and a cache is enough.** The answer for a fix is kept
under `~/.miso-context/geocode/`, keyed to the fix rounded to about a hundred
metres, so a canvasser working one street geocodes it once. If the network is
down and there is no cached answer, the list is the streets alone; if there is
no `streets.json` either, the list is empty and both rungs transcribe unseeded
— worse words, never no words.

**A post with no place gets the constituency and nothing else.** The map pill
is what puts a place on a post, and `/where-taken` asks for one when a
recording is made — but a fix can be refused, and a clip made indoors on a
cold phone may have none.

**Thirty streets, and a cap of forty phrases.** Whisper's prompt window is
about 224 tokens and a phrase is two or three; forty phrases with the
constituency at the front is inside it with room, and Speechmatics takes the
same forty as `additional_vocab` entries. Nearest first, so the cut falls on
the streets a canvasser is least likely to be standing on.

## glossary

- **vocabulary**: the phrases a transcriber is told to expect for one clip,
  ordered coarse to fine and cut at a budget.

## code description

`vocabulary.rs` redefines `transcribe_vocab(card)` and adds nothing else to
any chain.

`vocab_place` reads the post's location block through `/location`'s own
`card_place_of`, so a garbage coordinate reads as no coordinate.

`vocab_geocode(lat, lon)` is the two lookups and the cache: the cache file
first, then Nominatim and postcodes.io through `curl` (`/reports`' way of
reaching the network from this server), then the cache is written — and a
failed lookup writes nothing, so the next clip tries again rather than
inheriting a bad answer for ever.

`vocab_streets(lat, lon)` reads `streets.json` and returns the thirty nearest
names by a flat-earth distance, which is exact enough over a constituency.

`vocab_context_dir` spells out the op store's path rather than borrowing
`/remember`'s, for the reason `/pic-beside` gives for the same two lines.
