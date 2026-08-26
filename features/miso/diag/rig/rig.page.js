// on a rig server, every page is observable and driveable from its first
// paint: /readout posts the screen, /drive polls for commands, and the
// /blackbox ships every second instead of every ten — with no query string,
// so an installed home-screen app is under test from launch (#p164a).
const feature_RigPage = {
  on: false,
  arm() {
    this.on = true;
    if (typeof feature_Readout !== 'undefined' && !feature_Readout.active) {
      feature_Readout.active = true;
      new MutationObserver(() => feature_Readout.schedule()).observe(
        document.documentElement,
        { subtree: true, childList: true, characterData: true, attributes: true });
      feature_Readout.schedule();
    }
    if (typeof feature_Drive !== 'undefined' && !feature_Drive.active) {
      feature_Drive.active = true;
      setInterval(() => feature_Drive.poll(), 250);
    }
    if (typeof feature_Blackbox !== 'undefined') {
      setInterval(() => { if (document.visibilityState === 'visible') feature_Blackbox.flush(); }, 1000);
    }
  },
};
fetch('/diag/rig', { cache: 'no-store' }).then((r) => r.ok ? r.json() : null)
  .then((j) => { if (j && j.rig) feature_RigPage.arm(); }).catch(() => {});
