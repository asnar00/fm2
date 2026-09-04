# on-the-reel
*the "words are coming" mark, drawn where a post actually is*

> (transcripts/2026-09-04-field-walk.md#p77)
> we should also show a "transcribing..." indicator on the post with some animation to let the user know that something is happening

## user

Make a post and its lozenge in the band carries a dot, breathing quietly beside the author and the time; open it and the line under the clip reads **transcribing…**, where the words will land. When the words arrive both go and the words are there. If the server is stuck it says **still trying** instead.

## spec

`/shows-progress` gets the state to the phone — the server publishes `Transcribing {working, stuck}` and the world keeps it as `dict_working` — and marks the drawn page with `data-work`. It then drew that mark on two surfaces that have both moved out from under it: the grid tile, which `/map-only` took away, and `/as-posts`' audio play row, which a post of the current shape does not have — `/video-only` made every post a clip, so its media holder is `/poster`'s frame or `/capture/video`'s player. The state arrived and nothing showed.

**Nothing changes on the server side, and nothing needs to.** `shows_progress_mark` puts `data-work` on whatever carries `data-rec="<id>"`, and both `/poster`'s frame and `/capture/video`'s player row carry exactly that. So the open post's mark is already on the right element; what was missing was a rule that draws it. This node is that rule.

**The lozenge is the other half, and it is the page's.** The band is drawn in JavaScript, outside the loop's own html, so the server's marking cannot reach it: the world says which recordings are working, the cards say which post owns each recording, and the lozenge for each is marked after every sync — set on the ones that are waiting and taken off the ones that are not, so the mark goes the moment the words land with nothing needed to clear it. `stuck` outranks `working` for a recording in both lists.

**The manner is the parent's**, deliberately: one dot at its `transcribing-breath` rate rather than a second set of keyframes, never a spinner, and steady under `prefers-reduced-motion` — the mark is the information, the breathing only the manner. On the lozenge the dot sits beside the author and the time, because a band 66% of the width wide has no room for a word. On the open post the clip's length is hidden rather than removed while the words are coming, so the line keeps its height and nothing moves under it when they arrive.

Untick and the state still arrives and still marks the page, and still nothing draws.

## hostile cases

- **A post whose words have landed.** Not in either list; the mark is taken off the lozenge on the next sync and the page is drawn without `data-work`.
- **A recording in both lists.** `stuck` wins — the more informative word.
- **A post that is not in the band** (another surface, a set that sifts it out). Nothing to mark there; the open post's line is unaffected.
- **The band not drawn yet.** No list; the next sync marks it.
- **A foreign copy.** `/poster`'s dim row carries no `data-rec`, so the server never marks it and nothing draws.
- **`prefers-reduced-motion`.** Both dots steady.
- **A post with no clip** (an older written post). No holder with `data-rec`; nothing to draw.

## glossary

(no new terms)

## code description

`on-the-reel.js` — `feature_OnTheReel`. `waiting()` turns `dict_working` and
the cards into `{card id: 'on'|'stuck'}`; `mark()` sets `data-work` on the
lozenges that are waiting and removes it from the rest. The wrapper on
`feature_Map.sync` runs it after `/reel` has drawn its row, so a rebuilt band
and an unchanged one are both marked.

`on-the-reel.css` — the lozenge's dot beside its meta line, and on the open
post the hidden duration, the breathing dot and the line that reads
`transcribing…` or `still trying`, all at the parent's own rate and with its
own reduced-motion ruling.
