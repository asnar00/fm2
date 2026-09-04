// closing a post puts it back where it came from: the reel scrolls to the post
// you are actually on, the map goes to that post's pin, and the card shrinks
// down onto the lozenge before the page is put away.
const feature_BackToTheLozenge = {
  MS: 220,
  STUCK: 1200,     // a close that never finished must not swallow the tap
  going: 0,        // when the close began; 0 when nothing is closing
  run: null,       // the shrink in flight
  carry: null,     // it, held across a repaint

  page() {
    if (!document.body.classList.contains('fm-map-behind')) return null;
    return document.querySelector('.card-page');
  },

  quiet() {
    return !!(window.matchMedia
              && window.matchMedia('(prefers-reduced-motion: reduce)').matches);
  },

  running() { return !!this.going && Date.now() - this.going < this.STUCK; },

  // the events that put a card page away: the ‹ and any tool button (the map
  // tap and /swipe-away both send one). The view picker is not among them —
  // it leaves the map view, so there is no lozenge to go back to.
  closes(event) {
    if (!event || event.type !== 'click') return false;
    const ev = event.ev || '';
    return ev === 'tools_home' || ev.indexOf('tool_') === 0;
  },

  // the lozenge of the post that is open NOW — after a sweep has walked the
  // list, that is not the one that was tapped. /opens-over-map hides the reel
  // while a page is up, so it is shown again first: the card is about to land
  // on it, and it is coming back in a moment anyway.
  aim(id) {
    if (typeof feature_Reel === 'undefined' || !feature_Reel.list || !id) return null;
    let el = null;
    for (const p of feature_Reel.list.querySelectorAll('.reel-post')) {
      if ((p.getAttribute('data-ev') || '') === 'browse_open:' + id) { el = p; break; }
    }
    if (!el) return null;
    if (feature_Reel.host) feature_Reel.host.style.display = 'block';
    // the reel scrolls so this lozenge is the one at the left edge, which is
    // what /current calls current and what /on-the-pin rings on the map
    feature_Reel.list.scrollLeft = el.offsetLeft;
    try { feature_Reel.follow(); } catch (e) { /* no map yet: no pan */ }
    const r = el.getBoundingClientRect();
    if (!r.width || !r.height) return null;
    return r;
  },

  // the card down onto it, then the page away
  close(send) {
    const page = this.page();
    if (!page) { send(); return; }
    const id = page.getAttribute('data-card') || '';
    // the reel and the map are put right on every road, motion or no motion —
    // once now, so the card has somewhere to land and the map moves under it,
    // and once after the send, because /reel redraws its band when the set
    // comes back and puts its scroll at the head while doing it
    const r = this.aim(id);
    const after = () => {
      this.going = 0;
      send();
      try { this.aim(id); } catch (e) { /* the band is where /reel left it */ }
    };
    if (!r || !this.shrinks(page) || this.quiet()) { after(); return; }
    const frames = this.frames(page, r);
    if (!frames) { after(); return; }
    this.going = Date.now();
    this.play(page, id, frames, Date.now(), () => {
      after();
      // the send did not put the page away after all (a tool button that
      // opened something else): the card must not be left shrunk on the reel
      const still = document.querySelector('.card-page');
      if (still && (still.getAttribute('data-card') || '') === id) this.clear(still);
    });
  },

  // whether this road has a shrink of its own to run. An /extension point/:
  // /swipe-away has already taken the card off sideways, and that sideways
  // motion is the platform idiom for putting a card away (/learned 5), so a
  // shrink on top of it would haul the card back into view in order to send it
  // somewhere else. A node that owns the whole closing motion says otherwise.
  shrinks(page) {
    return !(page.classList.contains('fm-swipe-left')
          || page.classList.contains('fm-swipe-right'));
  },

  // where the card goes, as two keyframes — the /extension point/ for the
  // shape of the closing. The scale is uniform here, so the card ends the
  // lozenge's width at its own proportions.
  frames(page, r) {
    const c = page.getBoundingClientRect();
    if (!c.width || !c.height) return null;
    const s = Math.min(1, r.width / c.width);
    if (!(s > 0)) return null;
    return [
      { transform: 'none', opacity: 1 },
      { transform: 'translate(' + (r.left - c.left) + 'px, ' + (r.top - c.top)
                 + 'px) scale(' + s + ')', opacity: 0.4 },
    ];
  },

  play(page, id, frames, began, done) {
    page.style.transformOrigin = 'top left';
    page.classList.add('fm-going-back');
    const a = page.animate(frames, { duration: this.MS, easing: 'ease-out', fill: 'both' });
    const at = Math.max(0, Math.min(this.MS, Date.now() - began));
    if (at) { try { a.currentTime = at; } catch (e) { /* from the start, then */ } }
    const run = { id: id, frames: frames, began: began, anim: a, page: page, done: done };
    this.run = run;
    const self = this;
    a.finished.then(() => {
      if (self.run !== run) return;
      self.run = null;
      if (done) done();
    }).catch(() => { /* cancelled: carried onto the element that came back */ });
  },

  // an animation left filling goes on applying its last frame for the life of
  // the element, so a card that survived the close is put back by hand
  clear(page) {
    for (const a of page.getAnimations()) { try { a.cancel(); } catch (e) { /* gone */ } }
    page.classList.remove('fm-going-back');
    page.style.transformOrigin = '';
    page.style.transform = '';
    page.style.opacity = '';
  },
};

{
  if (typeof feature_Loop !== 'undefined') {
    const B = feature_BackToTheLozenge;

    // every road out of a post ends in one of /loop's own sends — the ‹
    // through the delegated click listener, the tap on the map and
    // /swipe-away's flick through their own calls — so one wrapper here
    // catches all three without any of them knowing about it.
    const fm_backSend = feature_Loop.send;
    feature_Loop.send = function (event) {
      const self = this;
      if (B.running() || !B.closes(event) || !B.page()) {
        return fm_backSend.call(self, event);
      }
      B.close(() => fm_backSend.call(self, event));
    };

    // an animation in flight belongs to a page the next paint throws away:
    // its progress is measured by the wall clock and put on the element that
    // comes back, and when nothing comes back the close is let go rather than
    // swallowing the tap for ever.
    const fm_backPaint = feature_Loop.paint;
    feature_Loop.paint = function (html) {
      B.carry = null;
      try {
        if (B.run && Date.now() - B.run.began < B.MS) {
          B.carry = B.run;
          B.run = null;
          B.carry.anim.cancel();
        }
      } catch (e) { B.carry = null; }
      fm_backPaint.call(this, html);
      try {
        const k = B.carry;
        B.carry = null;
        if (!k) return;
        const page = B.page();
        if (!page || (page.getAttribute('data-card') || '') !== k.id) {
          B.going = 0;
          if (k.done) k.done();
          return;
        }
        B.play(page, k.id, k.frames, k.began, k.done);
      } catch (e) { B.going = 0; }
    };
  }
}
