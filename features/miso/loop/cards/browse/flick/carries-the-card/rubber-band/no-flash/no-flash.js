// the card keeps its layer a few frames past the end of the spring, so the
// pull at the ends of the list settles without the media re-rasterising.
const feature_NoFlash = {
  HOLD: 140,       // how long the promotion outlives the animation that needed it

  timer: null,

  // the layer is held by a class of this node's own rather than by leaving the
  // parent's on: the parent takes its class off to end its own animation, and
  // that is its business — this only keeps the hint alive across the hand-off.
  hold(page) {
    if (!page) return;
    page.classList.add('fm-settling');
    if (this.timer) clearTimeout(this.timer);
    const self = this;
    this.timer = setTimeout(() => {
      self.timer = null;
      try { page.classList.remove('fm-settling'); } catch (e) { /* gone with its card */ }
    }, this.HOLD);
  },

  // one line for the release, so a phone that still flashes says what its media
  // element was at that moment — /arriving-picture writes the same shape for a
  // card arriving, and the ends write none because no card arrives there.
  note(page, when) {
    if (typeof feature_Blackbox === 'undefined' || !page) return;
    const im = page.querySelector('.poster-frame img, img');
    const v = page.querySelector('video');
    const src = im ? String(im.getAttribute('src') || '') : '';
    feature_Blackbox.record({
      type: 'media3', at: when,
      card: (page.getAttribute('data-card') || '').slice(-6),
      src: src.indexOf('blob:') === 0 ? 'blob:' : (src.indexOf('pic/') === 0 ? 'pic/' : (src ? 'other' : 'none')),
      complete: im ? !!im.complete : null,
      w: im ? im.naturalWidth : null,
      video: !!v, ready: v ? v.readyState : null,
      will: getComputedStyle(page).willChange,
    }, null);
  },
};

{
  if (typeof feature_CarriesTheCard !== 'undefined') {
    const F = feature_NoFlash;

    // the parent ends a carry by cancelling the animation, taking its class
    // off and clearing the inline transform — all in one turn. Measured in
    // WebKit, that is one frame in which the promotion, the animation and the
    // transform all go at once, and the media element is re-rasterised: the
    // only discontinuity in the whole gesture. The hint is put on first and
    // taken off later, so the layer survives the hand-off.
    const fm_nfClear = feature_CarriesTheCard.clear;
    feature_CarriesTheCard.clear = function (page, cls, a) {
      try { F.hold(page); } catch (e) { /* the parent's own clear stands */ }
      return fm_nfClear.call(this, page, cls, a);
    };

    // and the release itself, in the record
    const fm_nfEnd = feature_CarriesTheCard.end;
    feature_CarriesTheCard.end = function () {
      const at = this.at;
      const page = at && at.on ? this.page() : null;
      if (page) { try { F.note(page, 'release'); } catch (e) { /* nothing to say */ } }
      return fm_nfEnd.call(this);
    };
  }
}
