const feature_Birthplace = {
  // where is the user right now? the open tool, and its registering node
  context() {
    let tool = '';
    try { tool = JSON.parse(feature_Loop.state || '{}').open_tool || ''; } catch (e) {}
    if (!tool) return {};
    let at = '';
    if (typeof feature_Chooser !== 'undefined' && feature_Chooser.flat) {
      const n = feature_Chooser.flat.find((x) => x.tool === tool);
      if (n) at = n.path;
    }
    return at ? { tool, at } : { tool };
  },
};
if (typeof feature_Ask !== 'undefined') {
  feature_Ask.file = function (text) {
    feature_Loop.send({ type: 'Ask',
      data: Object.assign({ t: Date.now(), text }, feature_Birthplace.context()) });
  };
}
