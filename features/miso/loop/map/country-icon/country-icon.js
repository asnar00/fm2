// which country is this fix in, and what does it look like? The outlines
// are ours (vendored by tools/fetch_countries.py), so this needs no network
// and no one else's geocoder.
const feature_CountryIcon = {
  data: null,
  loading: false,

  async load() {
    if (this.data || this.loading) return;
    this.loading = true;
    try {
      const r = await fetch('geo/countries.json');
      if (r.ok) this.data = await r.json();
    } catch (e) { /* absent outlines: the emoji stays, nothing breaks */ }
    this.loading = false;
    this.paint();
  },

  // ray casting: odd crossings means inside
  inRing(x, y, ring) {
    let inside = false;
    for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
      const xi = ring[i][0], yi = ring[i][1];
      const xj = ring[j][0], yj = ring[j][1];
      if ((yi > y) !== (yj > y)
          && x < ((xj - xi) * (y - yi)) / (yj - yi || 1e-12) + xi) {
        inside = !inside;
      }
    }
    return inside;
  },

  find(lon, lat) {
    if (!this.data) return null;
    for (const code of Object.keys(this.data)) {
      const c = this.data[code];
      const b = c.b;
      if (lon < b[0] || lon > b[2] || lat < b[1] || lat > b[3]) continue;
      for (const ring of c.r) {
        if (this.inRing(lon, lat, ring)) return code;
      }
    }
    return null;
  },

  // north is up, so latitude flips against SVG's downward y
  svg(code) {
    const c = this.data && this.data[code];
    if (!c) return '';
    const x0 = c.b[0], y0 = c.b[1], x1 = c.b[2], y1 = c.b[3];
    const w = (x1 - x0) || 1, h = (y1 - y0) || 1;
    const d = c.r.map((ring) => 'M' + ring.map(
      (p) => (p[0] - x0).toFixed(2) + ',' + (y1 - p[1]).toFixed(2)).join('L') + 'Z').join('');
    return '<svg viewBox="0 0 ' + w.toFixed(2) + ' ' + h.toFixed(2) + '" '
      + 'preserveAspectRatio="xMidYMid meet"><path d="' + d + '" '
      + 'fill="currentColor"/></svg>';
  },

  // renders are whole-DOM swaps, so the outline is repainted after each one
  paint() {
    for (const el of document.querySelectorAll('.cc[data-cc]')) {
      const code = el.getAttribute('data-cc');
      if (el.dataset.drawn === code) continue;
      const markup = this.svg(code);
      if (!markup) { this.load(); continue; }
      el.innerHTML = markup;
      el.dataset.drawn = code;
    }
  },

  watch() {
    if (typeof feature_Replay !== 'undefined' && feature_Replay.active) return;
    let s = {};
    try { s = JSON.parse(feature_Loop.state || '{}'); } catch (e) { return; }
    const fix = s.map_fix;
    if (fix && typeof fix.lat === 'number') {
      if (!this.data) { this.load(); return; }
      const code = this.find(fix.lon, fix.lat);
      if (code && code !== s.map_country) {
        try { localStorage.misoCountry = code; } catch (e) {}
        feature_Loop.send({ type: 'CountryFound', data: { code } });
        return;
      }
    }
    this.paint();
  },

  init() {
    this.load();
    const fm_ccApply = feature_Loop.apply;
    const self = this;
    feature_Loop.apply = function (p) {
      fm_ccApply.call(this, p);
      self.watch();
    };
    // the country you were in is still the country you are in, most
    // mornings: the icon survives a restart without asking for a fix
    try {
      const remembered = localStorage.misoCountry;
      if (remembered) {
        feature_Loop.send({ type: 'CountryFound', data: { code: remembered } });
      }
    } catch (e) {}
    this.watch();
  },
};
const fm_ccInit = setInterval(() => {
  if (typeof feature_Loop !== 'undefined' && feature_Loop.state !== null) {
    clearInterval(fm_ccInit);
    feature_CountryIcon.init();
  }
}, 100);
