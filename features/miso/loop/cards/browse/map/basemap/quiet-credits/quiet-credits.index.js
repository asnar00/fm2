const feature_QuietCredits = {
  SRC: 'tiles/attribution',
  reading: null,     // the gather, memoised: two opens share one pair of reads

  // ---- the map keeps nothing floating over it ---------------------------
  // Leaflet's attribution control is REMOVED, not hidden: a hidden control
  // still measures and still holds the map's bottom-right corner. The two
  // credit() calls that later add lines to it (/map's and /boundaries') are
  // no-ops on a removed control — Leaflet's _update returns early with no
  // map — so neither file is touched.
  strip() {
    if (typeof feature_Map === 'undefined' || !feature_Map.map) return;
    const c = feature_Map.map.attributionControl;
    if (!c || !c.remove) return;
    try { c.remove(); } catch (e) { }
  },

  // ---- what the credits say ---------------------------------------------
  // the extension point: everything the app must credit, in order. A later
  // node with a source of its own wraps this the way this file wraps the
  // panel's open, and pushes its line onto the result.
  async gather() {
    const out = [];
    const tile = await this.tileLine();
    if (tile) out.push(tile);
    const bound = await this.boundaryLine();
    if (bound) out.push(bound);
    return out;
  },

  // the server's own route — the same one /map's credit() reads, so a change
  // of tile source changes this line too. With /tiles unticked the route is
  // gone and whatever answers instead is not a credit: one short plain line
  // or nothing.
  async tileLine() {
    try {
      const r = await fetch(this.SRC);
      if (!r.ok) return '';
      const t = (await r.text()).trim();
      if (!t || t.length > 300 || t.indexOf('<') >= 0) return '';
      return t;
    } catch (e) { return ''; }
  },

  // the boundaries file's own words. If the map view has been opened the
  // parsed file is already in hand; if it has not, the file is read once for
  // its credit alone — a credit that appears only after you visit a view is
  // not a credit. Our own asset, cached by the service worker.
  async boundaryLine() {
    if (typeof feature_Boundaries === 'undefined') return '';
    if (feature_Boundaries.data) return this.creditOf(feature_Boundaries.data);
    try {
      const d = await fetch(feature_Boundaries.FILE)
        .then((r) => (r.ok ? r.json() : null));
      return d ? this.creditOf(d) : '';
    } catch (e) { return ''; }
  },

  creditOf(d) {
    return String((d && (d.credit || d.attribution)) || '').trim();
  },

  // ---- the section at the bottom of the sheet ---------------------------

  async show() {
    const box = document.getElementById('credits');
    if (!box) return;
    if (!this.reading) this.reading = this.gather();
    const lines = await this.reading;
    if (!lines.length) this.reading = null;   // nothing said: ask again next open
    box.innerHTML = lines.length
      ? '<div class="credit-head">credits</div>'
        + lines.map((l) => '<div class="credit-line">' + this.esc(l) + '</div>').join('')
      : '';
    box.style.display = lines.length ? 'block' : 'none';
  },

  esc(s) {
    return String(s).replace(/&/g, '&amp;').replace(/</g, '&lt;')
      .replace(/>/g, '&gt;');
  },
};

{
  // the section, made at load and appended to the sheet. This node's fragment
  // is composed last, so the append lands below /less-busy's arrangement —
  // the bottom of the popup, which is where the ask put it.
  const fm_qcPanel = document.getElementById('panel');
  if (fm_qcPanel) {
    const fm_qcBox = document.createElement('div');
    fm_qcBox.id = 'credits';
    fm_qcBox.style.display = 'none';
    fm_qcPanel.appendChild(fm_qcBox);
  }

  if (typeof feature_Panel !== 'undefined') {
    const fm_qcOpen = feature_Panel.open.bind(feature_Panel);
    // started BEFORE the original, not after it: the credits depend on
    // nothing the panel's open does, and the open awaits the feature list —
    // which /arrives allows up to 2.5s. Behind it the section arrived late
    // enough for a rig to photograph the sheet without it.
    feature_Panel.open = async function () {
      feature_QuietCredits.show().catch(() => { });
      await fm_qcOpen();
    };
  }

  // /map's mount, taken by property replacement at load — not a timer
  // (notes.md, "the apply-wrapper race"). attributionControl:false is a mount
  // option, and reaching it would mean editing map.js; the control is made
  // and then removed instead.
  if (typeof feature_Map !== 'undefined') {
    const fm_qcMount = feature_Map.mount;
    feature_Map.mount = function () {
      const ok = fm_qcMount.call(this);
      if (ok) {
        try { feature_QuietCredits.strip(); } catch (e) { }
      }
      return ok;
    };
  }
}
