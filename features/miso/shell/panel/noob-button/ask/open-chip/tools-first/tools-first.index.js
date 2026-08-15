const feature_ToolsFirst = {
  // the node that registered this tool — its row is the tool's readout
  owner(toolId) {
    if (typeof feature_Chooser === 'undefined' || !feature_Chooser.flat) return null;
    return feature_Chooser.flat.find((n) => n.tool === toolId) || null;
  },
};
if (typeof feature_Ask !== 'undefined' && typeof feature_OpenChip !== 'undefined') {
  const fm_toolsFirstFeatures = feature_Ask.features.bind(feature_Ask);
  feature_Ask.features = async function (words) {
    const hits = await fm_toolsFirstFeatures(words);
    const owners = [];
    const seen = new Set();
    for (const n of hits) {
      const tool = feature_OpenChip.toolFor(n.path);
      if (!tool || seen.has(tool)) continue;
      seen.add(tool);
      const owner = feature_ToolsFirst.owner(tool);
      if (owner) owners.push(owner);
    }
    // no tool in sight: capability questions keep the reading path
    return owners.length ? owners : hits;
  };
}
