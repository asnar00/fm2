const feature_LinesToo = {
  // the boundary lines are in the squares now, so there is one tile layer on
  // the map and nothing else: no second ground in a pane of its own, and no
  // vector paths in the overlay. Everything the map draws is scaled by the one
  // transform Leaflet puts on that layer's level container, which is what
  // /baked bought and this node extends to the lines.

  // /map's own ground layer, re-pointed at the baked route. It covers the
  // whole view rather than the region's box, because a boundary line crosses
  // squares the region never touches — outside the region a baked square is
  // the ground with the lines on it, and that is the ground now.
  point(region) {
    if (typeof feature_Map === 'undefined') return false;
    if (!feature_Map.map || !feature_Map.layer) return false;
    if (typeof feature_Baked === 'undefined') return false;
    const code = feature_Baked.codeNow(region);
    if (!code) return false;
    const want = feature_Baked.tagged(code);
    if (feature_Map.layer._url !== want) feature_Map.layer.setUrl(want);
    return true;
  },

  // the vector overlay goes. Removed from the map rather than styled away:
  // the ask is that the lines are in the tiles, and a path left in the overlay
  // pane would be a second copy of every boundary drawn over the first — the
  // readout proof is that `.leaflet-overlay-pane path` holds none of them.
  hide() {
    if (typeof feature_Boundaries === 'undefined') return;
    if (typeof feature_Map === 'undefined' || !feature_Map.map) return;
    const map = feature_Map.map;
    const lines = feature_Boundaries.lines;
    if (lines && map.hasLayer(lines)) map.removeLayer(lines);
  },

  // what /stocked should be filling the phone's cache with. Without this the
  // pre-load would go on stocking plain ground squares while the map asks for
  // baked ones, and a canvasser in a stairwell would get a blank map — the
  // one thing /stocked exists to prevent.
  stockUrl(t) {
    if (typeof feature_Baked === 'undefined') return '';
    if (typeof feature_Region === 'undefined') return '';
    const code = feature_Baked.codeNow(feature_Region);
    if (!code) return '';
    return feature_Baked.tagged(code)
      .replace('{z}', t[0]).replace('{x}', t[1]).replace('{y}', t[2]);
  },
};

{
  // /boundaries' place() again, wrapped rather than replaced so /outlined's
  // removal of the names still runs. It is called at draw and on every zoomend
  // and moveend, so a layer re-added by any path is off again by the next
  // gesture — the same beat /outlined chose, for the same reason.
  if (typeof feature_Boundaries !== 'undefined') {
    const fm_ltPlace = feature_Boundaries.place;
    feature_Boundaries.place = function () {
      fm_ltPlace.call(this);
      try {
        feature_LinesToo.hide();
      } catch (e) {
      }
    };
  }
}

{
  // /region's ensure(), replaced outright. There is nothing of it left to do:
  // its whole job was to build and cut a second tile layer in a pane of its
  // own, and there is no second layer any more. off() takes that layer and
  // pane away if an earlier paint made them, and the ground layer is
  // re-pointed instead. /baked's own wrapper and its dress() go with it, and
  // both come back the moment this node is unticked.
  //
  // /region's map handlers were registered inside the block this replaces, so
  // they never attach; they are not needed. One tile layer needs no help to
  // follow a pan or a zoom, which is the entire point.
  if (typeof feature_Region !== 'undefined') {
    feature_Region.ensure = function () {
      this.off();
      if (!feature_LinesToo.point(this)) this.load();
    };
  }
}

{
  // /stocked's url seam, so the pre-load stocks what the map will ask for.
  // Guarded both ways: with /stocked unticked there is nothing to point, and
  // if this node cannot name a square (no region resolved yet) /stocked's own
  // url stands.
  if (typeof feature_Stocked !== 'undefined' && feature_Stocked.url) {
    const fm_ltUrl = feature_Stocked.url;
    feature_Stocked.url = function (t) {
      let u = '';
      try {
        u = feature_LinesToo.stockUrl(t);
      } catch (e) {
        u = '';
      }
      return u || fm_ltUrl.call(this, t);
    };
  }
}
