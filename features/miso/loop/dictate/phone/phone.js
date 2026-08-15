const feature_Phone = {
  busy: '', worker: null,

  // decode any recorded container (mp4/AAC on iOS, webm/opus elsewhere)
  // to what whisper eats: 16kHz mono float PCM
  async pcm16k(blob) {
    const raw = await blob.arrayBuffer();
    const ac = new AudioContext();
    const buf = await ac.decodeAudioData(raw);
    ac.close();
    const len = Math.max(1, Math.ceil(buf.duration * 16000));
    const oc = new OfflineAudioContext(1, len, 16000);
    const src = oc.createBufferSource();
    src.buffer = buf;
    src.connect(oc.destination);
    src.start();
    const out = await oc.startRendering();
    return out.getChannelData(0);
  },

  async run(id, grade) {
    try {
      const blob = typeof feature_Dictate !== 'undefined'
        ? await feature_Dictate.getBlob(id) : null;
      if (!blob) throw new Error('no blob');
      const audio = await this.pcm16k(blob);
      const engine = await import('/stt/engine.js');
      const text = await engine.transcribe(audio);
      this.busy = '';
      feature_Loop.send({ type: 'Transcribed',
        data: { id, text, rung: 'local', grade } });
    } catch (e) {
      // stamp the failed attempt so the scheduler moves on, never loops;
      // the error rides along for /diag device reports
      this.busy = '';
      feature_Loop.send({ type: 'Transcribed',
        data: { id, text: '', rung: 'local', grade, failed: true,
                err: String(e && e.message || e).slice(0, 200) } });
    }
  },

  // transcription is compute, not hardware — but replayed state changes are
  // re-enactment, and re-enactment sends no events
  watch() {
    if (typeof feature_Replay !== 'undefined' && feature_Replay.active) return;
    let s = {};
    try { s = JSON.parse(feature_Loop.state || '{}'); } catch (e) {}
    const t = s.dict_transcribe;
    if (!t || t.rung !== 'local' || this.busy) return;
    this.busy = t.id;
    this.run(t.id, t.grade);
  },

  init() {
    const fm_phoneApply = feature_Loop.apply;
    const self = this;
    feature_Loop.apply = function (p) {
      fm_phoneApply.call(this, p);
      self.watch();
    };
    this.watch();
  },
};
const fm_phoneInit = setInterval(() => {
  if (typeof feature_Loop !== 'undefined' && feature_Loop.state !== null) {
    clearInterval(fm_phoneInit);
    feature_Phone.init();
  }
}, 100);
