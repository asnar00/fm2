// fm:fragment-gate — the linker's hook: this token asks for every OTHER
// node's index fragments to be wrapped in their gates (the watch/gate blocks
// around each fragment, `data-fm-node` on body roots and stylesheet links).
// Untick this node and none of it is emitted: fragments act regardless, which
// is what they did before this node existed.
const feature_Obey = {
  // this node's own address, for the one question it asks about itself
  path: 'miso/loop/context/changing/enabled/obey',
  raw: null,
  map: {},
  // answers for this frozen view, thrown away when the view changes. A gated
  // function pays a hash lookup rather than a walk, however many links hang on
  // one paint.
  cache: Object.create(null),

  // effective enablement, exactly as the Rust gate computes it: this node's
  // own answer AND every ancestor's. The map carries own answers (already
  // resolved through the shared layer by the server), so the ancestor half is
  // this prefix walk — the same walk the chooser does to shade a row.
  on(path) {
    const known = this.cache[path];
    if (known !== undefined) return known;
    const parts = path.split('/');
    let answer = true;
    for (let i = 1; i <= parts.length; i++) {
      if (this.map[parts.slice(0, i).join('/')] === false) { answer = false; break; }
    }
    this.cache[path] = answer;
    return answer;
  },

  // the paint's frozen view. The payload being applied carries the map the
  // server published from ITS re-frozen world, so reading it here — before any
  // gated fragment runs — gives every fragment in this paint one truth, and
  // the truth of the state they are painting rather than the one before it.
  // Answers whether anything changed: an unchanged map needs no work at all.
  freeze(payload) {
    let raw = '{}';
    try {
      raw = JSON.parse(JSON.parse(payload).state || '{}').feature_ticks || '{}';
    } catch (e) { raw = '{}'; }
    if (raw === this.raw) return false;
    this.raw = raw;
    try { this.map = JSON.parse(raw) || {}; } catch (e) { this.map = {}; }
    this.cache = Object.create(null);
    // this node is not exempt from itself. Unticked, it stops holding anyone
    // to the map — and because the freeze still runs, re-ticking is seen.
    if (!this.on(this.path)) {
      this.map = {};
      this.cache = Object.create(null);
    }
    return true;
  },

  // the seam the object half hangs from (see /absent); nothing by default.
  extra() {},

  // what a runtime can do about furniture it cannot delete: a marked element
  // is hidden, a marked stylesheet is switched off. Both are reversible, and
  // both leave the element's own styling alone.
  paint() {
    for (const el of document.querySelectorAll('[data-fm-node]')) {
      const off = !this.on(el.getAttribute('data-fm-node'));
      if (el.tagName === 'LINK') el.disabled = off;
      else if (off) el.setAttribute('data-fm-off', '');
      else el.removeAttribute('data-fm-off');
    }
  },
};
// the read every generated gate block does, late-bound: while this is missing
// the page is still loading, and load time is when everything is on.
self.fmOn = (path) => feature_Obey.on(path);
if (typeof feature_Loop !== 'undefined') {
  // composed last, so this is the OUTERMOST link of the paint: the freeze runs
  // before any gated fragment, and the furniture is settled after the last one.
  const fm_obeyApply = feature_Loop.apply;
  feature_Loop.apply = function (p) {
    const changed = feature_Obey.freeze(p);
    if (changed) feature_Obey.extra();
    fm_obeyApply.call(this, p);
    if (changed) feature_Obey.paint();
  };
}
