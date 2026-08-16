// which of your instances is this? a short name, minted once per install.
// Not a secret and not an identity — identity rides the cookie. (#p24a)
const feature_Instance = {
  id: (() => {
    const mint = () => Math.random().toString(36).slice(2, 8);
    try {
      let v = localStorage.misoInstance;
      if (!v) { v = mint(); localStorage.misoInstance = v; }
      return v;
    } catch (e) {
      return 's-' + mint();   // storage walled off: per-session, still attributable
    }
  })(),
};
{
  // every report says which instance sent it
  if (typeof feature_Diag !== 'undefined' && feature_Diag.report) {
    const fm_instReport = feature_Diag.report;
    feature_Diag.report = (data) =>
      fm_instReport({ ...data, inst: feature_Instance.id });
  }
}
