const feature_ContextBias = {
  bonus: 0.08,

  family(p, home) {
    return p === home || p.startsWith(home + '/') || home.startsWith(p + '/');
  },

  // the open tool's registering node — the centre of the family
  home() {
    try {
      const tool = JSON.parse(feature_Loop.state || '{}').open_tool || '';
      if (!tool || typeof feature_Chooser === 'undefined' || !feature_Chooser.flat)
        return '';
      const n = feature_Chooser.flat.find((x) => x.tool === tool);
      return n ? n.path : '';
    } catch (e) { return ''; }
  },
};
if (typeof feature_SemanticFind !== 'undefined') {
  const fm_biasScore = feature_SemanticFind.score.bind(feature_SemanticFind);
  feature_SemanticFind.score = async function (q) {
    const scores = await fm_biasScore(q);
    const home = feature_ContextBias.home();
    if (home && scores) {
      for (let m = 0; m < this.paths.length; m++) {
        if (feature_ContextBias.family(this.paths[m], home))
          scores[m] += feature_ContextBias.bonus;
      }
    }
    return scores;
  };
}
