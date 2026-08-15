const feature_Delta = {
  // is this path code the page runs, or data it reads?
  code(p) {
    return p === 'index.html' || p === 'client.wasm' || p === 'sw.js'
      || p === 'login.html' || p === 'install.html' || p.startsWith('f/');
  },

  stored() {
    try { return JSON.parse(localStorage.misoHashes || 'null'); } catch (e) { return null; }
  },
  store(m) {
    try { localStorage.misoHashes = JSON.stringify(m); } catch (e) {}
  },
  fetchLive() {
    return fetch('hashes.json', { cache: 'no-store' })
      .then((r) => r.ok ? r.json() : null).catch(() => null);
  },

  // the delta: paths whose hash changed or that vanished (new paths need no
  // eviction — nothing is cached under them yet)
  diff(old, fresh) {
    const changed = [];
    for (const p in old) {
      if (fresh[p] !== old[p]) changed.push(p);
    }
    return changed;
  },

  async evict() {
    const fresh = await this.fetchLive();
    const old = this.stored();
    if (!fresh || !old) {
      try { await caches.delete('miso'); } catch (e) {}
      if (fresh) this.store(fresh);
      return;
    }
    try {
      const cache = await caches.open('miso');
      for (const p of this.diff(old, fresh)) {
        await cache.delete(new URL(p, location.href).href, { ignoreSearch: true });
        if (p === 'index.html')
          await cache.delete(new URL('.', location.href).href, { ignoreSearch: true });
      }
    } catch (e) {}
    this.store(fresh);
  },

  // a build whose delta holds no code: stamp and evict data, skip the reload
  async quiet(build) {
    if (typeof feature_Update !== 'undefined') {
      await feature_Update.evict();
      feature_Update.running = String(build);
    }
    try { localStorage.misoVersion = String(build); } catch (e) {}
    const handle = $('build');
    if (handle) handle.classList.remove('update');
  },
};
{
  if (typeof feature_Update !== 'undefined')
    feature_Update.evict = () => feature_Delta.evict();

  if (typeof feature_Review !== 'undefined') {
    const fm_deltaApply = feature_Review.apply.bind(feature_Review);
    feature_Review.apply = async function (build) {
      const fresh = await feature_Delta.fetchLive();
      const old = feature_Delta.stored();
      if (fresh && old
          && !feature_Delta.diff(old, fresh).some((p) => feature_Delta.code(p))) {
        await feature_Delta.quiet(build);
        return;
      }
      await fm_deltaApply(build);
    };
  }

  // baseline: an instance that has never seen a manifest learns its own
  // right after launch settles — but only while it RUNS the build the live
  // manifest describes; seeding with a pending build's manifest would make
  // the next apply's delta read empty against old code
  if (typeof feature_Update !== 'undefined') {
    const fm_deltaLaunch = feature_Update.launch.bind(feature_Update);
    feature_Update.launch = async function (who) {
      await fm_deltaLaunch(who);
      if (!feature_Delta.stored() && this.server
          && this.running === this.server)
        feature_Delta.fetchLive().then((m) => { if (m) feature_Delta.store(m); });
    };
  }
}
