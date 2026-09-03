{
  if (typeof feature_Reel !== 'undefined') {
    feature_Reel.make();
    // the mark moves with the scroll; the pan waits only 60 ms
    feature_Reel.list.addEventListener('scroll', () => {
      clearTimeout(feature_Reel.settle);
      feature_Reel.settle = setTimeout(() => feature_Reel.follow(), 60);
      if (typeof feature_Reel.mark === 'function') {
        try { feature_Reel.mark(); } catch (e) { /* no list yet */ }
      }
    }, { passive: true });
    const fm_quickPan = feature_Reel.pan;
    feature_Reel.pan = function (lat, lon) {
      if (typeof feature_Map === 'undefined' || !feature_Map.map) return;
      const map = feature_Map.map;
      const real = map.panTo.bind(map);
      map.panTo = (ll, o) => real(ll, Object.assign({}, o || {}, { duration: 0.3 }));
      try { fm_quickPan.call(this, lat, lon); } finally { map.panTo = real; }
    };
  }
}
