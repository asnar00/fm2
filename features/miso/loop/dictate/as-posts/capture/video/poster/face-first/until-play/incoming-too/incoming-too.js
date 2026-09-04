// a post that arrives under the finger shows a still, not a clip loading. The
// rule is /until-play's — the media waits for a tap — carried onto the road
// where the card ARRIVES rather than only the one where it is repainted.
const feature_IncomingToo = {
  // /poster's own play mark, drawn ink in a thin ring: the two roads into a
  // clip have to look like the same thing, and a post with no face has no
  // frame of its own to put the mark over.
  MARK: '<svg class="icon-svg" viewBox="0 0 24 24" aria-hidden="true">'
      + '<circle cx="12" cy="12" r="10.4" fill="rgba(16,16,18,0.55)" '
      + 'stroke="currentColor" stroke-width="1.5"/>'
      + '<path d="M9.8 7.9l6.4 4.1 -6.4 4.1z" fill="currentColor"/></svg>',

  // a player row the reader has not asked for. /poster's row is a picture and
  // is never one of these; this is the bare row a post whose face never
  // arrived is drawn with.
  waiting(h) {
    const id = h.getAttribute('data-vid');
    return !!id && !feature_Poster.opened[id];
  },

  // the still stands exactly where the clip will be — same square, same
  // ground, same corner — so nothing moves when the clip goes in.
  dress() {
    for (const h of document.querySelectorAll('.post-video[data-vid]')) {
      const has = h.querySelector('.vid-still');
      if (!this.waiting(h)) { if (has) has.remove(); continue; }
      if (has) continue;
      const s = document.createElement('span');
      s.className = 'vid-still';
      s.innerHTML = this.MARK;
      h.insertBefore(s, h.firstChild);
      // the blob URL made while the still is showing, so the tap can mount in
      // one turn and the play lands inside its own gesture — /poster's warm,
      // for /poster's reason, on the road /poster does not own.
      feature_Poster.warm(h.getAttribute('data-vid'));
    }
  },

  // the reader has asked: the clip is the reader's now, so the still goes, the
  // player mounts and the play happens inside the tap.
  open(h) {
    const id = h.getAttribute('data-vid');
    if (!id || h.querySelector('video')) return;
    feature_Poster.opened[id] = true;
    const s = h.querySelector('.vid-still');
    if (s) s.remove();
    feature_Video.mount();
    feature_Poster.start(h);
  },
};

{
  if (typeof feature_Poster !== 'undefined' && typeof feature_Video !== 'undefined'
      && typeof feature_Loop !== 'undefined') {
    const I = feature_IncomingToo;

    // ---- the clip is not touched until it is asked for ----------------------
    // /capture/video's mount gives a holder a <video> with `src` and
    // preload="metadata", and the browser then fetches, decodes and paints a
    // frame — and seeks to where the clip had got to if it has been played.
    // That is the scanning: it happened two seconds after the card arrived,
    // with no finger anywhere near it. A holder the reader has not opened is
    // hidden from mount by lifting its handle for the length of the call.
    const fm_incMount = feature_Video.mount;
    feature_Video.mount = function () {
      const held = [];
      try {
        for (const h of document.querySelectorAll('.post-video[data-vid]')) {
          if (!I.waiting(h)) continue;
          h.setAttribute('data-vid-waiting', h.getAttribute('data-vid'));
          h.removeAttribute('data-vid');
          held.push(h);
        }
      } catch (e) { /* mount sees what it sees */ }
      try {
        return fm_incMount.call(this);
      } finally {
        for (const h of held) {
          h.setAttribute('data-vid', h.getAttribute('data-vid-waiting'));
          h.removeAttribute('data-vid-waiting');
        }
      }
    };

    // ---- the still, drawn on every paint ------------------------------------
    // after /poster's own restore, which is where an opened clip is put back,
    // so a holder that is about to become a player is never dressed as a still
    const fm_incApply = feature_Loop.apply;
    feature_Loop.apply = function (p) {
      fm_incApply.call(this, p);
      try { I.dress(); } catch (e) { /* the row is as /capture/video drew it */ }
    };

    document.addEventListener('click', (e) => {
      if (!e.target || !e.target.closest) return;
      const h = e.target.closest('.post-video[data-vid]');
      if (!h || !I.waiting(h)) return;
      I.open(h);
    });

    try { I.dress(); } catch (e) { /* nothing on the page yet */ }
  }
}
