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
  // extension point: may a newer build apply without asking? (a policy
  // feature may replace this; the base always consents)
  consented: async () => true,
  // extension point: drop cached app files before an apply. The base drops
  // everything; a feature may replace this with something more precise.
  evict: async () => { try { await caches.delete('muon'); } catch (e) {} },
  async launch(who) {
    const v = await this.fetchVersion();
    this.server = v;
    if (typeof feature_Diag !== 'undefined')
      feature_Diag.report({ launch: true, running: this.running,
        server: v || 'offline', authed: !!(who && who.authed),
        pwa: typeof feature_Standalone !== 'undefined' && feature_Standalone.standalone(),
        sw: !!(navigator.serviceWorker && navigator.serviceWorker.controller),
        ua: navigator.userAgent.slice(0, 90) });
    if (v) {
      if (this.running !== 'first-run' && this.running !== v) {
        if (await this.consented(v)) {
          localStorage.muonVersion = v;
          await this.evict();
          location.reload();
          return;
        }
        // declined for now: stay (honestly) on the running build; the
        // pulsing handle carries the ask
        if (typeof feature_Watch !== 'undefined') feature_Watch.check();
        return;
      }
      localStorage.muonVersion = v;
      this.running = v;
    } else if (typeof feature_Honest !== 'undefined') {
      feature_Honest.retry(); // launched without network: keep trying
    }
  },
};
