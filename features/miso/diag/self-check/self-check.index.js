// the boot self-check: after paint, hash every code fragment this device
// actually holds against the live manifest, probe three basics, and post the
// result as a second diag/report line (kind "self-check") — the only layer
// that sees the real phone (saturday #p31: three phone-only divergences that
// nothing could diagnose).
const feature_SelfCheck = {
  last: null,
  running: null,

  device() {
    try {
      if (!localStorage.misoDevice) {
        const b = new Uint8Array(4); crypto.getRandomValues(b);
        localStorage.misoDevice = [...b].map((x) => x.toString(16).padStart(2, '0')).join('');
      }
      return localStorage.misoDevice;
    } catch (e) { return 'nostore'; }
  },

  // which paths are code the page runs (the delta's own judgement when present)
  code(p) {
    if (typeof feature_Delta !== 'undefined') return feature_Delta.code(p);
    return p === 'index.html' || p === 'client.wasm' || p === 'sw.js'
      || p === 'login.html' || p === 'install.html' || p.startsWith('f/');
  },
  // the live manifest, through the delta's fetch when present (reuse, no duplicate)
  manifest() {
    if (typeof feature_Delta !== 'undefined') return feature_Delta.fetchLive();
    return fetch('hashes.json', { cache: 'no-store' })
      .then((r) => r.ok ? r.json() : null).catch(() => null);
  },
  async sha1(buf) {
    const d = await crypto.subtle.digest('SHA-1', buf);
    return [...new Uint8Array(d)].map((b) => b.toString(16).padStart(2, '0')).join('').slice(0, 16);
  },

  // what this device holds for a path: the service worker's cache entry when
  // there is one (what the app runs offline or under /fresh's deadline), else
  // what a fetch brings now (a device with no cache runs the network's copy)
  async hashOf(path) {
    const url = new URL(path, location.href).href;
    let res = null, from = 'cache';
    try {
      if (window.caches) {
        const c = await caches.open('miso');
        res = await c.match(url, { ignoreSearch: true });
      }
    } catch (e) { res = null; }
    if (!res) {
      from = 'fetch';
      try { res = await fetch(url); if (!res.ok) res = null; } catch (e) { res = null; }
    }
    if (!res) return { from: 'missing', hash: null };
    try { return { from, hash: await this.sha1(await res.arrayBuffer()) }; }
    catch (e) { return { from, hash: null }; }
  },

  async fragments() {
    const out = { manifest: false, count: 0, cached: 0, mismatched: [], missing: [], unhashed: 0 };
    const m = await this.manifest();
    if (!m) return out;
    out.manifest = true;
    const paths = Object.keys(m).filter((p) => this.code(p)).sort();
    out.count = paths.length;
    for (const p of paths) {
      const h = await this.hashOf(p);
      if (h.from === 'missing') { out.missing.push(p); continue; }
      if (h.from === 'cache') out.cached++;
      if (h.hash === null) out.unhashed++;
      else if (h.hash !== m[p]) out.mismatched.push(p);
    }
    return out;
  },

  // three basics, each a boolean with its reason beside it
  basics() {
    const b = { tap: false, veil: false, wrappers: false, orphans: [] };
    // (a) the tap seam: the lozenge's tap reaches feature_Panel.open — probed
    // by standing a counter in for open, calling the seam, restoring
    if (typeof feature_Panel !== 'undefined' && feature_Panel.buttonTap) {
      const real = feature_Panel.open;
      let hit = 0;
      try {
        feature_Panel.open = () => { hit++; return Promise.resolve(); };
        feature_Panel.buttonTap();
      } catch (e) { hit = -1; }
      feature_Panel.open = real;
      const btn = $('build');
      b.tap = hit === 1 && !!(btn && btn.onclick);
    }
    // (b) the boot veil is lifted (or was never composed)
    b.veil = typeof feature_Veil === 'undefined'
      || (!$('veil') && document.body.classList.contains('fm-joined'));
    // (c) no orphaned update wrapper: the seams /delta replaced at load still
    // resolve through it — the linker's enablement trampoline names the
    // tenant node's path, a bare replacement names its struct (a stale
    // fragment mix would show as neither)
    if (typeof feature_Delta !== 'undefined' && typeof feature_Update !== 'undefined') {
      const viaDelta = (fn) => /feature_Delta|review\/delta"/.test(String(fn));
      if (!viaDelta(feature_Update.evict)) b.orphans.push('update.evict');
      if (!viaDelta(feature_Update.launch)) b.orphans.push('update.launch');
    }
    b.wrappers = b.orphans.length === 0;
    return b;
  },

  run() {
    if (this.running) return this.running;
    this.running = (async () => {
      const t0 = performance.now();
      const up = typeof feature_Update !== 'undefined' ? feature_Update : null;
      const f = await this.fragments();
      const b = this.basics();
      const r = {
        kind: 'self-check', device: this.device(),
        running: up ? up.running : '?', server: up && up.server ? up.server : 'offline',
        sw: !!(navigator.serviceWorker && navigator.serviceWorker.controller),
        pwa: typeof feature_Standalone !== 'undefined' && feature_Standalone.standalone(),
        manifest: f.manifest, count: f.count, cached: f.cached, unhashed: f.unhashed,
        mismatched: f.mismatched, missing: f.missing,
        tap: b.tap, veil: b.veil, wrappers: b.wrappers, orphans: b.orphans,
        ms: Math.round(performance.now() - t0),
      };
      r.ok = f.manifest && !f.mismatched.length && !f.missing.length && b.tap && b.veil && b.wrappers;
      this.last = r;
      this.running = null;
      if (typeof feature_Diag !== 'undefined') feature_Diag.report(r);
      return r;
    })();
    return this.running;
  },

  // the report as plain text, for the engineer section
  text() {
    const r = this.last;
    if (!r) return 'self-check: ' + (this.running ? 'running…' : 'not run yet');
    const lines = [
      'self-check ' + (r.ok ? 'ok' : 'FAIL') + ' · build ' + r.running + ' (server ' + r.server + ')'
        + ' · sw ' + (r.sw ? 'controlled' : 'none') + (r.pwa ? ' · pwa' : '') + ' · ' + r.ms + 'ms',
      r.manifest
        ? 'fragments ' + r.count + ' · cached ' + r.cached + ' · mismatched ' + r.mismatched.length
          + ' · missing ' + r.missing.length + (r.unhashed ? ' · unhashed ' + r.unhashed : '')
        : 'fragments: no manifest (hashes.json) — a dev build, or offline',
      'tap ' + (r.tap ? 'ok' : 'FAIL') + ' · veil ' + (r.veil ? 'ok' : 'FAIL')
        + ' · wrappers ' + (r.wrappers ? 'ok' : 'FAIL: ' + r.orphans.join(' ')),
    ];
    if (r.mismatched.length) lines.push('mismatched: ' + r.mismatched.join(' '));
    if (r.missing.length) lines.push('missing: ' + r.missing.join(' '));
    lines.push('device ' + r.device);
    return lines.join('\n');
  },
};
{
  // after paint: once the loop has state, a beat later (the veil's own
  // budget is 2s), once per page load. This node draws nothing: /engineer,
  // the later node, shows text() under its gear.
  const fm_scBoot = setInterval(() => {
    if (typeof feature_Loop !== 'undefined' && feature_Loop.state !== null) {
      clearInterval(fm_scBoot);
      setTimeout(() => feature_SelfCheck.run(), 2500);
    }
  }, 100);
}
