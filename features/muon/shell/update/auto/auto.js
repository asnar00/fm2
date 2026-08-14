const feature_Auto = {
  act() {
    if (typeof feature_Update === 'undefined' || !feature_Update.newer()) return;
    if (typeof feature_Replay !== 'undefined' && feature_Replay.active) return;
    if (document.visibilityState !== 'visible') return;
    localStorage.muonVersion = feature_Update.server;
    caches.delete('muon').then(() => location.reload());
  },
};
if (typeof feature_Watch !== 'undefined') {
  const fm_autoCheck = feature_Watch.check;
  feature_Watch.check = async function () {
    const v = await fm_autoCheck.call(this);
    feature_Auto.act();
    return v;
  };
}
