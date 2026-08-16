// the page half of per-feature logging: drain what the Rust turn gathered,
// and provide the call the linker rewrites page-side logging into. Lines go
// to the console and ride /blackbox to the mini. (#p23)
const feature_Logging = {
  // which paths are switched on for this user (absent means off)
  on(path) {
    try {
      const raw = JSON.parse(feature_Loop.state || '{}').feature_log;
      if (!raw) return false;
      const map = JSON.parse(raw);
      for (const p of Object.keys(map)) {
        if (!map[p]) continue;
        if (path === p || (path.length > p.length && path.startsWith(p)
                           && path[p.length] === '/')) return true;
      }
    } catch (e) {}
    return false;
  },

  emit(path, msg) {
    console.log('[' + path + '] ' + msg);
    // ride the flight recorder: batching, offline survival and the trip to
    // the mini all already exist there
    if (typeof feature_Blackbox !== 'undefined' && feature_Blackbox.log
        && Array.isArray(feature_Blackbox.log.entries)) {
      feature_Blackbox.log.entries.push({ t: Date.now(),
        log: { p: path, m: String(msg).slice(0, 400),
               i: typeof feature_Instance !== 'undefined' ? feature_Instance.id : '' } });
      if (feature_Blackbox.trim) feature_Blackbox.trim();
      if (feature_Blackbox.save) feature_Blackbox.save();
    }
  },

  // what the Rust half gathered this turn, handed over through state
  drain() {
    try {
      const s = JSON.parse(feature_Loop.state || '{}');
      if (!Array.isArray(s._log) || !s._log.length) return;
      const lines = s._log;
      delete s._log;
      feature_Loop.state = JSON.stringify(s);
      for (const l of lines) this.emit(l.p, l.m);
    } catch (e) {}
  },
};

// what `fm_log(...)` becomes in page fragments; the linker supplies the path
function fm_log_at(path) {
  if (!feature_Logging.on(path)) return;
  const parts = [];
  for (let i = 1; i < arguments.length; i++) {
    const a = arguments[i];
    parts.push(typeof a === 'string' ? a : (() => {
      try { return JSON.stringify(a); } catch (e) { return String(a); }
    })());
  }
  feature_Logging.emit(path, parts.join(' '));
}
{
  const fm_jApply = feature_Loop.apply;
  feature_Loop.apply = function (p) {
    fm_jApply.call(this, p);
    feature_Logging.drain();
  };
}
