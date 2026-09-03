{
  if (typeof feature_FanOut !== 'undefined') {
    // every marker on the map with a pin face, whoever drew it
    feature_FanOut.layout = function () {
      if (typeof feature_Map === 'undefined' || !feature_Map.map) return;
      const map = feature_Map.map;
      const groups = [];
      map.eachLayer((m) => {
        if (!m.getLatLng || !m.getElement) return;
        const el = m.getElement();
        const pin = el && el.querySelector ? el.querySelector('.map-pin') : null;
        if (!pin) return;
        const pt = map.latLngToLayerPoint(m.getLatLng());
        let g = null;
        for (const cand of groups) {
          if (pt.distanceTo(cand.at) <= this.NEAR) { g = cand; break; }
        }
        if (!g) { g = { at: pt, pins: [] }; groups.push(g); }
        g.pins.push(pin);
      });
      for (const g of groups) {
        const n = g.pins.length;
        const extra = n > 1 ? Math.max(0, Math.round(n * 5.8) - 33) : 0;
        g.pins.forEach((pin, k) => this.turn(pin, n > 1 ? (360 * k) / n : 0, extra));
      }
    };
    if (typeof feature_Live !== 'undefined' && typeof feature_Live.draw === 'function') {
      const fm_liveFanDraw = feature_Live.draw;
      feature_Live.draw = function (rows) {
        fm_liveFanDraw.call(this, rows);
        try { feature_FanOut.hook(); feature_FanOut.layout(); } catch (e) { /* the pins stand */ }
      };
    }
  }
}
