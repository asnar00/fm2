const feature_Blackbox = {
  // the always-on ring: bounded by age and count, offline-first. the server
  // copy is best-effort; the ring is the record. entries are small event
  // deltas; full state lives in keyframes ([0] = boot state, /keyframes adds
  // periodic ones so replay needn't start from the beginning).
  ringMs: 5 * 60 * 1000,
  maxEntries: 500,
  key: 'muonBlackbox',
  log: { keyframes: [], entries: [], sentT: 0 },

  load() {
    try {
      const stored = JSON.parse(localStorage[this.key]);
      if (stored && stored.keyframes) this.log = stored;
    } catch (e) {}
  },
  save() {
    try { localStorage[this.key] = JSON.stringify(this.log); } catch (e) {}
  },
  record(event, stateAfter) {
    const _ = stateAfter;   // base keeps entries lean; /keyframes uses it
    this.log.entries.push({ t: Date.now(), event });
    this.trim();
    this.save();
  },
  trim() {
    const cutoff = Date.now() - this.ringMs;
    while (this.log.entries.length > this.maxEntries
           || (this.log.entries.length && this.log.entries[0].t < cutoff)) {
      this.log.entries.shift();
    }
    // keep the newest keyframe at-or-before the window start, drop older ones
    const windowStart = this.log.entries.length ? this.log.entries[0].t : cutoff;
    while (this.log.keyframes.length > 1 && this.log.keyframes[1].t <= windowStart) {
      this.log.keyframes.shift();
    }
  },
  async flush(hidden) {
    const unsentE = this.log.entries.filter((e) => e.t > (this.log.sentT || 0));
    const unsentK = this.log.keyframes.filter((k) => k.t > (this.log.sentT || 0));
    if (!unsentE.length && !unsentK.length) return;
    const newest = Math.max(
      unsentE.length ? unsentE[unsentE.length - 1].t : 0,
      unsentK.length ? unsentK[unsentK.length - 1].t : 0);
    try {
      const r = await fetch('blackbox/events', { method: 'POST',
        keepalive: !!hidden,
        body: JSON.stringify({ keyframes: unsentK, entries: unsentE }) });
      if (r.ok) {
        this.log.sentT = newest;
        this.save();
      }
    } catch (e) {}
  },
};

// wrap the event loop (the JS extension idiom: reassign around the original)
if (typeof feature_Events !== 'undefined') {
  feature_Blackbox.load();
  feature_Blackbox.flush();   // ship any previous session's tail (crashes included)
  const fm_bbSend = feature_Events.send;
  feature_Events.send = function (event) {
    fm_bbSend.call(this, event);
    feature_Blackbox.record(event, this.state);
  };
  const fm_bbApply = feature_Events.apply;
  feature_Events.apply = function (p) {
    fm_bbApply.call(this, p);
    if (!feature_Blackbox.log.keyframes.length) {
      feature_Blackbox.log.keyframes.push({ t: Date.now(), state: this.state });
      feature_Blackbox.save();
    }
  };
  setInterval(() => {
    if (document.visibilityState === 'visible') feature_Blackbox.flush();
  }, 10000);
  window.addEventListener('online', () => feature_Blackbox.flush());
  document.addEventListener('visibilitychange', () => {
    if (document.visibilityState === 'hidden') feature_Blackbox.flush(true);
  });
}
