const feature_Mirror = {
  // ---- lazy audio: a missing blob fetches from the exchange on first play
  installFetch() {
    const orig = feature_Dictate.getBlob.bind(feature_Dictate);
    feature_Dictate.getBlob = async (id) => {
      const local = await orig(id);
      if (local) return local;
      try {
        const r = await fetch('blob/' + id);
        if (!r.ok) return null;
        const blob = await r.blob();
        await feature_Dictate.put(id, blob);
        // record a local meta too: restarts then list it as here, and it
        // never re-uploads (the exchange is where it came from)
        try {
          const files = JSON.parse(feature_Loop.state || '{}').dict_files || [];
          const m = files.find((f) => f && f.id === id);
          if (m) await feature_Dictate.put('meta:' + id, { ...m, here: true, uploaded: true });
        } catch (e) {}
        feature_Loop.send({ type: 'RecFetched', data: { id } });
        return blob;
      } catch (e) { return null; }
    };
  },

  // ---- eager metadata + catch-up upload: every not-yet-uploaded blob POSTs
  // to the exchange, then announces through the persistent outbox
  uploading: false,
  async upload() {
    if (this.uploading) return;
    this.uploading = true;
    try {
      const metas = await feature_Dictate.list();
      for (const m of metas) {
        if (m.uploaded) continue;
        const blob = await feature_Dictate.getBlob(m.id);
        if (!blob) continue;
        try {
          const r = await fetch('blob/' + m.id, { method: 'POST', body: blob });
          if (!r.ok) break;             // offline or refused: retry later
        } catch (e) { break; }
        m.uploaded = true;
        await feature_Dictate.put('meta:' + m.id, m);
        const { uploaded, ...meta } = m;
        feature_Messaging.queue.push({ type: 'RecShared', data: { ...meta, here: undefined } });
        feature_Messaging.save();
      }
      feature_Messaging.flush();
    } finally { this.uploading = false; }
  },

  init() {
    this.installFetch();
    // boot catch-up: one index request through the outbox
    if (!feature_Messaging.queue.some((m) => m && m.type === 'RecIndex')) {
      feature_Messaging.queue.push({ type: 'RecIndex' });
      feature_Messaging.save();
    }
    // upload on: startup, each new recording (seen via apply), reconnect
    const fm_mirrorApply = feature_Loop.apply;
    const self = this;
    let lastCount = -1;
    feature_Loop.apply = function (p) {
      fm_mirrorApply.call(this, p);
      try {
        const files = JSON.parse(feature_Loop.state || '{}').dict_files || [];
        if (files.length !== lastCount) {
          lastCount = files.length;
          self.upload();
        }
      } catch (e) {}
    };
    window.addEventListener('online', () => self.upload());
    this.upload();
  },
};
const fm_mirrorInit = setInterval(() => {
  if (typeof feature_Dictate !== 'undefined' && feature_Dictate.db &&
      typeof feature_Messaging !== 'undefined' && feature_Messaging.queue) {
    clearInterval(fm_mirrorInit);
    feature_Mirror.init();
  }
}, 100);
