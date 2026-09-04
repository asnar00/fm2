// the clip goes up while it is being made. MediaRecorder is asked for a piece
// every two seconds and each piece is posted as it arrives, so when the finger
// leaves the stop button most of the note is already on the server.
//
// Everything here is done through seams /video and /mirror opened for it; not
// a line of either is edited, and with this node unticked both behave exactly
// as they did.
const feature_Streams = {
  SLICE: 2000,
  // a piece is under a megabyte at this bitrate; the ceiling is the server's
  // and is here only so a wedged recorder cannot try to post the moon.
  MAX_PART: 8388608,
  MAX_PARTS: 200,

  id: '', count: 0, sending: {},

  key(id, n) { return 'p:' + id + ':' + n; },

  // ---- while recording -----------------------------------------------------
  // a piece is written to the device's own store FIRST and posted second: the
  // point of the store is that a piece the network refused can be sent later,
  // and a piece that was never written is a piece that cannot.
  async chunk(blob, n) {
    if (typeof feature_Video === 'undefined' || !feature_Video.startedAt) return;
    const id = 'vid-' + feature_Video.startedAt;
    if (id !== this.id) { this.id = id; this.count = 0; this.sending = {}; }
    if (n >= this.MAX_PARTS || blob.size > this.MAX_PART) return;
    this.count = Math.max(this.count, n + 1);
    try {
      if (typeof feature_Dictate !== 'undefined' && feature_Dictate.db) {
        await feature_Dictate.put(this.key(id, n), blob);
      }
    } catch (e) { /* a full device: the whole-clip road still has the bytes */ }
    await this.post(id, n, blob);
  },

  async post(id, n, blob) {
    if (this.sending[id + ':' + n]) return false;
    this.sending[id + ':' + n] = true;
    try {
      const r = await fetch('blob/' + id + '/part/' + n, { method: 'POST', body: blob });
      return r.ok;
    } catch (e) {
      return false;
    } finally {
      this.sending[id + ':' + n] = false;
    }
  },

  // ---- catching up ---------------------------------------------------------
  // /mirror's upload() pass comes here for a recording's bytes. Parts first,
  // oldest first; and if any of them cannot be sent — the piece was never
  // stored, the server refused it — the whole clip goes the old way and the
  // metadata says `parts: 0`, which is how the server is told to stop waiting
  // for pieces. Degrading to yesterday's behaviour is the point: a note that
  // arrives late is a note; a note that never arrives is not.
  async sendBytes(meta, blob) {
    const want = meta && meta.parts ? meta.parts : 0;
    if (!want) return this.whole(meta, blob);
    for (let n = 0; n < want; n++) {
      const part = await feature_Dictate.getBlob(this.key(meta.id, n));
      if (!part) return this.whole(meta, blob);
      if (!(await this.post(meta.id, n, part))) return false;   // offline: try later
    }
    await this.tidy(meta.id, want);
    return true;
  },

  async whole(meta, blob) {
    const r = await fetch('blob/' + meta.id, { method: 'POST', body: blob });
    if (r.ok && meta) meta.parts = 0;
    return r.ok;
  },

  // ---- the pieces on the device --------------------------------------------
  // once the exchange holds every piece there is nothing left for the copies
  // here to do — the whole clip is in the store beside them — and a device
  // that keeps both keeps every note twice. /dictate has no delete of its own,
  // so this opens a transaction on the store it publishes, for this node's own
  // keys only ('p:<id>:<n>'); nothing else's key is ever named here.
  del(key) {
    return new Promise((res) => {
      try {
        const tx = feature_Dictate.db.transaction('audio', 'readwrite');
        tx.objectStore('audio').delete(key);
        tx.oncomplete = res; tx.onerror = res;
      } catch (e) { res(); }
    });
  },

  async tidy(id, want) {
    for (let n = 0; n < want; n++) {
      await this.del(this.key(id, n));
    }
  },

  install() {
    if (typeof feature_Video === 'undefined' || feature_Video.fm_streams) return;
    feature_Video.fm_streams = true;
    const self = this;
    feature_Video.timeslice = function () { return self.SLICE; };
    feature_Video.onChunk = function (blob, n) { self.chunk(blob, n); };
    feature_Video.metaFor = function (meta) {
      return Object.assign({}, meta, { parts: self.count });
    };
    // the companion audio-only recording went with /phone: it existed to feed
    // a model on this device, and there is no model on this device any more.
    // A second encode of every note, stored and never read, is what it would
    // be now — so this node retires it through the seam rather than by
    // editing the recorder that owns it.
    feature_Video.companionAudio = function () { this.arec = null; };
  },

  installMirror() {
    if (typeof feature_Mirror === 'undefined' || feature_Mirror.fm_streams) return;
    feature_Mirror.fm_streams = true;
    const self = this;
    feature_Mirror.sendBytes = function (meta, blob) { return self.sendBytes(meta, blob); };
  },
};
const fm_streamsInit = setInterval(() => {
  if (typeof feature_Video === 'undefined' || typeof feature_Mirror === 'undefined') return;
  clearInterval(fm_streamsInit);
  feature_Streams.install();
  feature_Streams.installMirror();
}, 100);
