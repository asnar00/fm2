const feature_Dictate = {
  db: null, media: null, recorder: null, chunks: [], startedAt: 0, active: false,

  openDb() {
    return new Promise((res, rej) => {
      const rq = indexedDB.open('miso-blobs', 1);
      rq.onupgradeneeded = () => rq.result.createObjectStore('audio');
      rq.onsuccess = () => res(rq.result);
      rq.onerror = () => rej(rq.error);
    });
  },
  put(id, value) {
    return new Promise((res, rej) => {
      const tx = this.db.transaction('audio', 'readwrite');
      tx.objectStore('audio').put(value, id);
      tx.oncomplete = res; tx.onerror = () => rej(tx.error);
    });
  },
  // list stored metadata (kept under 'meta:' keys beside the blobs)
  list() {
    return new Promise((res) => {
      const items = [];
      const tx = this.db.transaction('audio', 'readonly');
      const cur = tx.objectStore('audio').openCursor();
      cur.onsuccess = () => {
        const c = cur.result;
        if (!c) return res(items);
        if (String(c.key).startsWith('meta:')) items.push(c.value);
        c.continue();
      };
      cur.onerror = () => res(items);
    });
  },

  async start() {
    try {
      this.media = await navigator.mediaDevices.getUserMedia({ audio: true });
    } catch (e) {
      feature_Loop.send({ type: 'click', ev: 'dict_stop' }); // state must not lie
      return;
    }
    this.chunks = [];
    this.startedAt = Date.now();
    this.recorder = new MediaRecorder(this.media);
    this.recorder.ondataavailable = (e) => { if (e.data.size) this.chunks.push(e.data); };
    this.recorder.onstop = () => this.save();
    this.recorder.start();
  },
  stop() {
    if (this.recorder && this.recorder.state !== 'inactive') this.recorder.stop();
    if (this.media) { this.media.getTracks().forEach((t) => t.stop()); this.media = null; }
  },
  async save() {
    const blob = new Blob(this.chunks, { type: this.recorder.mimeType });
    const id = 'rec-' + this.startedAt;
    const t = new Date(this.startedAt);
    const meta = {
      id, t: this.startedAt, here: true,
      dur: Math.round((Date.now() - this.startedAt) / 1000),
      size: blob.size, mime: this.recorder.mimeType,
      label: t.getHours() + ':' + String(t.getMinutes()).padStart(2, '0'),
    };
    await this.put(id, blob);
    await this.put('meta:' + id, meta);
    feature_Loop.send({ type: 'RecSaved', data: meta });
  },

  // state-driven effects: mic and speaker follow state's edges. During a
  // /replay the state changes are re-enactment, not intent — no hardware.
  watch() {
    if (typeof feature_Replay !== 'undefined' && feature_Replay.active) return;
    let s = {};
    try { s = JSON.parse(feature_Loop.state || '{}'); } catch (e) {}
    const wanted = !!s.dict_recording;
    if (wanted && !this.active) { this.active = true; this.start(); }
    if (!wanted && this.active) { this.active = false; this.stop(); }
    this.watchPlay(s.dict_playing || '');
  },

  getBlob(id) {
    return new Promise((res) => {
      const rq = this.db.transaction('audio', 'readonly').objectStore('audio').get(id);
      rq.onsuccess = () => res(rq.result);
      rq.onerror = () => res(null);
    });
  },
  playingId: '', audio: null,
  watchPlay(want) {
    if (want === this.playingId) return;
    if (this.audio) { this.audio.pause(); this.audio = null; }
    this.playingId = want;
    if (!want) return;
    this.getBlob(want).then((blob) => {
      if (!blob || this.playingId !== want) return;
      const url = URL.createObjectURL(blob);
      const done = () => { URL.revokeObjectURL(url); feature_Loop.send({ type: 'PlayEnded' }); };
      this.audio = new Audio(url);
      this.audio.onended = done;
      this.audio.onerror = done;
      this.audio.play().catch(done); // state must not claim playback that isn't
    });
  },

  async init() {
    this.db = await this.openDb();
    const fm_dictApply = feature_Loop.apply;
    const self = this;
    feature_Loop.apply = function (p) {
      fm_dictApply.call(this, p);
      self.watch();
    };
    const items = await this.list();
    items.sort((a, b) => a.t - b.t);
    items.forEach((i) => { i.here = true; });   // locally stored = here
    feature_Loop.send({ type: 'RecList', data: { items } });
  },
};
const fm_dictInit = setInterval(() => {
  if (typeof feature_Loop !== 'undefined' && feature_Loop.state !== null) {
    clearInterval(fm_dictInit);
    feature_Dictate.init();
  }
}, 100);
