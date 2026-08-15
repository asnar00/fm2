const feature_ConsentOnce = {
  // the single key: has this user accepted this build? Loop state when we
  // have it; the localStorage mirror at launch, before /join delivers.
  accepted() {
    let a = 0;
    try {
      const s = JSON.parse(feature_Loop.state || '{}');
      a = parseInt(s.update_accepted || '0', 10) || 0;
    } catch (e) {}
    if (!a) a = parseInt(localStorage.misoAccepted || '0', 10) || 0;
    return a;
  },
};
{
  // launch consent = acceptance covers the build; nothing else opens the door
  if (typeof feature_Update !== 'undefined')
    feature_Update.consented = async (v) =>
      feature_ConsentOnce.accepted() >= (parseInt(v, 10) || 0);

  // /auto's per-device self-apply stands down: mid-session application is
  // /review's watch (the acceptance arriving over sync) and nothing else
  if (typeof feature_Auto !== 'undefined')
    feature_Auto.act = function () {};

  // mirror the acceptance for the next launch
  const fm_consentOnceApply = feature_Loop.apply;
  feature_Loop.apply = function (p) {
    fm_consentOnceApply.call(this, p);
    try {
      const s = JSON.parse(feature_Loop.state || '{}');
      if (s.update_accepted) localStorage.misoAccepted = s.update_accepted;
    } catch (e) {}
  };
}
