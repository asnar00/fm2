const feature_Frame = {
  // the held original: /cards' own chose, everything after the framing —
  // budget, quality ladder, toast, CardPic send — stays /cards'.
  onward: null,

  sheet: null,
  win: null,
  cv: null,
  img: null,
  // the square region of the SOURCE photo the window is showing: a centre in
  // image pixels and a scale (window css px per image px)
  scale: 1,
  min: 1,
  cx: 0,
  cy: 0,
  // how far past the floor a photo may be pushed. Twelve is far enough to
  // pick an eye out of a phone photograph and near enough that a fingertip
  // still moves something visible.
  MAX: 12,

  // the window's edge in css px, asked at open time so a rotated phone gets
  // the square it deserves
  side() {
    return Math.max(80, Math.round(Math.min(window.innerWidth * 0.9,
                                            window.innerHeight * 0.6)));
  },

  // THE geometry, in one place: the visible square in image pixels, with the
  // centre clamped so the window can never show anything but photograph.
  region() {
    const w = this.side();
    const size = w / this.scale;
    const half = size / 2;
    const cx = Math.min(Math.max(this.cx, half), this.img.width - half);
    const cy = Math.min(Math.max(this.cy, half), this.img.height - half);
    this.cx = cx;
    this.cy = cy;
    return { x: cx - half, y: cy - half, size, w };
  },

  draw() {
    if (!this.img || !this.cv) return;
    const r = this.region();
    const dpr = window.devicePixelRatio || 1;
    if (this.cv.width !== Math.round(r.w * dpr)) {
      this.cv.width = Math.round(r.w * dpr);
      this.cv.height = Math.round(r.w * dpr);
    }
    const g = this.cv.getContext('2d');
    g.setTransform(dpr, 0, 0, dpr, 0, 0);
    g.clearRect(0, 0, r.w, r.w);
    g.imageSmoothingQuality = 'high';
    g.drawImage(this.img, r.x, r.y, r.size, r.size, 0, 0, r.w, r.w);
  },

  // the two gestures, reduced to numbers: touch, mouse and wheel all land here
  zoom(factor) {
    if (!this.img) return;
    this.scale = Math.min(Math.max(this.scale * factor, this.min),
                          this.min * this.MAX);
    this.draw();
  },

  pan(dx, dy) {
    if (!this.img) return;
    this.cx -= dx / this.scale;
    this.cy -= dy / this.scale;
    this.draw();
  },

  // the floor is "the photo fills the square": the shorter side exactly
  // covers the window, so a portrait can be slid up and down and a landscape
  // left and right, and neither can be slid off.
  open(img) {
    this.img = img;
    const w = this.side();
    if (this.win) {
      this.win.style.width = w + 'px';
      this.win.style.height = w + 'px';
    }
    this.min = w / Math.max(1, Math.min(img.width, img.height));
    this.scale = this.min;
    this.cx = img.width / 2;
    this.cy = img.height / 2;
    if (this.sheet) this.sheet.classList.add('show');
    this.draw();
  },

  close() {
    if (this.sheet) this.sheet.classList.remove('show');
    this.img = null;
  },

  // what is kept is drawn from the SOURCE image, not from the preview: the
  // preview is display resolution and would throw detail away. /cards' EDGE
  // is read at use, so /roomier owns the number exactly as it does for shrink.
  keep() {
    if (!this.img) return;
    const edge = (typeof feature_Cards !== 'undefined' && feature_Cards.EDGE)
      ? feature_Cards.EDGE : 256;
    const r = this.region();
    const img = this.img;
    const out = document.createElement('canvas');
    out.width = edge;
    out.height = edge;
    const g = out.getContext('2d');
    g.imageSmoothingQuality = 'high';
    g.drawImage(img, r.x, r.y, r.size, r.size, 0, 0, edge, edge);
    this.close();
    const onward = this.onward;
    // a square already at EDGE: /cards' shrink scales by 1 and only its
    // quality ladder does any work, which is why a Blob is handed over
    // rather than a data URL encoded here.
    out.toBlob((blob) => {
      if (blob && onward && typeof feature_Cards !== 'undefined')
        onward.call(feature_Cards, blob);
    }, 'image/png');
  },
};

