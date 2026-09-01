const feature_LightBasemap = {
  // the boundary inks, re-mixed for paper: the same two-tone rule /boundaries
  // drew — dashed constituency a step stronger than the wards — in colours
  // that stand on a light ground instead of vanishing into it.
  styleOf(f) {
    const kind = (f.properties || {}).kind;
    if (kind === 'constituency') {
      return { color: '#4a4a54', weight: 2, opacity: 0.9, fill: false,
               dashArray: '7 5', lineJoin: 'round' };
    }
    return { color: '#84848e', weight: 1, opacity: 0.8, fill: false,
             lineJoin: 'round' };
  },
};

{
  // property replacement at load, the idiom /boundaries itself used on /map's
  // sync; with /boundaries unticked there is nothing to re-ink and this does
  // nothing.
  if (typeof feature_Boundaries !== 'undefined') {
    feature_Boundaries.styleOf = function (f) {
      return feature_LightBasemap.styleOf(f);
    };
  }
}
