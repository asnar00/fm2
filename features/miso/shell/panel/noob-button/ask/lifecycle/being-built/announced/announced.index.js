// what is being built for everyone, announced by the builder. An ask made
// in conversation with the builder has no record in anyone's asks list, so
// nothing said it was under way (#p150a). `builds` is a global list of
// {t, text, status, build?} the builder writes from the repo
// (tools/stamp_ask.py --announce); its building entries join the "building"
// rows in every world's sheet, newest first, and leave when shipped — the
// shipped ones are already in the release list.
const feature_Announced = {
  building() {
    try {
      const s = JSON.parse(feature_Loop.state || '{}');
      const list = JSON.parse(s.builds || '[]');
      return Array.isArray(list)
        ? list.filter((a) => a.status === 'building')
        : [];
    } catch (e) { return []; }
  },
};
if (typeof feature_BeingBuilt !== 'undefined') {
  const fm_announcedBuilding = feature_BeingBuilt.building.bind(feature_BeingBuilt);
  feature_BeingBuilt.building = function () {
    const own = fm_announcedBuilding();
    const seen = new Set(own.map((a) => a.t));
    return own.concat(feature_Announced.building().filter((a) => !seen.has(a.t)));
  };
}