if (typeof feature_Cards !== 'undefined') {
  // the seam, taken by redefinition and kept in a closure — me.js takes
  // /account's openTool the same way. The linker's fragment gate wraps this
  // assignment, so unticking the node hands chose back at runtime too.
  const fm_frameChose = feature_Cards.chose;
  feature_Frame.onward = fm_frameChose;

  feature_Cards.chose = function (file) {
    if (!file || !this.target) return fm_frameChose.call(this, file);
    const url = URL.createObjectURL(file);
    const im = new Image();
    im.onload = () => {
      URL.revokeObjectURL(url);
      feature_Frame.open(im);
    };
    // not a picture: hand the file straight on, so /cards raises its own
    // "that file is not a picture" toast. One voice, and it was there first.
    im.onerror = () => {
      URL.revokeObjectURL(url);
      fm_frameChose.call(feature_Cards, file);
    };
    im.src = url;
  };

  {
    // furniture made at load and living OUTSIDE #app, so a repaint of the
    // loop's html while the sheet is open cannot take it away — the
    // #cardToast precedent in cards.js.
    const fm_frameSheet = document.createElement('div');
    fm_frameSheet.id = 'frameSheet';

    const fm_frameWin = document.createElement('div');
    fm_frameWin.id = 'frameWindow';
    const fm_frameCv = document.createElement('canvas');
    fm_frameCv.id = 'frameCanvas';
    fm_frameWin.appendChild(fm_frameCv);
    fm_frameSheet.appendChild(fm_frameWin);

    const fm_frameBar = document.createElement('div');
    fm_frameBar.id = 'frameBar';
    const fm_frameCancel = document.createElement('button');
    fm_frameCancel.id = 'frameCancel';
    fm_frameCancel.type = 'button';
    fm_frameCancel.textContent = 'cancel';
    const fm_frameKeep = document.createElement('button');
    fm_frameKeep.id = 'frameKeep';
    fm_frameKeep.type = 'button';
    fm_frameKeep.textContent = 'keep';
    fm_frameBar.appendChild(fm_frameCancel);
    fm_frameBar.appendChild(fm_frameKeep);
    fm_frameSheet.appendChild(fm_frameBar);

    document.body.appendChild(fm_frameSheet);
    feature_Frame.sheet = fm_frameSheet;
    feature_Frame.win = fm_frameWin;
    feature_Frame.cv = fm_frameCv;

    // no data-ev on either button, so the loop's own delegated click never
    // fires for them — the rule cards.js follows for .card-pic.
    fm_frameCancel.addEventListener('click', (e) => {
      e.preventDefault();
      feature_Frame.close();
    });
    fm_frameKeep.addEventListener('click', (e) => {
      e.preventDefault();
      feature_Frame.keep();
    });

    // one finger pans, two fingers pinch. Non-passive and prevented, so
    // framing does not scroll the page underneath it.
    let fm_frameTouch = null;
    const fm_frameSpread = (t) => Math.hypot(t[0].clientX - t[1].clientX,
                                             t[0].clientY - t[1].clientY);
    const fm_frameMid = (t) => (t.length > 1
      ? { x: (t[0].clientX + t[1].clientX) / 2, y: (t[0].clientY + t[1].clientY) / 2 }
      : { x: t[0].clientX, y: t[0].clientY });

    fm_frameWin.addEventListener('touchstart', (e) => {
      if (!feature_Frame.img) return;
      e.preventDefault();
      const t = e.touches;
      fm_frameTouch = { at: fm_frameMid(t),
                        spread: t.length > 1 ? fm_frameSpread(t) : 0 };
    }, { passive: false });

    fm_frameWin.addEventListener('touchmove', (e) => {
      if (!feature_Frame.img || !fm_frameTouch) return;
      e.preventDefault();
      const t = e.touches;
      const at = fm_frameMid(t);
      if (t.length > 1) {
        const spread = fm_frameSpread(t);
        if (fm_frameTouch.spread > 0 && spread > 0)
          feature_Frame.zoom(spread / fm_frameTouch.spread);
        fm_frameTouch.spread = spread;
      }
      feature_Frame.pan(at.x - fm_frameTouch.at.x, at.y - fm_frameTouch.at.y);
      fm_frameTouch.at = at;
    }, { passive: false });

    const fm_frameLet = (e) => {
      if (!fm_frameTouch) return;
      e.preventDefault();
      fm_frameTouch = e.touches && e.touches.length
        ? { at: fm_frameMid(e.touches),
            spread: e.touches.length > 1 ? fm_frameSpread(e.touches) : 0 }
        : null;
    };
    fm_frameWin.addEventListener('touchend', fm_frameLet, { passive: false });
    fm_frameWin.addEventListener('touchcancel', fm_frameLet, { passive: false });

    // the same two gestures for a mouse, so a desktop rig can drive it
    let fm_frameDrag = null;
    fm_frameWin.addEventListener('mousedown', (e) => {
      if (!feature_Frame.img) return;
      e.preventDefault();
      fm_frameDrag = { x: e.clientX, y: e.clientY };
    });
    document.addEventListener('mousemove', (e) => {
      if (!fm_frameDrag || !feature_Frame.img) return;
      feature_Frame.pan(e.clientX - fm_frameDrag.x, e.clientY - fm_frameDrag.y);
      fm_frameDrag = { x: e.clientX, y: e.clientY };
    });
    document.addEventListener('mouseup', () => { fm_frameDrag = null; });
    fm_frameWin.addEventListener('wheel', (e) => {
      if (!feature_Frame.img) return;
      e.preventDefault();
      feature_Frame.zoom(Math.exp(-e.deltaY * 0.002));
    }, { passive: false });
  }
}
