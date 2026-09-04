# until-play
*the face comes up still and waits for a finger*

> (transcripts/2026-09-04-field-walk.md#p6)
> 2) the post opens, but then the video flashes a couple of times and auto-plays, which it shouldn't. It should just come up without flashing and wait for you to hit play.

## user

Open a video post and its face is simply there — one still picture, not moving, not blinking, whether or not you have watched this clip before. Nothing plays until you tap the play mark. Tap it and the clip plays and keeps playing: a picture arriving, a place landing, a transcript coming back no longer jog it.

## spec

Ash opened a post on the walk and the video blinked twice and started itself (#p6). Both halves were measured on the rig before anything was changed, opening a seeded video post from the reel:

- opening a post never played once — **four paints in the three seconds after the tap, four `<img>` elements made and three thrown away, no `play()`**. Every paint is `innerHTML` on `#app`, so the face is a new `<img>` each time and shows nothing until its bytes are back. That is the blinking.
- opening a post **played once earlier in the visit — two `play()` calls and one `loadstart`, with no finger anywhere near the play mark**. That is the starting itself.
- and while a clip is open, three paints cost **four `play()` calls, three `loadstart`s, and three `<video>` elements made and thrown away** — the clip restarting under the reader.

The cause of the second is that `/poster` remembers which clips are open in `opened`, and never forgot: the memory outlived the post's visit, so `restore()` re-opened the clip on the *next* opening too — inside the tap that opened the post, which is exactly the gesture a browser will give a `play()` sound for. The cause of the first and third is that a paint remakes both elements.

Three rules, none of them touching another node's files:

**The play belongs to the tap.** `/poster` was refactored to name the play — `start(h)` — and to say which road an open came down: `replaying` is set around the re-open `restore()` owes an already-open clip and cleared in a `finally`. This node redefines `start` to do nothing on that road. A finger's open still starts the clip inside its own gesture, which is where the sound is.

**Open is remembered for the visit, not the session.** After every paint, a clip whose holder is no longer on the screen is forgotten. So a repaint while the post is up still puts the player back, and opening that post tomorrow — or two taps later — shows the face and waits.

**The paint carries the picture and the player.** Before `/loop` swaps the DOM, the live poster `<img>` and any live `<video>` are held by their clip id; after the swap they are moved into the new holder — the picture only when it is already decoded and its `src` is unchanged, the player only when the new holder has none. The move happens inside the paint, before the `restore()` that follows it, so `/poster`'s re-open finds the player already there and `/capture/video`'s `mount()` leaves it alone. A clip whose holder did not come back is off the screen: a detached `<video>` plays on until it is collected, so its sound is stopped here.

Untick and the face blinks on arrival and a post watched once starts itself when it is opened again.

## hostile cases

- **A poster that changed under the paint** (the frame arrived, or a picture replaced it). The `src` differs, so the new `<img>` is left to load its own bytes — one honest blink, once.
- **A picture still loading when the paint comes.** `complete` is false; not carried, and the new one loads as it would have.
- **A clip playing when the post is closed.** The holder does not come back, the element is paused, and `opened` forgets it. Tap into the post again and the face is there; the play mark resumes where the clip had got to (`/capture/video` keeps the position).
- **A repaint that loses the player anyway** (a holder redrawn under a different id). `mount()` makes a new one and `/capture/video`'s own `playing[id]` resumes it — this node suppresses `/poster`'s start, never that.
- **A post with no face at all** (the frame could not be taken). Nothing to carry; the player as before.
- **A foreign copy** (the dim row). No `data-vid`, nothing carried, nothing started.
- **`open()` throwing inside `restore`.** The `finally` clears `replaying`, so the next finger's open is still a finger's.

## glossary

(no new terms)

## code description

`until-play.js` — three wrappers, taken at load.

`feature_Poster.start` is redefined to return without playing while
`feature_Poster.replaying` is set, and to call the captured original otherwise.

`feature_Loop.apply` is wrapped to prune `feature_Poster.opened` after the
paint: an id with no holder on the screen is dropped, so open is a fact about
the visit rather than the session. `fm_untilHolder(id)` is the lookup.

`feature_Loop.paint` is wrapped to carry elements across the swap: the live
`.poster-frame img` and `video` of every `[data-vid]` holder are collected
before, and after the swap each is moved into the holder that came back — the
picture only when it is `complete` and its `src` matches, the player only into
a holder that has none. Anything not claimed by a holder was a clip that left
the screen, and a `<video>` among them is paused.
