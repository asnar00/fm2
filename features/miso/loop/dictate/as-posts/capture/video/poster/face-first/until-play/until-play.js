// the face comes up and waits: no play but a finger's, and the picture and the
// player are carried across a repaint rather than made again.
{
  if (typeof feature_Poster !== 'undefined' && typeof feature_Loop !== 'undefined') {
    // ---- the play belongs to the tap ---------------------------------------
    // /poster sets `replaying` around the re-open a repaint owes an open clip.
    // On that road the clip is put back as it stood and never started; a
    // finger's open still starts inside its own gesture, which is where a
    // browser will give it sound.
    const fm_untilStart = feature_Poster.start.bind(feature_Poster);
    feature_Poster.start = function (h) {
      if (feature_Poster.replaying) return;
      return fm_untilStart(h);
    };

    // ---- open is remembered for the visit, not the session ------------------
    // `opened` never forgot, so a post played once played itself every later
    // time it was opened — the clip's holder is drawn as a poster and the
    // restore that follows the paint re-opened it, inside the tap that opened
    // the post. A clip whose holder has left the screen is closed again here.
    const fm_untilApply = feature_Loop.apply;
    feature_Loop.apply = function (p) {
      fm_untilApply.call(this, p);
      try {
        for (const id of Object.keys(feature_Poster.opened)) {
          if (!fm_untilHolder(id)) delete feature_Poster.opened[id];
        }
      } catch (e) { /* the page is as the paint left it */ }
    };

    // ---- the paint carries the picture and the player ----------------------
    // A render is a whole-DOM swap. The poster became a fresh <img> that shows
    // nothing until its bytes are back — four of them in the three seconds
    // after a post opens, which is the flashing — and an open clip became a
    // fresh <video> that loaded from the start again. Both elements are moved
    // into the new DOM instead of being remade, so neither blinks and the clip
    // keeps its place. The move happens inside the paint, before the restore
    // that follows it, so /poster's own re-open finds the player already there
    // and /capture/video's mount leaves it alone.
    const fm_untilPaint = feature_Loop.paint;
    feature_Loop.paint = function (html) {
      const kept = {};
      try {
        for (const h of document.querySelectorAll('[data-vid]')) {
          const id = h.getAttribute('data-vid');
          if (!id) continue;
          const v = h.querySelector('video');
          kept[id] = { img: h.querySelector('.poster-frame img'), video: v,
                       ran: !!v && !v.paused };
        }
      } catch (e) { /* nothing to carry */ }
      fm_untilPaint.call(this, html);
      try {
        for (const h of document.querySelectorAll('[data-vid]')) {
          const k = kept[h.getAttribute('data-vid')];
          if (!k) continue;
          const now = h.querySelector('.poster-frame img');
          // only a picture that is already decoded, and only the same one: a
          // poster that changed under the paint must load the new bytes.
          if (k.img && now && now !== k.img && k.img.complete
              && now.getAttribute('src') === k.img.getAttribute('src')) {
            now.replaceWith(k.img);
          }
          if (k.video && !h.querySelector('video')) {
            h.insertBefore(k.video, h.firstChild);
            // a browser pauses a media element that leaves the document, and
            // does it however quickly the element comes back — the pause
            // arrives in a task of the browser's own, after this one. So a
            // clip that was running is set going again when that pause lands.
            // This is a resume of the finger's own play, never a start: `ran`
            // was read off the element before the paint.
            if (k.ran) fm_untilKeepGoing(k.video);
          }
          delete kept[h.getAttribute('data-vid')];
        }
        // a clip whose holder did not come back is off the screen, and a
        // detached <video> goes on playing until it is collected: the post was
        // closed, so the sound stops with it.
        for (const id of Object.keys(kept)) {
          const v = kept[id].video;
          if (v && !v.paused) { try { v.pause(); } catch (e) { /* already gone */ } }
        }
      } catch (e) { /* the new DOM stands as it was painted */ }
    };
  }
}

// a clip carried across a paint, set going again where it was. The browser's
// own pause for the move lands in the next task or two, so the resume waits
// for it rather than racing it, and gives up after a third of a second: past
// that, a pause is the reader's. A play that cannot be granted is not an
// error here — the clip is on the screen, paused, with its controls, exactly
// as it would have been.
const FM_UNTIL_MOVE = 300;
function fm_untilKeepGoing(v) {
  if (!v) return;
  let done = false;
  const off = () => { if (!done) { done = true; v.removeEventListener('pause', on); } };
  const on = () => {
    off();
    if (!v.isConnected) return;
    const p = v.play();
    if (p && p.catch) p.catch(() => {});
  };
  v.addEventListener('pause', on);
  setTimeout(off, FM_UNTIL_MOVE);
}

function fm_untilHolder(id) {
  for (const h of document.querySelectorAll('[data-vid]')) {
    if (h.getAttribute('data-vid') === id) return true;
  }
  return false;
}
