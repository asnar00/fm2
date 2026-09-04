const feature_Outlined = {
  // the ward line, in ink: black, thin, and the same on either ground. The
  // constituency ring is not this node's business — /light-basemap re-inks it
  // for the ground of the day and that must keep working, so the style is
  // WRAPPED rather than replaced (misses.md, "siblings at one anchor").
  WARD: '#000000',
  WEIGHT: 1.2,      // visible at zoom 11 on a phone; still thinner than the ring

  styleOf(f, prev) {
    const s = prev || {};
    if ((f.properties || {}).kind !== 'ward') return s;
    return Object.assign({}, s, {
      color: this.WARD,
      weight: this.WEIGHT,
      opacity: 1,
    });
  },

  // the names, gone. The layer comes OFF the map rather than being hidden:
  // twenty-six divIcons behind a display:none pane are still twenty-six
  // markers Leaflet repositions on every pan. Idempotent, and cheap — this
  // stands where /boundaries' declutter stood, so it runs on every moveend.
  hush() {
    if (typeof feature_Map === 'undefined' || !feature_Map.map) return;
    const map = feature_Map.map;
    const b = feature_Boundaries;
    if (b.labels && map.hasLayer(b.labels)) map.removeLayer(b.labels);
    const pane = map.getPane(b.PANE);
    if (pane) pane.style.display = 'none';
  },
};

{
  // property replacement at load, /boundaries' own idiom on /map's sync. With
  // /boundaries unticked there is nothing to ink and nothing to quieten.
  if (typeof feature_Boundaries !== 'undefined') {
    const fm_oStyle = feature_Boundaries.styleOf;
    feature_Boundaries.styleOf = function (f) {
      return feature_Outlined.styleOf(f, fm_oStyle.call(this, f));
    };
    // place() is the ONLY caller of anything to do with the names — draw()
    // once, then every zoomend and moveend. Replacing it puts the removal on
    // exactly those beats, so a label layer re-added by any path is off again
    // by the next gesture.
    feature_Boundaries.place = function () {
      try {
        feature_Outlined.hush();
      } catch (e) {
      }
    };
  }
}
