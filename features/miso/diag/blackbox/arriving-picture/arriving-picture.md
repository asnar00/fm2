# arriving-picture
*what a card's picture is the moment it reaches the screen, in the record*

> (transcripts/2026-09-04-field-walk.md#p95)
> I'm still seeing major "flashing" of the media panel on posts when scrolling from one to another.

## user

Nothing to see. On the next sweep the phone writes down what its own screen did, so the flash can be read rather than guessed at.

## spec

Three readings of the flash have been measured on the rig and all three were wrong. The black box said there is one paint per sweep and no other event within two and a half seconds of one, so it is not a repaint storm. A cold picture store on a throttled network still painted thirteen of thirteen arriving cards complete, so it is not the fetch. A warm store — the phone's own case, three posts recorded so `/pic-beside` held a copy of each — inserted `blob:` every time with `complete` true and **no request for `pic/…` at all**, so it is not the dressing order either. The symptom is on one device and the rig cannot make it.

**So the device writes it down.** On every paint that leaves a card page on the screen, one line goes into the ring: the picture's `src` by kind — `blob:`, `pic/`, `data:` or none — its `complete` and `naturalWidth` *at insertion*, its `data-away`, whether a `<video>` is there and its `readyState`, which event's turn was being painted, and the milliseconds since the previous card paint. Then, at the next animation frame, a second line: whether that same element is still in the document and whether its `src` changed under it.

**The second line is the one a rig cannot produce.** A source swapped after the element is in the DOM is a blank frame and then a picture, which is exactly what flashing looks like; every rig run has shown the source already right at insertion, so if the phone shows otherwise the second line will say so, with the frame it happened in.

`/unbroken` keeps the outgoing card outside `#app` while a sweep is under the finger, so each line says whether that ghost was present — a sweep's lines are then distinguishable from an opening's without guessing from the event.

**The cost is a few fields on card paints only.** A paint with no card page writes nothing, and the ring, the trim, the flush and the server's ingest are all `/blackbox`'s own, so nothing new travels and nothing new is stored beyond two small entries per card paint.

Untick and the ring holds what it held before.

## hostile cases

- **A paint with no card page** (the map, a tool page). Nothing recorded.
- **A card with no picture at all.** One line with `src: none`; the second line is skipped, there being no element to watch.
- **The card gone by the next frame** (a sweep completing, the post closed). The second line says the element is no longer in the document, which is itself the answer for that paint.
- **`/replay` playing a recording back.** `record` is `/blackbox`'s own and honours its `paused` flag, so a replay is not recorded as if it were a session.
- **The boot payload's paint.** No event was in flight, so the cause is empty rather than the last event's name.
- **`/unbroken` unticked.** The ghost field is simply false.
- **A phone with no network.** The ring is the record; the lines ship when there is a network, like every other entry.

## glossary

(no new terms)

## code description

`arriving-picture.js` — `feature_ArrivingPicture`. `kind(src)` reduces a source
to its shape; `look()` reads the card in `#app`, its first picture and any
player; `after()` writes the line at insertion and schedules the one at the
next frame.

The wrapper on `feature_Loop.send` holds the event's name for the length of the
turn, so a line can say which paint it was. The wrapper on `feature_Loop.paint`
runs after the swap, so what is read is what was inserted.

`tools/sweeps.py` is the reader: it prints a sweep's whole sequence — the
gesture, the switch, and every media line around it — from the live log.
