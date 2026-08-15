const feature_BuildOrder = {
  sort(flat) {
    flat.sort((a, b) =>
      (b.build || 0) - (a.build || 0)
      || (a.ts < b.ts ? 1 : a.ts > b.ts ? -1 : 0)
      || (a.path < b.path ? -1 : 1));
  },
};
if (typeof feature_Chooser !== 'undefined') {
  const fm_buildOrderLoad = feature_Chooser.load.bind(feature_Chooser);
  feature_Chooser.load = async function () {
    await fm_buildOrderLoad();
    if (this.flat) feature_BuildOrder.sort(this.flat);
  };
}
