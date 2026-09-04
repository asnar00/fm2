const feature_Video = {
  media: null, recorder: null, arec: null, chunks: [], achunks: [],
  startedAt: 0, active: false, cap: 0, view: null,
  urls: {}, pending: {}, at: {}, playing: {},

  // a minute, and no more. The blob travels whole to the exchange through
  // /mirror's route, and the serve layer reads a body of up to 16MB: at the
  // bitrate below a minute is about 8MB, so the cap is what keeps a recording
  // from becoming one that can never be handed on.
  LIMIT: 60000,

  opts() {
    const want = ['video/mp4', 'video/webm;codecs=vp8,opus', 'video/webm'];
    for (const m of want) {
      if (typeof MediaRecorder !== 'undefined' && MediaRecorder.isTypeSupported
          && MediaRecorder.isTypeSupported(m)) {
        return { mimeType: m, videoBitsPerSecond: 1000000, audioBitsPerSecond: 64000 };
      }
    }
    return { videoBitsPerSecond: 1000000 };
  },

  // what the camera is asked for — the seam a later node redefines to ask for
  // a different one. The answer here is the one this node always asked for.
  constraints() {
    return { video: { facingMode: 'environment' }, audio: true };
  },

  // ---- three seams, all answering "as before" ------------------------------
  // opened for /streams, which sends the clip up while it is being made. The
  // answers here are exactly what this node did without them: no timeslice
  // (one blob at stop), nothing done per chunk, and the metadata unchanged.

  // undefined is what MediaRecorder.start() means by "no timeslice", so the
  // recorder behaves as it always did.
  timeslice() {
    return undefined;
  },

  onChunk(blob, n) {
    const _ = [blob, n];
  },

  metaFor(meta) {
    return meta;
  },

  // a companion audio-only recording, kept beside the video and never listed
  // or uploaded: whisper eats audio, and pulling the audio track back out of
  // a recorded video container is not something every browser will do. This
  // way the transcript never depends on that question. A seam, because the
  // whole point of it is /phone's on-device model — retire that and this
  // recording is a second encode of every note for nobody.
  companionAudio() {
    if (!this.media) return;
    const tracks = this.media.getAudioTracks();
    if (!tracks.length) return;
    try {
      this.arec = new MediaRecorder(new MediaStream([tracks[0]]),
                                    { audioBitsPerSecond: 64000 });
      this.arec.ondataavailable = (e) => { if (e.data.size) this.achunks.push(e.data); };
      this.arec.start();
    } catch (e) { this.arec = null; }
  },

  async start() {
    try {
      this.media = await navigator.mediaDevices.getUserMedia(this.constraints());
    } catch (e) {
      feature_Loop.send({ type: 'click', ev: 'vid_stop' });  // state must not lie
      if (typeof feature_Cards !== 'undefined') feature_Cards.say('no camera here');
      return;
    }
    this.chunks = []; this.achunks = []; this.startedAt = Date.now();
    this.recorder = new MediaRecorder(this.media, this.opts());
    this.recorder.ondataavailable = (e) => {
      if (!e.data.size) return;
      this.chunks.push(e.data);
      this.onChunk(e.data, this.chunks.length - 1);
    };
    this.recorder.onstop = () => this.save();
    this.recorder.start(this.timeslice());
    this.companionAudio();
    this.viewfinder();
    this.cap = setTimeout(() => feature_Loop.send({ type: 'click', ev: 'vid_stop' }),
                          this.LIMIT);
  },

  stop() {
    clearTimeout(this.cap);
    if (this.recorder && this.recorder.state !== 'inactive') this.recorder.stop();
    if (this.arec && this.arec.state !== 'inactive') this.arec.stop();
    if (this.media) { this.media.getTracks().forEach((t) => t.stop()); this.media = null; }
    this.hide();
  },

  async save() {
    if (!this.chunks.length || typeof feature_Dictate === 'undefined' || !feature_Dictate.db) return;
    const blob = new Blob(this.chunks, { type: this.recorder.mimeType });
    const id = 'vid-' + this.startedAt;
    const t = new Date(this.startedAt);
    const meta = this.metaFor({
      id, t: this.startedAt, here: true, kind: 'video',
      dur: Math.round((Date.now() - this.startedAt) / 1000),
      size: blob.size, mime: this.recorder.mimeType,
      label: t.getHours() + ':' + String(t.getMinutes()).padStart(2, '0'),
    });
    // the disk can be full, and a clip that cannot be stored must not become
    // a post pointing at nothing: the meta is written LAST, and only if the
    // bytes are down. A failure says so and leaves no wreckage — the words
    // are /cards' own voice, the one this app uses for "no room".
    try {
      await feature_Dictate.put(id, blob);
      if (this.achunks.length) {
        // 'a:' — never a legal blob id on the exchange, so nothing can
        // mistake this for something to upload or fetch
        await feature_Dictate.put('a:' + id,
          new Blob(this.achunks, { type: this.arec ? this.arec.mimeType : 'audio/webm' }));
      }
      await feature_Dictate.put('meta:' + id, meta);
    } catch (e) {
      if (typeof feature_Cards !== 'undefined') feature_Cards.say('no room for that video');
      return;
    }
    feature_Loop.send({ type: 'RecSaved', data: meta });
  },

  // ---- transcription: hand whisper the companion audio ---------------------
  // /phone decodes a recording to 16kHz mono PCM before the engine sees it.
  // For a video it gets the audio that was recorded beside it; with no
  // companion (an old file, a device with no microphone) the original path
  // still runs, and a failure there is stamped and moved past, never looped.
  installPcm() {
    if (typeof feature_Phone === 'undefined'
        || typeof feature_Phone.pcm16k !== 'function' || feature_Phone.fm_vidWrapped) return;
    const orig = feature_Phone.pcm16k.bind(feature_Phone);
    feature_Phone.fm_vidWrapped = true;
    feature_Phone.pcm16k = async (blob) => {
      if (blob && /^video\//.test(blob.type || '')) {
        const id = feature_Phone.busy || '';
        const a = id ? await feature_Dictate.getBlob('a:' + id) : null;
        if (a) return orig(a);
      }
      return orig(blob);
    };
  },

  // ---- the viewfinder ------------------------------------------------------
  // you cannot film a door you cannot see. The preview lives OUTSIDE #app, so
  // a repaint never takes it away mid-recording, and it clears the toolbar.
  viewfinder() {
    if (!this.view) {
      const v = document.createElement('video');
      v.id = 'vidView';
      v.autoplay = true; v.muted = true; v.defaultMuted = true;
      v.setAttribute('muted', ''); v.setAttribute('playsinline', '');
      document.body.appendChild(v);
      this.view = v;
    }
    this.view.srcObject = this.media;
    this.view.classList.add('show');
    const p = this.view.play();
    if (p && p.catch) p.catch(() => {});
  },
  hide() {
    if (!this.view) return;
    this.view.classList.remove('show');
    this.view.srcObject = null;
  },

  // ---- the player on the page ----------------------------------------------
  // a render is a whole-DOM swap, so the <video> is put back after every one.
  // Where it had got to and whether it was playing are remembered here, not
  // in the world — a repaint mid-clip is invisible rather than a restart.
  mount() {
    if (typeof feature_Dictate === 'undefined' || !feature_Dictate.db) return;
    const holders = document.querySelectorAll('.post-video[data-vid]');
    for (const h of holders) {
      if (h.querySelector('video')) continue;
      const id = h.getAttribute('data-vid');
      if (this.urls[id]) { this.put(h, id); continue; }
      if (this.pending[id]) continue;
      this.pending[id] = true;
      feature_Dictate.getBlob(id).then((blob) => {
        this.pending[id] = false;
        if (!blob) return;                      // not here yet; the next render asks again
        this.urls[id] = URL.createObjectURL(blob);
        const now = document.querySelector('.post-video[data-vid="' + id + '"]');
        if (now && !now.querySelector('video')) this.put(now, id);
      }).catch(() => { this.pending[id] = false; });
    }
  },

  put(holder, id) {
    const el = document.createElement('video');
    el.src = this.urls[id];
    el.controls = true; el.preload = 'metadata';
    el.setAttribute('playsinline', '');
    const was = this.at[id] || 0;
    el.addEventListener('loadedmetadata', () => {
      if (was && was < el.duration) el.currentTime = was;
      if (this.playing[id]) { const p = el.play(); if (p && p.catch) p.catch(() => {}); }
    });
    el.addEventListener('timeupdate', () => { this.at[id] = el.currentTime; });
    el.addEventListener('play', () => { this.playing[id] = true; });
    el.addEventListener('pause', () => { this.playing[id] = false; });
    el.addEventListener('ended', () => { this.playing[id] = false; this.at[id] = 0; });
    holder.insertBefore(el, holder.firstChild);
  },

  // state-driven effects: the camera follows state's edges. During a /replay
  // the state changes are re-enactment, not intent — no hardware.
  watch() {
    if (typeof feature_Replay !== 'undefined' && feature_Replay.active) return;
    let s = {};
    try { s = JSON.parse(feature_Loop.state || '{}'); } catch (e) {}
    const wanted = !!s.vid_recording;
    if (wanted && !this.active) { this.active = true; this.start(); }
    if (!wanted && this.active) { this.active = false; this.stop(); }
    this.mount();
  },

  init() {
    this.installPcm();
    const fm_vidApply = feature_Loop.apply;
    const self = this;
    feature_Loop.apply = function (p) {
      fm_vidApply.call(this, p);
      self.installPcm();          // /phone may have booted since the last one
      self.watch();
    };
    this.watch();
  },
};
const fm_vidInit = setInterval(() => {
  if (typeof feature_Loop !== 'undefined' && feature_Loop.state !== null
      && typeof feature_Dictate !== 'undefined' && feature_Dictate.db) {
    clearInterval(fm_vidInit);
    feature_Video.init();
  }
}, 100);
