const feature_Poster = {
  // fallbacks for when /cards is toggled off: the budget is the wire's and
  // belongs to /cards, but a poster must not depend on reading it.
  CAP: 8192,
  EDGE: 256,
  WAIT: 6000,
  opened: {},

  cap() {
    return (typeof feature_Cards !== 'undefined' && feature_Cards.CAP) || this.CAP;
  },

  // WHEN in the clip the face comes from. Random inside the middle half: a
  // clip's first moments are a lens waking up and its last are a hand
  // reaching for stop, and the ask said random is fine. This is the one
  // function a chooser replaces — "pick a better poster" is the next ask
  // (/anticipation), and it changes this and nothing else.
  pick(dur) {
    return dur * (0.25 + Math.random() * 0.5);
  },

  // the whole frame, longest edge to EDGE — /cards' own sizing, and its
  // fallback when /cards is not here. `frameOf` is where a crop lives.
  whole(w, h) {
    const long = Math.max(w, h) || 1;
    const scale = Math.min(1, this.EDGE / long);
    return { sx: 0, sy: 0, sw: w, sh: h,
             dw: Math.max(1, Math.round(w * scale)),
             dh: Math.max(1, Math.round(h * scale)) };
  },

  frame(w, h) {
    if (typeof feature_Cards !== 'undefined' && feature_Cards.frameOf) {
      return feature_Cards.frameOf(w, h);
    }
    return this.whole(w, h);
  },

  // an event, or a timeout. Every wait here is bounded: a decode that never
  // finishes must leave the post alone, not hang a promise inside save().
  once(el, name, ms) {
    return new Promise((res, rej) => {
      let done = false;
      const off = () => { el.removeEventListener(name, hit); clearTimeout(timer); };
      const hit = () => { if (!done) { done = true; off(); res(); } };
      const timer = setTimeout(() => {
        if (!done) { done = true; off(); rej(new Error('slow')); }
      }, ms);
      el.addEventListener(name, hit);
    });
  },

  // the frame off the clip. A MediaRecorder webm often reports Infinity for
  // its duration until it has been seeked once, so a seek past the end comes
  // first and the recording's own measured length is the fallback.
  async grab(blob, est) {
    const url = URL.createObjectURL(blob);
    const v = document.createElement('video');
    try {
      v.muted = true;
      v.preload = 'auto';
      v.setAttribute('playsinline', '');
      v.src = url;
      await this.once(v, 'loadedmetadata', this.WAIT);
      let dur = v.duration;
      if (!isFinite(dur) || dur <= 0) {
        v.currentTime = 1e6;
        try { await this.once(v, 'seeked', this.WAIT); } catch (e) { /* below */ }
        dur = isFinite(v.duration) && v.duration > 0 ? v.duration : est;
      }
      if (!isFinite(dur) || dur <= 0) return null;
      // the clip can be shorter than the moment chosen — clamp, and leave a
      // frame's worth of room before the end so the seek lands on picture.
      const want = Math.max(0, Math.min(this.pick(dur), dur - 0.1));
      v.currentTime = want;
      await this.once(v, 'seeked', this.WAIT);
      return this.draw(v);
    } catch (e) {
      return null;                       // no poster is a whole answer
    } finally {
      v.src = '';
      URL.revokeObjectURL(url);
    }
  },

  // onto a canvas, and down the quality ladder until it fits the wire's
  // budget — /cards' ladder, because the poster travels in the cards list
  // exactly as a picture does. A tainted canvas throws on toDataURL; the
  // catch above owns that too.
  draw(v) {
    const w = v.videoWidth, h = v.videoHeight;
    if (!w || !h) return null;
    const f = this.frame(w, h);
    const cv = document.createElement('canvas');
    cv.width = f.dw; cv.height = f.dh;
    cv.getContext('2d').drawImage(v, f.sx, f.sy, f.sw, f.sh, 0, 0, f.dw, f.dh);
    for (const q of [0.8, 0.65, 0.5, 0.4, 0.3, 0.2]) {
      const d = cv.toDataURL('image/jpeg', q);
      if (d.length <= this.cap()) return d;
    }
    return null;
  },

  // the whole capture: read the clip back out of the store, take a frame,
  // send it at the recording. Nothing here ever speaks to the user — the
  // post is made and the clip is safe whatever happens, and a face that
  // could not be taken is a post without a face, not an error.
  async make(id, est) {
    if (typeof feature_Dictate === 'undefined' || !feature_Dictate.db) return null;
    if (typeof feature_Loop === 'undefined' || !feature_Loop.state) return null;
    let blob = null;
    try { blob = await feature_Dictate.getBlob(id); } catch (e) { return null; }
    if (!blob) return null;
    const data = await this.grab(blob, est);
    if (!data) return null;
    // the budget, /cards' own: '' names no card, so nothing is discounted.
    if (typeof feature_Cards !== 'undefined' && feature_Cards.held
        && feature_Cards.held('', 1) + data.length > feature_Cards.LIST_CAP) {
      return null;
    }
    feature_Loop.send({ type: 'CardPoster',
                        data: { rec: id, data, t: Date.now() } });
    return data;
  },

  // ---- the tap -------------------------------------------------------------
  // the poster becomes the player in place: the holder puts on /capture/video's
  // own class and its mount finds it on the next pass. `playing` is set first,
  // so the <video> starts as soon as it has metadata — one tap, not two.
  open(h) {
    const id = h.getAttribute('data-vid');
    if (!id) return;
    this.opened[id] = true;
    const face = h.querySelector('.poster-frame');
    if (face) face.remove();
    h.classList.remove('post-poster');
    h.classList.add('post-video');
    if (typeof feature_Video === 'undefined') return;
    feature_Video.playing[id] = true;
    feature_Video.mount();
    // inside the tap, not after it: a browser gives sound to a play() that
    // is still in the gesture and refuses one that is not. The blob URL was
    // warmed while the poster was showing, so the mount above is synchronous
    // and there is a <video> here to start.
    const el = h.querySelector('video');
    if (el) { const p = el.play(); if (p && p.catch) p.catch(() => {}); }
  },

  // the clip's blob URL, made while the poster is still showing, in
  // /capture/video's own caches so its mount finds it and puts the player up
  // in one turn. Without this the tap costs a round trip to IndexedDB and the
  // play lands outside the gesture.
  warm(id) {
    if (typeof feature_Video === 'undefined') return;
    if (feature_Video.urls[id] || feature_Video.pending[id]) return;
    if (typeof feature_Dictate === 'undefined' || !feature_Dictate.db) return;
    feature_Video.pending[id] = true;
    feature_Dictate.getBlob(id).then((b) => {
      feature_Video.pending[id] = false;
      if (b) feature_Video.urls[id] = URL.createObjectURL(b);
    }).catch(() => { feature_Video.pending[id] = false; });
  },

  // a render is a whole-DOM swap and the poster comes back with it. Which
  // clips are open is remembered here, the way /capture/video remembers where
  // one had got to — so a repaint mid-play is invisible rather than a poster
  // sliding back over a playing video.
  restore() {
    const holders = document.querySelectorAll('.post-poster[data-vid]');
    for (const h of holders) {
      const id = h.getAttribute('data-vid');
      if (this.opened[id]) this.open(h); else this.warm(id);
    }
  },

  // ---- the hook ------------------------------------------------------------
  // /capture/video's save writes the blob and announces the recording, which
  // is what mints the card; the poster is taken after, when there is a card
  // to put it on. Wrapped rather than edited: the node owns its own act.
  hook() {
    if (typeof feature_Video === 'undefined' || feature_Video.fm_posterWrapped) return;
    feature_Video.fm_posterWrapped = true;
    const orig = feature_Video.save.bind(feature_Video);
    feature_Video.save = async function () {
      const at = feature_Video.startedAt;
      await orig();
      const est = Math.max(1, Math.round((Date.now() - at) / 1000));
      try { await feature_Poster.make('vid-' + at, est); } catch (e) { /* no face */ }
    };
  },

  init() {
    this.hook();
    const fm_posterApply = feature_Loop.apply;
    const self = this;
    feature_Loop.apply = function (p) {
      fm_posterApply.call(this, p);
      self.hook();                  // /capture/video may have booted since
      self.restore();
    };
    document.addEventListener('click', (e) => {
      if (!e.target || !e.target.closest) return;
      const h = e.target.closest('.post-poster[data-vid]');
      if (!h) return;
      self.open(h);
    });
    this.restore();
  },
};
const fm_posterInit = setInterval(() => {
  if (typeof feature_Loop !== 'undefined' && feature_Loop.state !== null) {
    clearInterval(fm_posterInit);
    feature_Poster.init();
  }
}, 100);
