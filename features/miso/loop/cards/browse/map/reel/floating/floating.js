// the pan aims a half-band higher, so the place lands in the clear above
// the floating lozenges rather than under them. /reel's pan seam, and only
// that: follow and whatever wraps it (/current's mark) are left alone.
{
  if (typeof feature_Reel !== 'undefined') {
    feature_Reel.pan = function (lat, lon) {
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
