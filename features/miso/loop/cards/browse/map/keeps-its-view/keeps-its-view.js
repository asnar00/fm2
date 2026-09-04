// the map opens where you left it. /map makes Leaflet at the world — `setView`
// at zoom 3 — and relies on the first `draw` to fit the pins, so a map made on
// a screen with no set showing has nothing to fit and stays at the world. That
// is what ash saw after the update to 685: centre 51.2719,0.1904 at zoom 15
// before, zoom 0 after (measured on the rig, #p119).
//
// So the view is remembered on the device and put back whenever a map is made,
// and the first fit after that is not allowed to take it away again.
const feature_KeepsItsView = {
  restoreOnce: false,   // a view was applied at mount; the next fit must not undo it

  // "<lat>,<lon>,<zoom>" off the bridged var. One turn stale at worst, which
  // for "where was I looking" is the same answer.
  remembered() {
    try {
      const s = JSON.parse(feature_Loop.state || '{}');
      const v = String(s.map_view || '').split(',');
      if (v.length !== 3) return null;
      const lat = parseFloat(v[0]), lon = parseFloat(v[1]), z = parseInt(v[2], 10);
      if (!isFinite(lat) || !isFinite(lon) || !isFinite(z)) return null;
      return { lat: lat, lon: lon, z: z };
    } catch (e) {
      return null;
    }
  },

  said: '',
  say(map) {
    if (!map) return;
    try {
      const c = map.getCenter();
      const v = c.lat.toFixed(5) + ',' + c.lng.toFixed(5) + ',' + map.getZoom();
      if (v === this.said) return;
      this.said = v;
      feature_Loop.send({ type: 'MapView', data: { v: v } });
    } catch (e) { /* mid-mount: the next moveend says it */ }
  },

  // every road that moves the map ends in moveend or zoomend — a drag, a
  // pinch, /recentre's setView, /floating's pan — so one pair of handlers
  // records them all, and /recentre needs nothing from this node.
  watch(map) {
    if (!map || map.fm_kvWatched) return;
    map.fm_kvWatched = true;
    const self = this;
    map.on('moveend', () => self.say(map));
    map.on('zoomend', () => self.say(map));
  },
};

{
  if (typeof feature_Map !== 'undefined') {
    // a map made at any time opens at the remembered view — never the world.
    const fm_kvMount = feature_Map.mount;
    feature_Map.mount = function () {
      const made = fm_kvMount.call(this);
      if (!made || !this.map) return made;
      try {
        feature_KeepsItsView.watch(this.map);
        const v = feature_KeepsItsView.remembered();
        if (v) {
          this.map.setView([v.lat, v.lon], v.z, { animate: false });
          feature_KeepsItsView.said = v.lat.toFixed(5) + ',' + v.lon.toFixed(5) + ',' + v.z;
          // /map's own flag: with a view of the user's there is nothing for
          // locate() to ask the device about.
          this.fitted = true;
          feature_KeepsItsView.restoreOnce = true;
        }
      } catch (e) { /* the world, as /map left it */ }
      return made;
    };

    // the first draw after a mount fits the pins, which would take the
    // remembered view away again. The pins are still drawn — only the fit is
    // undone, and only that once: after it, /map's own rule (refit only when
    // the set of pins changes) is back, untouched.
    const fm_kvDraw = feature_Map.draw;
    feature_Map.draw = function (pins) {
      if (!feature_KeepsItsView.restoreOnce || !this.map) return fm_kvDraw.call(this, pins);
      const c = this.map.getCenter();
      const z = this.map.getZoom();
      const r = fm_kvDraw.call(this, pins);
      feature_KeepsItsView.restoreOnce = false;
      try { this.map.setView(c, z, { animate: false }); } catch (e) { /* as drawn */ }
      return r;
    };
  }
}
