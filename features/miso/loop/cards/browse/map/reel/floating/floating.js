// the pan aims a half-band higher, so the place lands in the clear above
// the floating lozenges rather than under them
{
  if (typeof feature_Reel !== 'undefined') {
    feature_Reel.follow = function () {
      const el = this.current();
      if (!el) return;
      const lat = parseFloat(el.getAttribute('data-lat')), lon = parseFloat(el.getAttribute('data-lon'));
      if (!isFinite(lat) || !isFinite(lon)) return;
      if (typeof feature_Map === 'undefined' || !feature_Map.map) return;
      try {
        const map = feature_Map.map;
        const p = map.project([lat, lon]);
        p.y += this.HEIGHT / 2;
        map.panTo(map.unproject(p), { animate: true, duration: 0.45 });
      } catch (e) { /* mid-mount */ }
    };
  }
}
