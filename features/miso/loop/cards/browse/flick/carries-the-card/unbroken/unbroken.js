// the two cards are on the screen at once: the one you are leaving keeps
// moving with your finger while the one you are going to comes up behind it,
// so the sweep is one strip and never an empty screen.
const feature_Unbroken = {
  GAP: 14,          // between the two cards in the strip
  SETTLE: 190,      // the strip coming to rest, or springing back
  REACH: 60,        // /flick's own threshold, asked while the finger is down
  SIDE: 40,         // and its sideways limit
  SPAN: 600,        // and its clock

  ghost: null,      // the card being left, outside #app
  dir: '',          // the way the strip is going
  step: 0,          // the offset between the two cards
  live: false,      // a strip is under the finger
  ids: [],          // the surface's own list, for whether there IS a neighbour

  remember() {
    const d = document.getElementById('mapData');
    if (!d) return;
    const raw = d.getAttribute('data-ids');
    if (raw === null) return;
    this.ids = raw.split(',').filter((x) => x);
  },

  // is there a post that way? At the ends there is not, and /rubber-band's
  // pull is the answer there — this node must not start a strip with nothing
  // to put in it.
  has(dir) {
    const page = document.querySelector('.card-page');
    if (!page || !this.ids.length) return false;
    const at = this.ids.indexOf(page.getAttribute('data-card') || '');
    if (at < 0) return false;
    return dir === 'next' ? at < this.ids.length - 1 : at > 0;
  },

  // /flick's rule, asked mid-drag instead of at the release: far enough, not
  // sideways, quick enough, and at the end of the card the sweep needs.
  crossing(a, x, y) {
    const d = a.d;
    if (Date.now() - d.t > this.SPAN) return '';
    const dy = y - d.y, dx = x - d.x;
    if (Math.abs(dy) < this.REACH || Math.abs(dx) >= this.SIDE) return '';
    if (dy < 0 && d.atBottom) return 'next';
    if (dy > 0 && d.atTop) return 'prev';
    return '';
  },

  // the switch, sent while the finger is still down. The card being left is
  // moved out of #app first so the paint cannot take it away: it is fixed to
  // the viewport by its own rule, so it does not move by being reparented,
  // and it keeps the transform the finger gave it.
  start(C, dir, dy) {
    const page = document.querySelector('.card-page');
    if (!page) return false;
    this.step = Math.round(page.getBoundingClientRect().height) + this.GAP;
    page.classList.add('fm-strip-ghost');
    document.body.appendChild(page);
    this.ghost = page;
    this.dir = dir;
    this.live = true;
    C.busy = Date.now();                  // the parent's own release stands down
    feature_Loop.send({ type: 'click', ev: 'browse_' + dir });
    this.place(dy);
    return true;
  },

  // both cards from one number: the one being left where the finger is, the
  // one arriving a card and a gap behind it.
  place(dy) {
    const off = this.dir === 'next' ? this.step : -this.step;
    if (this.ghost) this.ghost.style.transform = 'translateY(' + Math.round(dy) + 'px)';
    const now = document.querySelector('#app .card-page');
    if (now) now.style.transform = 'translateY(' + Math.round(dy + off) + 'px)';
  },

  // the finger has gone. Past the threshold the strip completes onto the card
  // that arrived; short of it the switch is undone — a real turn back through
  // the same event, because the card that came in was a real turn too.
  release(C, dy) {
    const off = this.dir === 'next' ? this.step : -this.step;
    const back = Math.abs(dy) < this.REACH;
    if (back) {
      const other = this.dir === 'next' ? 'prev' : 'next';
      feature_Loop.send({ type: 'click', ev: 'browse_' + other });
      this.drop();
      const page = document.querySelector('#app .card-page');
      if (page) this.glide(page, dy, 0, C);
      else this.done(C);
      return;
    }
    const now = document.querySelector('#app .card-page');
    const ghost = this.ghost;
    const to = -off;                       // the arriving card home, the other away
    if (ghost) this.glide(ghost, dy, dy + to, null);
    if (now) this.glide(now, dy + off, 0, C);
    else this.done(C);
  },

  glide(el, from, to, C) {
    const a = el.animate([{ transform: 'translateY(' + Math.round(from) + 'px)' },
                          { transform: 'translateY(' + Math.round(to) + 'px)' }],
                         { duration: this.SETTLE, easing: 'ease-out', fill: 'both' });
    const self = this;
    a.finished.then(() => {
      try { a.cancel(); } catch (e) { /* gone */ }
      el.style.transform = '';
      if (C) self.done(C);
    }).catch(() => { if (C) self.done(C); });
  },

  drop() {
    if (this.ghost && this.ghost.parentNode) this.ghost.parentNode.removeChild(this.ghost);
    this.ghost = null;
  },

  done(C) {
    this.drop();
    this.live = false;
    this.dir = '';
    if (C) C.busy = 0;
  },
};

{
  if (typeof feature_CarriesTheCard !== 'undefined' && typeof feature_Flick !== 'undefined'
      && typeof feature_Loop !== 'undefined' && typeof feature_Map !== 'undefined') {
    const U = feature_Unbroken;

    // the finger, after the parent (and /rubber-band) have had it
    const fm_unMove = feature_CarriesTheCard.move;
    feature_CarriesTheCard.move = function (x, y) {
      const out = fm_unMove.call(this, x, y);
      try {
        const a = this.at;
        if (!a || !a.on) return out;
        const dy = y - a.d.y;
        if (U.live) { U.place(dy); return true; }
        const dir = U.crossing(a, x, y);
        if (dir && U.has(dir)) U.start(this, dir, dy);
      } catch (e) { /* the parent's own sweep, as before */ }
      return out;
    };

    // the release, after the parent's — which stands down while a strip is
    // live because `busy` is set
    const fm_unEnd = feature_CarriesTheCard.end;
    feature_CarriesTheCard.end = function () {
      const a = this.at;
      const dy = a && a.on ? a.dy : 0;
      const strip = U.live;
      fm_unEnd.call(this);
      if (strip) { try { U.release(this, dy); } catch (e) { U.done(this); } }
    };

    // one switch per crossing: /flick's release still fires on the way up and
    // its event has already gone.
    const fm_unGo = feature_Flick.go;
    feature_Flick.go = function (dir) {
      if (U.live) return;
      return fm_unGo.call(this, dir);
    };

    // a new gesture never starts inside a strip
    const fm_unBegin = feature_CarriesTheCard.begin;
    feature_CarriesTheCard.begin = function () {
      if (U.live) { this.at = null; return; }
      return fm_unBegin.call(this);
    };

    const fm_unSync = feature_Map.sync;
    feature_Map.sync = function () {
      fm_unSync.call(this);
      try { U.remember(); } catch (e) { /* the last list stands */ }
    };
    try { U.remember(); } catch (e) { /* nothing drawn yet */ }
  }
}
