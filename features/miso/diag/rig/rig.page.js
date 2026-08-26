// on a rig server, every page is observable and driveable from its first
// paint: /readout posts the screen, /drive polls for commands, and the
// /blackbox ships every second instead of every ten — with no query string,
// so an installed home-screen app is under test from launch (#p164a).
const feature_RigPage = {
  on: false,
  arm() {
    this.on = true;
    // a rig runs the code it was given: no service worker, no cache, so
    // every cold launch fetches the site as it is now (the version-driven
    // reload does not fire in a standalone app on a relaunch)
    if (navigator.serviceWorker) navigator.serviceWorker.getRegistrations().then((rs) => rs.forEach((r) => r.unregister())).catch(() => {});
    if (window.caches) caches.keys().then((ks) => ks.forEach((k) => caches.delete(k))).catch(() => {});
    if (typeof feature_Readout !== 'undefined' && !feature_Readout.active) {
      feature_Readout.active = true;
      new MutationObserver(() => feature_Readout.schedule()).observe(
        document.documentElement,
        { subtree: true, childList: true, characterData: true, attributes: true });
      feature_Readout.schedule();
    }
    if (typeof feature_Drive !== 'undefined' && !feature_Drive.active) {
      feature_Drive.active = true;
      // on a rig, a drive command may also carry `js`: a script run on the
      // page, its value posted back through the readout's door as
      // {t, url, js: value}. Setup and assertions, not the interaction under
      // test — that is the finger's. Localhost only: the rig answered.
      const fm_rigPoll = feature_Drive.poll.bind(feature_Drive);
      feature_Drive.poll = async function () {
        const cmd = await fetch('/diag/drive/next', { cache: 'no-store' })
          .then((r) => r.ok ? r.json() : null).catch(() => null);
        if (!cmd) return;
        if (cmd.js) {
          let value;
          try { value = await (new Function(cmd.js))(); } catch (e) { value = 'error: ' + e.message; }
          fetch('/diag/readout', { method: 'POST', body: JSON.stringify({ t: new Date().toISOString(), url: location.pathname, js: value === undefined ? null : value, body: feature_Readout.capture(document.body) }) }).catch(() => {});
          return;
        }
        if (cmd.send && typeof feature_Loop !== 'undefined') feature_Loop.send(cmd.send);
        if (cmd.tap) { const el = document.querySelector(cmd.tap); if (el) el.click(); }
        if (cmd.type) { const el = document.querySelector(cmd.type); if (el) { el.value = cmd.value || ''; el.dispatchEvent(new Event('input', { bubbles: true })); } }
      };
      setInterval(() => feature_Drive.poll(), 250);
    }
    if (typeof feature_Blackbox !== 'undefined') {
      setInterval(() => { if (document.visibilityState === 'visible') feature_Blackbox.flush(); }, 1000);
    }
  },
};
fetch('/diag/rig', { cache: 'no-store' }).then((r) => r.ok ? r.json() : null)
  .then((j) => { if (j && j.rig) feature_RigPage.arm(); }).catch(() => {});
