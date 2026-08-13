const feature_Blackbox = {
  // the always-on ring: bounded by age and count, offline-first. the server
  // copy is best-effort; the ring is the record.
  ringMs: 5 * 60 * 1000,
  maxEntries: 500,
  key: 'muonBlackbox',
  log: { baseline: null, entries: [], sentT: 0 },

  load() {
    try { this.log = JSON.parse(localStorage[this.key]) || this.log; } catch (e) {}
  },
  save() {
    try { localStorage[this.key] = JSON.stringify(this.log); } catch (e) {}
  },
  record(event, stateAfter) {
    this.log.entries.push({ t: Date.now(), event, state: stateAfter });
    this.trim();
    this.save();
  },
  trim() {
    const cutoff = Date.now() - this.ringMs;
    while (this.log.entries.length > this.maxEntries
           || (this.log.entries.length && this.log.entries[0].t < cutoff)) {
      const dropped = this.log.entries.shift();
      this.log.baseline = dropped.state;   // window stays replayable
    }
  },
  async flush(hidden) {
    const unsent = this.log.entries.filter((e) => e.t > (this.log.sentT || 0));
    if (!unsent.length) return;
    try {
      const r = await fetch('blackbox/events', { method: 'POST',
        keepalive: !!hidden,
        body: JSON.stringify({ baseline: this.log.baseline, entries: unsent }) });
      if (r.ok) {
        this.log.sentT = unsent[unsent.length - 1].t;
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
    if (!feature_Blackbox.log.baseline) {
      feature_Blackbox.log.baseline = this.state;
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
