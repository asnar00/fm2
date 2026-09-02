const feature_FreshTiles = {
  // the ground tag: a new basemap generation gets a new value, so no cache —
  // service worker or browser — can answer with squares of the old ground.
  TAG: 'g=3', // 2: Stadia Alidade Smooth, 3: Alidade Smooth Dark (2026-09-02, self-check #p41, #p43),
  done: false,

  stamp() {
    if (this.done) return;
    if (typeof L === 'undefined') return;
    if (typeof feature_Map === 'undefined' || !feature_Map.map) return;
    let touched = false;
    feature_Map.map.eachLayer((l) => {
      if (l instanceof L.TileLayer && l._url && l._url.indexOf('g=') < 0) {
        l.setUrl(l._url + (l._url.indexOf('?') < 0 ? '?' : '&') + this.TAG);
        touched = true;
      }
    });
    if (touched) this.done = true;
  },
};

{
  // one more wrapper on /map's sync, the idiom /boundaries set; with /map
  // unticked there is no map and this does nothing.
  if (typeof feature_Map !== 'undefined') {
    const fm_ftSync = feature_Map.sync;
    feature_Map.sync = function () {
      fm_ftSync.call(this);
      try { feature_FreshTiles.stamp(); } catch (e) {}
    };
  }
}
