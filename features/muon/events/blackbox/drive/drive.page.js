const feature_Drive = {
  active: /(^|[?&])drive=/.test(location.search),
  async poll() {
    const cmd = await fetch('/diag/drive/next', { cache: 'no-store' })
      .then((r) => r.ok ? r.json() : null).catch(() => null);
    if (!cmd) return;
    if (cmd.send && typeof feature_Events !== 'undefined') {
      feature_Events.send(cmd.send);
    }
    if (cmd.tap) {
      const el = document.querySelector(cmd.tap);
      if (el) el.click();
    }
    if (cmd.type) {
      const el = document.querySelector(cmd.type);
      if (el) {
        el.value = cmd.value || '';
        el.dispatchEvent(new Event('input', { bubbles: true }));
      }
    }
  },
};
if (feature_Drive.active) {
  // a driven demo is not a user session: don't bounce it to login
  if (typeof feature_Gate !== 'undefined')
    feature_Gate.redirectIfLoggedOut = () => false;
  setInterval(() => feature_Drive.poll(), 250);
}
