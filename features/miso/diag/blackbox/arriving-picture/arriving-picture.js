// what a card's picture is at the moment it reaches the screen, and what it is
// one frame later. The flash is a phone-only symptom that three rig hypotheses
// have failed to reproduce; this stops guessing and asks the device.
const feature_ArrivingPicture = {
  cause: '',       // the event whose turn is being painted
  last: 0,         // when the previous card paint was

  kind(src) {
    if (!src) return 'none';
    if (src.indexOf('blob:') === 0) return 'blob:';
    if (src.indexOf('data:') === 0) return 'data:';
    if (src.indexOf('pic/') === 0) return 'pic/';
    return 'other';
  },

  // the card standing in #app — the ghost /unbroken keeps outside it is not
  // the arriving one and is reported separately
  look() {
    const p = document.querySelector('#app .card-page');
    if (!p) return null;
    const im = p.querySelector('.poster-frame img, .card-pic img, img');
    const v = p.querySelector('video');
    return {
      card: (p.getAttribute('data-card') || '').slice(-6),
      img: im,
      raw: im ? String(im.getAttribute('src') || '') : '',
      complete: im ? !!im.complete : null,
      w: im ? im.naturalWidth : null,
      away: im ? (im.getAttribute('data-away') || null) : null,
      video: !!v,
      ready: v ? v.readyState : null,
    };
  },

  // one line at the insertion and one at the next frame. The second is what a
  // rig can never show: a src swapped after the element is in the DOM is a
  // blank frame and then a picture, which is what flashing looks like.
  after() {
    const now = Date.now();
    const since = this.last ? now - this.last : 0;
    this.last = now;
    const s = this.look();
    if (!s) return;
    feature_Blackbox.record({
      type: 'media', card: s.card, src: this.kind(s.raw),
      complete: s.complete, w: s.w, away: s.away,
      video: s.video, ready: s.ready,
      cause: this.cause, since: since,
      ghost: typeof feature_Unbroken !== 'undefined' && !!feature_Unbroken.ghost,
    }, null);
    const im = s.img;
    if (!im) return;
    const was = s.raw;
    const self = this;
    requestAnimationFrame(() => {
      try {
        const src = String(im.getAttribute('src') || '');
        feature_Blackbox.record({
          type: 'media2', card: s.card,
          still: !!im.isConnected, changed: src !== was,
          src: self.kind(src), complete: !!im.complete, w: im.naturalWidth,
        }, null);
      } catch (e) { /* the frame is gone; nothing to say about it */ }
    });
  },
};

{
  if (typeof feature_Loop !== 'undefined' && typeof feature_Blackbox !== 'undefined') {
    // the event of the turn, so a line can say which paint it was. Set before
    // the send and cleared after, so a paint that comes from anywhere else —
    // the boot payload — says nothing rather than the last event's name.
    const fm_apSend = feature_Loop.send;
    feature_Loop.send = function (event) {
      feature_ArrivingPicture.cause = (event && (event.ev || event.type)) || '';
      try {
        return fm_apSend.call(this, event);
      } finally {
        feature_ArrivingPicture.cause = '';
      }
    };

    // after the paint, so what is read is what was inserted
    const fm_apPaint = feature_Loop.paint;
    feature_Loop.paint = function (html) {
      const out = fm_apPaint.call(this, html);
      try { feature_ArrivingPicture.after(); } catch (e) { /* never at the cost of a paint */ }
      return out;
    };
  }
}
