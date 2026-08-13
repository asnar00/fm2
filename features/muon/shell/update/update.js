const feature_Update = {
  // silent launch self-refresh: compare the deploy stamp with the build we
  // launched from; on change, drop the cache and reload once.
  running: localStorage.muonVersion || 'first-run',
  server: null,
  fetchVersion: () => fetch('version', { cache: 'no-store' })
    .then((r) => r.ok ? r.text() : null).then((t) => t ? t.trim() : null)
    .catch(() => null),
  newer() {
    return this.server && this.running !== 'first-run'
      && this.server !== this.running;
  },
  async launch(who) {
    const v = await this.fetchVersion();
    this.server = v;
    if (typeof feature_Diag !== 'undefined')
      feature_Diag.report({ launch: true, running: this.running,
        server: v || 'offline', authed: !!(who && who.authed),
        pwa: typeof feature_Pwa !== 'undefined' && feature_Pwa.standalone(),
        sw: !!(navigator.serviceWorker && navigator.serviceWorker.controller),
        ua: navigator.userAgent.slice(0, 90) });
    if (v) {
      localStorage.muonVersion = v;
      if (this.running !== 'first-run' && this.running !== v) {
        await caches.delete('muon');
        location.reload();
        return;
      }
      this.running = v;
    } else if (typeof feature_Honest !== 'undefined') {
      feature_Honest.retry(); // launched without network: keep trying
    }
  },
};
