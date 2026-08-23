const feature_Attention = {
  asks: null,        // the asks value this page has already seen
  lit: false,        // is the lozenge carrying the pulse?
  awake: false,      // has the page finished catching up?
  waking: false,     // is the catch-up window already running?
  settle: 1000,      // how long catching up is allowed to take, in ms

  panelOpen() {
    const p = $('panel');
    return !!(p && p.style.display === 'block');
  },

  // the lozenge is the app's one attention surface. Its own class, so that
  // `#build.update` keeps meaning only "a newer build is waiting".
  flash() {
    if (this.lit) return;
    const handle = $('build');
    if (!handle) return;
    handle.classList.add('attention');
    this.lit = true;
  },

  clear() {
    const handle = $('build');
    if (handle) handle.classList.remove('attention');
    this.lit = false;
  },

  asksIn(state) {
    try {
      const v = JSON.parse(state || '{}').asks;
      return typeof v === 'string' ? v : null;
    } catch (e) { return null; }
  },

  // did this device do it? An edit minted here rides out as a CtxOp in the
  // very payload that changed the value; an arriving CtxUpdate is applied by
  // assignment and mints nothing. That is the whole difference between "I did
  // this" and "this happened to me", and it is read from the payload rather
  // than from feature_Loop.state because /messaging empties the outbox out of
  // the state as soon as it has it.
  ownEdit(p) {
    try {
      const s = JSON.parse((p && p.state) || '{}');
      const out = s._send;
      if (!Array.isArray(out)) return false;
      return out.some((m) => m && m.type === 'CtxOp'
        && m.data && m.data.name === 'asks');
    } catch (e) { return false; }
  },

  // a page is not awake the moment it loads: it joins, and then the long poll
  // hands it whatever it missed while it was gone. Those arrivals are the page
  // catching up, not the world changing, so they only set the baseline — a
  // page that opens on news is not being interrupted by it. The clock starts
  // at the join rather than at script load, so a slow wasm fetch does not eat
  // the window, and a page that never joins never wakes: it has no live
  // connection to be interrupted by.
  wake(p) {
    if (this.awake || this.waking) return;
    let joined = false;
    try { joined = !!JSON.parse((p && p.state) || '{}')._joined; } catch (e) {}
    if (!joined) return;
    this.waking = true;
    setTimeout(() => { this.awake = true; }, this.settle);
  },

  // one applied payload, judged.
  saw(p) {
    const next = this.asksIn(p && p.state);
    const seen = this.asks;
    this.asks = next;
    this.wake(p);
    if (!this.awake) return;
    if (seen === null || next === null || next === seen) return;
    if (this.ownEdit(p)) return;
    if (this.panelOpen()) return;   // it updated in place; nothing to flag
    this.flash();
  },
};
{
  const fm_attnApply = feature_Loop.apply;
  feature_Loop.apply = function (payloadJson) {
    let p = null;
    try { p = JSON.parse(payloadJson); } catch (e) {}
    fm_attnApply.call(this, payloadJson);
    feature_Attention.saw(p);
  };
  // opening the panel is reading the news
  if (typeof feature_Panel !== 'undefined' && feature_Panel.open) {
    const fm_attnOpen = feature_Panel.open.bind(feature_Panel);
    feature_Panel.open = async function () {
      await fm_attnOpen();
      feature_Attention.clear();
    };
  }
  // the page half of the service worker's fork: a push that found a visible
  // window is handed here instead of ringing, and becomes the same flash.
  if (navigator.serviceWorker && navigator.serviceWorker.addEventListener) {
    navigator.serviceWorker.addEventListener('message', (e) => {
      if (e && e.data && e.data.fm === 'attention'
          && !feature_Attention.panelOpen()) feature_Attention.flash();
    });
  }
}
