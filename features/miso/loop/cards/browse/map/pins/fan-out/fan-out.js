const feature_FanOut = {
  NEAR: 30,        // px: within this of a group's first pin, a pin joins the group
  hooked: false,

  // every /map marker's screen point, grouped greedily, then the angles.
  layout() {
    if (typeof feature_Map === 'undefined' || !feature_Map.map) return;
    const map = feature_Map.map;
    const groups = [];
    for (const m of feature_Map.markers || []) {
      const el = m.getElement && m.getElement();
      const pin = el && el.querySelector ? el.querySelector('.map-pin') : null;
      if (!pin || !m.getLatLng) continue;
      const pt = map.latLngToLayerPoint(m.getLatLng());
      let g = null;
      for (const cand of groups) {
        if (pt.distanceTo(cand.at) <= this.NEAR) { g = cand; break; }
      }
      if (!g) { g = { at: pt, pins: [] }; groups.push(g); }
      g.pins.push(pin);
    }
    for (const g of groups) {
      const n = g.pins.length;
      // the ring must have room for n faces: past six the stem grows so the
      // faces sit further out (34 px faces, side by side on the circle)
      const extra = n > 1 ? Math.max(0, Math.round(n * 5.8) - 33) : 0;
      g.pins.forEach((pin, k) => this.turn(pin, n > 1 ? (360 * k) / n : 0, extra));
    }
  },

  // the whole pin about its tip, the face back the other way. A longer stem
  // makes the pin taller below its anchor, so it is lifted by the same
  // amount before the turn: the tip stays on the place.
  turn(pin, deg, extra) {
    const face = pin.querySelector('.map-pin-face');
    const stem = pin.querySelector('.map-pin-stem');
    if (!deg) {
      pin.style.transform = '';
      pin.style.height = '';
      if (face) face.style.transform = '';
      if (stem) stem.style.borderTopWidth = '';
      pin.classList.remove('fm-fanned');
      return;
    }
    extra = extra || 0;
    pin.style.height = (50 + extra) + 'px';
    if (stem) stem.style.borderTopWidth = (12 + extra) + 'px';
    pin.style.transform = (extra ? 'translateY(-' + extra + 'px) ' : '') + 'rotate(' + deg + 'deg)';
    if (face) face.style.transform = 'rotate(' + (-deg) + 'deg)';
    pin.classList.add('fm-fanned');
  },

  hook() {
    if (this.hooked || typeof feature_Map === 'undefined' || !feature_Map.map) return;
    this.hooked = true;
    feature_Map.map.on('zoomend', () => feature_FanOut.layout());
  },
};

{
  // /map's draw, taken by replacing the property at load — /square-posts' and
  // /boundaries' idiom. The layout runs after every draw, including one the
  // sig short-circuit skipped: the zoom may have moved in between.
  if (typeof feature_Map !== 'undefined') {
    const fm_fanDraw = feature_Map.draw;
    feature_Map.draw = function (pins) {
      fm_fanDraw.call(this, pins);
      try {
        feature_FanOut.hook();
        feature_FanOut.layout();
      } catch (e) {
        // a fan that throws must never cost the pins that were drawn
      }
    };
  }
}
