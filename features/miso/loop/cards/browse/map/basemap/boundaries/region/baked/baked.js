const feature_Baked = {
  // one ground, not two and a mask. /region drew the Outdoors squares in a
  // pane and cut them with a clip-path rebuilt on zoomend — which is why the
  // lit region stood still through a pinch and jumped at the end. A square
  // with the boundary already in it is one layer, and Leaflet scales every
  // tile layer in the map pane by the same transform at the same moment, so
  // there is nothing left that can fall out of step.
  ROOT: 'tiles/region/',

  // bare, for /region's URL slot: /region adds the ground tag itself when it
  // builds the layer, and adding it here as well would make `...png?g=3?g=3`
  template(code) {
    return this.ROOT + encodeURIComponent(code) + '/{z}/{x}/{y}.png';
  },

  // what the layer's own `_url` will read once /region has built it — the
  // string to compare against and to set
  tagged(code) {
    let url = this.template(code);
    if (typeof feature_FreshTiles !== 'undefined' && feature_FreshTiles.TAG) {
      url += '?' + feature_FreshTiles.TAG;
    }
    return url;
  },

  codeNow(region) {
    const f = region.featureFor(region.chosen());
    return ((f || {}).properties || {}).code || '';
  },

  // what /region's cut() used to be. There is nothing left to clip: a baked
  // square carries its own boundary, and a cut on top could only make the
  // region smaller than it already is. So the clip is cleared rather than
  // recomputed — and cleared on the same beats it used to be set on, so an
  // untick of this node puts it straight back.
  //
  // The pane is shown only once the layer is pointing at the baked route, so
  // there is never a frame in which a raw Outdoors square is on screen with no
  // boundary on it.
  dress(region) {
    if (typeof feature_Map === 'undefined' || !feature_Map.map) return;
    const pane = feature_Map.map.getPane(region.PANE);
    if (!pane) return;
    if (pane.style.clipPath !== 'none') {
      pane.style.clipPath = 'none';
      pane.style.webkitClipPath = 'none';
    }
    const layer = region.layer;
    if (!layer) { pane.style.display = 'none'; return; }
    const want = this.tagged(this.codeNow(region));
    if (layer._url !== want) layer.setUrl(want);
    if (pane.style.display !== '') pane.style.display = '';
  },
};

{
  // /region's own two seams, taken at load and typeof-guarded.
  //
  // ensure() is wrapped rather than replaced: all this node needs from it is
  // that the layer be BUILT, and later RE-POINTED, at the baked route.
  //
  //   - setting URL first means the layer is created pointing at the baked
  //     route, so no burst of raw Outdoors squares is ever asked for and then
  //     thrown away;
  //   - re-pointing an existing layer with setUrl(url, true) — Leaflet's own
  //     "no redraw" argument — matters more. /region answers a change of
  //     region by moving the layer's bounds and calling redraw(); a redraw
  //     between the two writes would ask the server to BAKE the new region
  //     over the old region's box, and a baked square is the expensive kind.
  //     Setting the url silently lets /region's own redraw be the only one,
  //     with the right url and the right box.
  //
  // cut() is replaced outright, because there is nothing of it left to run —
  // it existed only to write the clip-path this node removes. /region calls it
  // at the end of every ensure(), which is every paint and every zoomend,
  // moveend and viewreset, so dress() stands exactly where the cut stood.
  if (typeof feature_Region !== 'undefined') {
    const fm_bEnsure = feature_Region.ensure;
    feature_Region.ensure = function () {
      const code = feature_Baked.codeNow(this);
      if (code) {
        this.URL = feature_Baked.template(code);
        const want = feature_Baked.tagged(code);
        if (this.layer && this.layer._url !== want) this.layer.setUrl(want, true);
      }
      fm_bEnsure.call(this);
    };
    feature_Region.cut = function () {
      feature_Baked.dress(this);
    };
  }
}
