const feature_Scroll = {
  // how many places are remembered at once. A session that opens hundreds of
  // cards must not grow a record without end; the oldest recording goes first,
  // and the page on the screen is re-recorded before every paint, so it is
  // always the newest entry and can only be evicted after CAP other pages have
  // been scrolled while it is away.
  CAP: 50,
  // how long a freshly painted page is given to grow to its full height before
  // the restore gives up. Pictures decode after the paint, so the first
  // scrollTop write can be clamped by a page that is still short.
  SETTLE: 700,
  // how long a stash may wait for the reload it was written for. An update's
  // reload takes seconds; anything older belongs to a session that ended.
  FRESH: 120000,
  STASH: 'misoScroll',

  at: new Map(),        // key -> scrollTop, most recently recorded last
  gen: 0,               // the paint a settle belongs to; a newer paint abandons it
  waiting: new Set(),   // places whose restore has not landed yet
  userMoved: false,

  // ---- what scrolls, and the name it answers to --------------------------

  // The card page is named by the card it shows, so a different card is a
  // different place and starts at the top. A .card-page with no data-card —
  // the waiting card, the invite page, a deleted post — has no identity and
  // is never recorded. The two browse views are named by the view, which is
  // what makes "back to the list" land where the list was.
  keyOf(el) {
    const c = el.classList;
    if (c.contains('card-page')) {
      const id = el.getAttribute('data-card');
      return id ? 'card:' + id : null;
    }
    if (c.contains('browse-list')) return 'list:browse';
    if (c.contains('browse-grid')) return 'grid:browse';
    return null;
  },

  each() {
    return Array.from(
      document.querySelectorAll('.card-page, .browse-list, .browse-grid'));
  },

  // a place is remembered only once it has been scrolled: an untouched page
  // would otherwise spend a slot in the record saying "at the top", which is
  // the default anyway.
  remember(key, top) {
    if (!this.at.has(key) && !top) return;
    this.at.delete(key);
    this.at.set(key, top);
    while (this.at.size > this.CAP) this.at.delete(this.at.keys().next().value);
  },

  // ---- the two moments around a paint ------------------------------------

  // before the innerHTML goes in: where everything on the old screen was.
  //
  // Two things are not a reader's choice and must never be written down. An
  // element with nothing to scroll reports 0 because it is short, not because
  // anyone went to the top. And an element whose restore has not landed yet
  // (its page is still growing) reports the clamp of what we asked for — a
  // repaint in that window would otherwise record the clamp over the real
  // place and lose it for good.
  record() {
    for (const el of this.each()) {
      const k = this.keyOf(el);
      if (!k) continue;
      if (this.waiting.has(k)) continue;
      if (el.scrollHeight - el.clientHeight <= 0) continue;
      this.remember(k, el.scrollTop);
    }
  },

  // after it: put each place back on the element of the same name. An element
  // with no remembered place is left where the browser put it — the top.
  restore() {
    const g = ++this.gen;
    this.userMoved = false;
    const pending = [];
    const waiting = new Set();
    for (const el of this.each()) {
      const k = this.keyOf(el);
      if (!k) continue;
      const want = this.at.get(k);
      if (!want) continue;
      el.scrollTop = want;
      if (el.scrollTop < want - 1) {
        pending.push([el, want, k]);
        waiting.add(k);
      }
    }
    this.waiting = waiting;
    if (pending.length) this.settle(g, pending);
  },

  // the page was too short to hold the position — its pictures have not
  // decoded yet. Try again each frame until it fits, the deadline passes, a
  // newer paint arrives, or the reader touches the screen: a restore must
  // never fight a finger.
  settle(g, pending) {
    const t0 = Date.now();
    const give = (list) => { for (const p of list) this.waiting.delete(p[2]); };
    const step = () => {
      if (g !== this.gen) return;   // a newer paint owns the screen and the set
      if (this.userMoved) { give(pending); return; }
      const left = [];
      for (const pair of pending) {
        const el = pair[0], want = pair[1];
        if (!el.isConnected) { this.waiting.delete(pair[2]); continue; }
        if (el.scrollTop < want - 1) el.scrollTop = want;
        if (el.scrollTop < want - 1) left.push(pair);
        else this.waiting.delete(pair[2]);
      }
      if (!left.length) return;
      if (Date.now() - t0 > this.SETTLE) { give(left); return; }
      pending = left;
      requestAnimationFrame(step);
    };
    requestAnimationFrame(step);
  },

  // ---- across the reload an update makes ---------------------------------

  // written the moment before /review stamps the version and reloads, in the
  // shape /seamless uses for the state: the build it belongs to, and the
  // record. Its own key rather than a field inside misoStash, because
  // /seamless writes that key whole from inside its own wrapper and a newer
  // node cannot add to it without editing /seamless.
  stash(build) {
    this.record();
    const at = {};
    for (const pair of this.at) if (pair[1]) at[pair[0]] = pair[1];
    try {
      localStorage[this.STASH] = JSON.stringify({ v: build, t: Date.now(), at });
    } catch (e) {}
  },

  // read once at load, matching or not, and only when the build that wrote it
  // is the build now running — /seamless' own test. The places go into the
  // ordinary record, so the resumed page is restored by the ordinary paint
  // path whichever paint after boot happens to draw it.
  resume() {
    let s = null;
    try { s = JSON.parse(localStorage[this.STASH] || 'null'); } catch (e) {}
    try { delete localStorage[this.STASH]; } catch (e) {}
    if (!s || String(s.v) !== String(localStorage.misoVersion)) return;
    // and only if the reload it was written for is the one that just happened.
    // /patch takes some updates in place and still stamps misoVersion (through
    // /delta's quiet), so the build alone would let a stash written days ago
    // reach a boot that has nothing to do with it.
    if (!(Date.now() - (Number(s.t) || 0) < this.FRESH)) return;
    for (const k of Object.keys(s.at || {})) {
      const v = Number(s.at[k]) || 0;
      if (v) this.remember(k, v);
    }
  },
};

{
  // /loop's paint seam, taken by property replacement at load — /keep's own
  // idiom, and never a timer-installed wrapper (notes.md, "the apply-wrapper
  // race"). This fragment is the newest under the seam, so it loads last and
  // wraps outermost: the scroll goes back AFTER /keep has put the caret and
  // the focus back, and focus() scrolling the block into view cannot win.
  if (typeof feature_Loop !== 'undefined') {
    const fm_scrollPaint = feature_Loop.paint;
    feature_Loop.paint = function (html) {
      try { feature_Scroll.record(); } catch (e) {}
      try {
        fm_scrollPaint.call(this, html);
      } finally {
        try { feature_Scroll.restore(); } catch (e) {}
      }
    };
  }

  // /review's apply is the version stamp, the cache eviction and the reload.
  // Wrapping it here puts the record in front of all three.
  if (typeof feature_Review !== 'undefined') {
    const fm_scrollApply = feature_Review.apply.bind(feature_Review);
    feature_Review.apply = async function (build) {
      feature_Scroll.stash(build);
      return await fm_scrollApply(build);
    };
  }

  // a finger, a wheel or a key during the settle window ends it: the reader
  // has taken over and the remembered place is no longer where they want to
  // be. Capture phase, so no other listener can hide the gesture.
  for (const fm_scrollEv of ['wheel', 'touchstart', 'pointerdown', 'keydown'])
    document.addEventListener(fm_scrollEv, () => {
      feature_Scroll.userMoved = true;
    }, true);

  feature_Scroll.resume();
}
