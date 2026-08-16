// receipts for the transcription rung: what ran, how long, on which build.
// The transcript itself never travels — only its length. See the node spec.
const feature_EngineReceipts = {
  report(data) {
    if (typeof feature_Diag === 'undefined') return;
    const running = typeof feature_Update !== 'undefined'
      ? feature_Update.running : '?';
    feature_Diag.report({ ...data, running });
  },

  fileOf(id) {
    try {
      const files = JSON.parse(feature_Loop.state || '{}').dict_files || [];
      return files.find((f) => f && f.id === id) || {};
    } catch (e) { return {}; }
  },

  install() {
    const orig = feature_Phone.run.bind(feature_Phone);
    const self = this;
    feature_Phone.run = async function (id, grade) {
      self.report({ stt: 'start', id, grade });
      const t0 = performance.now();
      try {
        await orig(id, grade);
      } finally {
        const ms = Math.round(performance.now() - t0);
        let device = 'unknown';
        try {
          const mod = await import('/stt/engine.js');
          if (mod.lastDevice) device = mod.lastDevice() || 'none';
        } catch (e) { /* engine absent: 'unknown' is the honest answer */ }
        const f = self.fileOf(id);
        const text = f.transcript || '';
        self.report({ stt: 'done', id, device, ms,
                      dur: f.dur || 0, chars: text.length,
                      ok: !f.t_err && text.length > 0,
                      err: String(f.t_err || '').slice(0, 160) });
      }
    };
  },
};
const fm_receiptsInit = setInterval(() => {
  if (typeof feature_Phone !== 'undefined' && feature_Phone.run) {
    clearInterval(fm_receiptsInit);
    feature_EngineReceipts.install();
  }
}, 100);
