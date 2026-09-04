// at the ends of the list the sweep pulls against a spring instead of walking
// off a cliff: the card follows the finger less and less, and comes back. No
// event is sent, so the same post is never flown off the screen and returned.
const feature_RubberBand = {
  // how far the card can be pulled past the end, however hard you pull. The
  // curve is the platform's: the first pixels track the finger almost exactly
  // and the last ones hardly move.
  PULL: 160,

  ids: [],     // the surface's own list, as the map last wrote it

  // /reel writes the set's ids on #mapData — the same `cards` vector /browse
  // hands the surface, which is the list /flick walks. It is not on the page
  // while a card is open (Rust draws the page instead), so it is kept from the
  // last paint that had it: the world has not changed under the sweep.
  remember() {
    const data = document.getElementById('mapData');
    if (!data) return;
    const raw = data.getAttribute('data-ids');
    if (raw === null) return;
    this.ids = raw.split(',').filter((x) => x);
  },

  // is the sweep asking for a post that is not there? `dy` negative is the
  // next post (up at the bottom), positive the previous one.
  end(dy) {
    const page = document.querySelector('.card-page');
    if (!page || !this.ids.length) return false;
    const at = this.ids.indexOf(page.getAttribute('data-card') || '');
    if (at < 0) return false;                  // not this surface's list
    return dy < 0 ? at >= this.ids.length - 1 : at <= 0;
  },

  // the same question from the direction /flick names
  endOf(dir) { return this.end(dir === 'next' ? -1 : 1); },

  // the finger's travel, damped: asymptotic to PULL, so the card can never be
  // dragged clear of the screen and every pixel of the pull is resisted a
  // little more than the last.
  damp(dy) {
    const s = dy < 0 ? -1 : 1;
    const d = Math.abs(dy);
    return s * (1 - 1 / (1 + d / this.PULL)) * this.PULL;
  },
};

{
  if (typeof feature_CarriesTheCard !== 'undefined' && typeof feature_Flick !== 'undefined'
      && typeof feature_Loop !== 'undefined') {
    const R = feature_RubberBand;

    // ---- the pull ----------------------------------------------------------
    // the parent is handed a damped finger rather than a damped answer: it
    // then computes its own `dy`, its own transform and its own spring-back
    // from the same number, and none of its rules need to know about this.
    const fm_rubMove = feature_CarriesTheCard.move;
    feature_CarriesTheCard.move = function (x, y) {
      const a = this.at;
      if (!a) return fm_rubMove.call(this, x, y);
      const raw = y - a.d.y;
      if (!R.end(raw)) return fm_rubMove.call(this, x, y);
      return fm_rubMove.call(this, x, a.d.y + R.damp(raw));
    };

    // ---- and no switch at the end ------------------------------------------
    // /flick's release still fires — its threshold is the finger's own travel,
    // not the card's — so the stop is here, outside /carries-the-card's own
    // wrapper, which is what keeps the card from being flown off and brought
    // back to say "there is nothing that way".
    const fm_rubGo = feature_Flick.go;
    feature_Flick.go = function (dir) {
      if (R.endOf(dir)) return;
      return fm_rubGo.call(this, dir);
    };

    // the list, kept from every paint that carries it
    const fm_rubPaint = feature_Loop.paint;
    feature_Loop.paint = function (html) {
      fm_rubPaint.call(this, html);
      try { R.remember(); } catch (e) { /* the last list stands */ }
    };
    try { R.remember(); } catch (e) { /* nothing on the page yet */ }
  }
}
