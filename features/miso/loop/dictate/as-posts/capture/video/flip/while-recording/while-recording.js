// the recorder never sees the camera change, because it is not recording the
// camera: it is recording a canvas that the current camera is drawn onto.
//
// iOS decides the shape of this. A MediaRecorder ends with an error the moment
// its stream's track set changes, so the obvious move — swap the video track —
// is not available. What IS available is `canvas.captureStream()`, whose track
// is the canvas and never changes whatever is drawn into it. Measured on the
// simulator before it was built (2026-09-04): mp4 out, avc1 + aac, a 5.1 s
// take with the camera swapped at 2.6 s played back whole at its full length.
const feature_WhileRecording = {
  // 30 is what the capture is asked for and what the draw is throttled to.
  // rAF offers 60 and drawing twice for every frame the stream takes is heat
  // for nothing — the probe measured 58.7 fps drawing uncapped.
  FPS: 30,

  canvas: null, ctx: null, src: null, cam: null, out: null,
  extra: [], raf: 0, last: 0, facing: '', drawing: false, switching: false,

  // ---- the stream the recorder is handed -----------------------------------
  // called synchronously by /video's `start`, with `media` open and no frame
  // decoded yet — so the size comes off the track's own settings rather than
  // off a <video> that has not loaded. A camera that reports neither is given
  // 640x480, which is what every camera this app has met returns.
  begin() {
    const media = feature_Video.media;
    if (!media || typeof HTMLCanvasElement === 'undefined'
        || !HTMLCanvasElement.prototype.captureStream) {
      return media;                      // no canvas capture here: record as before
    }
    const track = media.getVideoTracks()[0];
    if (!track) return media;            // audio only: nothing to draw
    let w = 0, h = 0;
    try { const s = track.getSettings() || {}; w = s.width | 0; h = s.height | 0; } catch (e) {}
    if (!w || !h) { w = 640; h = 480; }

    this.canvas = document.createElement('canvas');
    this.canvas.width = w; this.canvas.height = h;
    this.ctx = this.canvas.getContext('2d');
    this.ctx.fillStyle = '#000';
    this.ctx.fillRect(0, 0, w, h);       // a black frame, never an empty one

    // the source the canvas is drawn from. Its own element, not /video's
    // viewfinder: the viewfinder is styled by /square-crop and owned by
    // /video, and a recording must not depend on how a preview is dressed.
    // Two pixels at 1% opacity rather than `display: none`, because iOS stops
    // decoding a video it is not drawing anywhere (proven in the probe).
    this.src = document.createElement('video');
    this.src.id = 'vidSrc';
    this.src.autoplay = true; this.src.muted = true; this.src.defaultMuted = true;
    this.src.setAttribute('muted', ''); this.src.setAttribute('playsinline', '');
    document.body.appendChild(this.src);
    this.src.srcObject = media;
    const p = this.src.play(); if (p && p.catch) p.catch(() => {});

    this.cam = media;
    this.facing = this.wanted();
    this.extra = [];
    this.drawing = true;
    this.last = 0;
    this.draw();

    this.out = this.canvas.captureStream(this.FPS);
    // the microphone is the one thing that must NOT be swapped: it is the same
    // microphone whichever way the camera points, and taking it out of the
    // recorder's track set is the very thing that would end the take. It stays
    // /video's own, from the first getUserMedia, for the whole recording.
    const audio = media.getAudioTracks();
    if (audio.length) this.out.addTrack(audio[0]);
    return this.out;
  },

  // cover, not fit: the frame fills the canvas and the overflow is cropped,
  // centred — so a camera that hands back a different shape than the one the
  // take started on does not letterbox it. /square-crop centre-crops at
  // display anyway, so this is the crop that surface already expects.
  draw() {
    if (!this.drawing) return;
    this.raf = requestAnimationFrame(() => this.draw());
    const now = (typeof performance !== 'undefined') ? performance.now() : Date.now();
    if (now - this.last < 1000 / this.FPS - 1) return;
    this.last = now;
    const v = this.src;
    if (!v || !v.videoWidth || !v.videoHeight) return;
    const cw = this.canvas.width, ch = this.canvas.height;
    const scale = Math.max(cw / v.videoWidth, ch / v.videoHeight);
    const dw = v.videoWidth * scale, dh = v.videoHeight * scale;
    try { this.ctx.drawImage(v, (cw - dw) / 2, (ch - dh) / 2, dw, dh); } catch (e) {}
  },

  // ---- the flip ------------------------------------------------------------
  // no new event and no new button: /armed's camera control already writes the
  // var and is already drawn while a recording runs. What was missing is that
  // nothing acted on it once `start` had its stream. This watches the answer
  // the whole chain gives — `constraints()`, whoever composed it — and swaps
  // the source when it changes.
  wanted() {
    try {
      const c = feature_Video.constraints();
      return (c && c.video && c.video.facingMode) || '';
    } catch (e) { return ''; }
  },

  onApply() {
    if (!this.drawing || this.switching) return;
    const want = this.wanted();
    if (!want || want === this.facing) return;
    this.swap(want);
  },

  // the canvas keeps being drawn throughout, so the recorder sees an unbroken
  // stream of frames across the swap: the take is one file, not two.
  async swap(facing) {
    this.switching = true;
    try {
      // video only. A second audio track would be a second microphone in the
      // room and, worse, a change to a track set that must not change.
      const next = await navigator.mediaDevices.getUserMedia({
        video: { facingMode: facing }, audio: false });
      const old = this.cam;
      this.cam = next;
      this.facing = facing;
      this.src.srcObject = next;
      const p = this.src.play(); if (p && p.catch) p.catch(() => {});
      // the viewfinder follows at once: a flip you cannot see is a flip that
      // lies about which way the phone is pointing.
      if (feature_Video.view) {
        feature_Video.view.srcObject = next;
        const q = feature_Video.view.play(); if (q && q.catch) q.catch(() => {});
      }
      // the camera we came off is released — but /video's own stream keeps its
      // microphone, which the recorder is still holding.
      if (old === feature_Video.media) {
        old.getVideoTracks().forEach((t) => t.stop());
      } else if (old) {
        old.getTracks().forEach((t) => t.stop());
      }
      this.extra.push(next);
    } catch (e) {
      // the camera refused: stay on the one we have and let the button's next
      // tap try again. A half-swapped take is the one outcome worth avoiding.
      this.facing = this.wanted();
    }
    this.switching = false;
  },

  // ---- the teardown ---------------------------------------------------------
  // every stream this node opened, and the canvas track it made. /video's own
  // `stop` releases `media`; nothing else knows about these.
  end() {
    this.drawing = false;
    if (this.raf) cancelAnimationFrame(this.raf);
    this.raf = 0;
    for (const s of this.extra) { try { s.getTracks().forEach((t) => t.stop()); } catch (e) {} }
    this.extra = [];
    if (this.out) { try { this.out.getVideoTracks().forEach((t) => t.stop()); } catch (e) {} }
    this.out = null;
    if (this.src) { this.src.srcObject = null; this.src.remove(); this.src = null; }
    this.canvas = null; this.ctx = null; this.cam = null; this.facing = '';
  },

  install() {
    if (typeof feature_Video === 'undefined' || typeof feature_Loop === 'undefined') return false;
    if (feature_Video.fm_whileRecording) return true;
    feature_Video.fm_whileRecording = true;

    feature_Video.recordStream = function () {
      return feature_WhileRecording.begin();
    };

    const fm_wrStop = feature_Video.stop.bind(feature_Video);
    feature_Video.stop = function () {
      feature_WhileRecording.end();
      return fm_wrStop();
    };

    // the var reaches the page as a repaint, so the paint is where the change
    // is noticed — /video watches the same place for its own recording edges.
    const fm_wrApply = feature_Loop.apply;
    feature_Loop.apply = function (p) {
      fm_wrApply.call(this, p);
      feature_WhileRecording.onApply();
    };
    return true;
  },
};

{
  let fm_wrTries = 0;
  const fm_wrInit = setInterval(() => {
    fm_wrTries = fm_wrTries + 1;
    if (feature_WhileRecording.install() || fm_wrTries > 100) clearInterval(fm_wrInit);
  }, 100);
}
