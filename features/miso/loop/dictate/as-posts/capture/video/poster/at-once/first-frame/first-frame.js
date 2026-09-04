// every clip gets a face, whatever its length: the first frame the camera
// decodes is taken before the first tick, and if the camera never gave one the
// canvas is used as it stands. A post without a thumbnail is the thing that
// was ruled out, so this node's whole job is that there is never none.
const feature_FirstFrame = {
  SOON: 30,        // how often the viewfinder is asked whether it has a frame
  WAIT: 4000,      // and for how long before giving up on one
  SIDE: 240,       // the square a frame that never came is drawn at

  poll: null,
  dark: '',        // the recording whose face is a frame the camera never gave
  said: '',        // the recording last answered for, and what was answered
  ref: '',

  // the first frame, the moment there is one. /at-once keeps a frame every
  // 400 ms, so a clip shorter than a tick had none at all — and that is the
  // clip most likely to be made by accident and most in need of a face.
  watch() {
    this.stop();
    const started = Date.now();
    this.poll = setInterval(() => {
      const v = feature_Video.view;
      if (!v || !feature_AtOnce.timer) { this.stop(); return; }   // not filming any more
      if (v.videoWidth && v.videoHeight) {
        this.stop();
        try { feature_AtOnce.grab(); } catch (e) { /* the ticks still come */ }
        return;
      }
      if (Date.now() - started > this.WAIT) this.stop();
    }, this.SOON);
  },

  stop() {
    if (this.poll) clearInterval(this.poll);
    this.poll = null;
  },

  // whatever the canvas holds, for a clip stopped before the camera decoded
  // anything: the app's own dark ground with the viewfinder drawn over it if
  // there is anything to draw. A dark square is a worse picture than a face
  // and a better one than none.
  dim() {
    const cv = document.createElement('canvas');
    cv.width = this.SIDE;
    cv.height = this.SIDE;
    const cx = cv.getContext('2d');
    cx.fillStyle = '#101012';
    cx.fillRect(0, 0, this.SIDE, this.SIDE);
    const v = feature_Video.view;
    if (v) {
      try { cx.drawImage(v, 0, 0, this.SIDE, this.SIDE); } catch (e) { /* nothing there */ }
    }
    cv.videoWidth = this.SIDE;
    cv.videoHeight = this.SIDE;
    let ref = null;
    try { ref = feature_Poster.draw(cv); } catch (e) { ref = null; }
    return ref || '';
  },

  // the slow road's later frame may REPLACE a dark one and must never remove
  // it. /poster's own op writes only into an empty picture block, so the
  // replacement goes by /cards' own CardPic, which sets a block's data and
  // leaves everything else — the poster mark included — as it was.
  replace(id, ref) {
    if (!ref || typeof feature_Loop === 'undefined' || !feature_Loop.state) return;
    let cards = [];
    try { cards = JSON.parse(JSON.parse(feature_Loop.state).cards || '[]'); } catch (e) { return; }
    for (const c of cards) {
      if (!c || c.rec !== id) continue;
      const blocks = Array.isArray(c.blocks) ? c.blocks : [];
      for (let i = 0; i < blocks.length; i++) {
        if (blocks[i] && blocks[i].kind === 'picture') {
          feature_Loop.send({ type: 'CardPic',
                              data: { id: c.id, i: i, data: ref, t: Date.now() } });
          return;
        }
      }
    }
  },
};

{
  if (typeof feature_AtOnce !== 'undefined' && typeof feature_Video !== 'undefined'
      && typeof feature_Poster !== 'undefined') {
    const F = feature_FirstFrame;

    // the first frame is asked for the moment the camera is up
    const fm_ffArm = feature_AtOnce.arm;
    feature_AtOnce.arm = function () {
      fm_ffArm.call(this);
      F.dark = '';
      try { F.watch(); } catch (e) { /* the ticks are still the road */ }
    };

    const fm_ffDisarm = feature_AtOnce.disarm;
    feature_AtOnce.disarm = function () {
      F.stop();
      return fm_ffDisarm.call(this);
    };

    // and a face goes out with every recording, without exception.
    //
    // The answer is remembered per recording and given again unchanged: the
    // parent empties its slots as it answers, so a second ask for the same
    // recording would find none and mint a dark face for a clip that already
    // had a real one. Nothing in the tree asks twice today, but the seam this
    // sits on is one a sibling has already assigned once (/streams), and the
    // cost of being asked twice should be nothing rather than a wasted picture
    // on the upload queue.
    const fm_ffFrame = feature_AtOnce.frameFor;
    feature_AtOnce.frameFor = function (id) {
      if (id && id === F.said) return F.ref;
      const ref = fm_ffFrame.call(this, id);
      let out = ref;
      if (!out && id && id === this.forId) {
        out = F.dim();
        if (out) F.dark = id;
      }
      if (id) { F.said = id; F.ref = out; }
      return out;
    };

    // a dark face does not stand the slow road down — it is the one face this
    // node would rather see replaced...
    const fm_ffAlready = feature_AtOnce.already;
    feature_AtOnce.already = function (id) {
      if (id && id === F.dark) return false;
      return fm_ffAlready.call(this, id);
    };

    // ...and what that road comes back with is written over it
    const fm_ffMake = feature_Poster.make;
    feature_Poster.make = function (id, est) {
      const out = fm_ffMake.call(this, id, est);
      if (id && id === F.dark && out && out.then) {
        out.then((ref) => {
          if (!ref || id !== F.dark) return;
          F.dark = '';
          F.replace(id, ref);
        }).catch(() => { /* the dark face stands */ });
      }
      return out;
    };
  }
}
