// the face is taken off the viewfinder while you are filming, so the post is
// minted with its picture already on it. Nothing waits for the clip to be read
// back, decoded and seeked.
const feature_AtOnce = {
  EVERY: 400,      // a frame kept this often while the camera runs
  OLD: 400,        // and the one used is at least this old at the stop

  timer: null,
  slots: [],       // {canvas, at} — two, so the newest can be passed over
  forId: '',       // the recording these frames belong to

  // the frames are kept as canvases and only ONE is ever turned into a
  // picture: /pic-beside's mint writes bytes to the device's store and puts
  // the id on the upload queue, so minting on every tick would leave a minute's
  // filming as a hundred and fifty stored pictures and a hundred and fifty
  // uploads for a post that shows one.
  grab() {
    const v = feature_Video.view;
    if (!v || !v.videoWidth || !v.videoHeight) return;
    const cv = document.createElement('canvas');
    cv.width = v.videoWidth;
    cv.height = v.videoHeight;
    try {
      cv.getContext('2d').drawImage(v, 0, 0);
    } catch (e) { return; }          // the stream is gone: keep what we have
    // /poster's draw reads these two off what it is given, and a canvas is a
    // drawable source like a video: naming them here is what lets the frame go
    // through /poster's own framing and quality ladder at the stop.
    cv.videoWidth = cv.width;
    cv.videoHeight = cv.height;
    this.slots.push({ canvas: cv, at: Date.now() });
    while (this.slots.length > 2) this.slots.shift();
  },

  arm() {
    this.disarm();
    this.slots = [];
    this.forId = 'vid-' + feature_Video.startedAt;
    this.timer = setInterval(() => { try { this.grab(); } catch (e) { this.disarm(); } },
                             this.EVERY);
  },

  disarm() {
    if (this.timer) clearInterval(this.timer);
    this.timer = null;
  },

  // the frame for this recording, made into a picture once. The newest is
  // passed over when it is younger than OLD: the last half second of a clip is
  // a hand reaching for the stop button, which is the moment /poster's own
  // chooser exists to avoid.
  frameFor(id) {
    if (!id || id !== this.forId || !this.slots.length) return '';
    const newest = this.slots[this.slots.length - 1];
    const pick = (this.slots.length > 1 && Date.now() - newest.at < this.OLD)
      ? this.slots[this.slots.length - 2] : newest;
    this.slots = [];
    let ref = null;
    try { ref = feature_Poster.draw(pick.canvas); } catch (e) { ref = null; }
    return ref || '';
  },

  mine: null,      // this node's metaFor, so a sibling's can be found under it

  // /streams ASSIGNS feature_Video.metaFor rather than wrapping it, from an
  // install() that runs when /video has booted — after this node's own load —
  // so a wrapper put on at load is simply gone by the time a recording is
  // saved (measured: the face never reached the metadata, and /poster's slow
  // road wrote it seconds later as before). The seam is taken again whenever
  // it is not ours, which is /poster's own hook() idiom for the same reason.
  hook() {
    if (typeof feature_Video === 'undefined' || feature_Video.metaFor === this.mine) return;
    const inner = feature_Video.metaFor;
    const self = this;
    this.mine = function (meta) {
      const m = inner.call(this, meta);
      let ref = '';
      try { ref = self.frameFor(m && m.id); } catch (e) { ref = ''; }
      if (!ref) return m;
      self.gave = m.id;
      return Object.assign({}, m, { poster: ref });
    };
    feature_Video.metaFor = this.mine;
  },

  gave: '',        // the recording this node has already handed a face to

  // has this recording got its face already? Then the slow road has nothing to
  // add: its op would find the block filled and write nothing, and the frame
  // it decoded would be bytes on the upload queue that no card names.
  //
  // The first test is what THIS node handed over, not what the world holds:
  // /poster asks this the moment the recording is saved, and the card is not
  // there yet at that moment (measured — `cardFound: false` every time), so a
  // test against the cards alone always said no and the slow road always ran.
  // The world is still read after it, for a face that came by another road.
  already(id) {
    if (id && id === this.gave) return true;
    if (!id || typeof feature_Loop === 'undefined' || !feature_Loop.state) return false;
    let cards = [];
    try { cards = JSON.parse(JSON.parse(feature_Loop.state).cards || '[]'); } catch (e) { return false; }
    for (const c of cards) {
      if (!c || c.rec !== id) continue;
      for (const b of (Array.isArray(c.blocks) ? c.blocks : [])) {
        if (b && b.kind === 'picture' && b.poster && b.data) return true;
      }
    }
    return false;
  },
};

{
  if (typeof feature_Video !== 'undefined' && typeof feature_Poster !== 'undefined') {
    const A = feature_AtOnce;

    // the camera is up: start keeping frames
    const fm_atOnceView = feature_Video.viewfinder;
    feature_Video.viewfinder = function () {
      fm_atOnceView.call(this);
      try { A.arm(); } catch (e) { /* no frames, and the slow road as before */ }
    };

    // and stop keeping them BEFORE the tracks are stopped, so the last frame
    // kept is one the camera was still making
    const fm_atOnceStop = feature_Video.stop;
    feature_Video.stop = function () {
      try { A.disarm(); } catch (e) { /* the interval is harmless either way */ }
      return fm_atOnceStop.call(this);
    };

    // the recording's metadata carries the face, so /as-posts mints the card
    // with its picture already in place — the same turn, the same paint. The
    // seam is taken again after every turn: a sibling may assign it later.
    A.hook();
    if (typeof feature_Loop !== 'undefined') {
      const fm_atOnceApply = feature_Loop.apply;
      feature_Loop.apply = function (p) {
        fm_atOnceApply.call(this, p);
        try { A.hook(); } catch (e) { /* the slow road as before */ }
      };
    }

    // and the slow road stands down when the face is already there
    const fm_atOnceMake = feature_Poster.make;
    feature_Poster.make = function (id, est) {
      if (A.already(id)) return Promise.resolve(null);
      return fm_atOnceMake.call(this, id, est);
    };
  }
}
