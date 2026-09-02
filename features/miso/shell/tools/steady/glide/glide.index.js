const feature_Glide = {
  MS: 220,             // one glide, from the old rectangle to the new
  EASE: 'ease-out',    // the app's easing (/taste 5: nothing bouncy)
  level: undefined,    // open_tool as of the previous paint; undefined = none seen
  moveUntil: 0,        // performance.now() at which the running level glide is over
  fadeUntil: 0,        // ...and the running fade, which a level need not have changed for
  ghosts: [],          // the departing clones currently fading, oldest first

  now() {
    return (window.performance && performance.now) ? performance.now() : Date.now();
  },

  // this render's level. The level a row belongs to IS `open_tool` — the
  // launcher is the empty string — which is /steady's own test, asked one
  // link earlier: `apply` sets `feature_Loop.state` before it calls `paint`,
  // so the value read here is already the level about to be drawn.
  openTool() {
    try {
      const s = JSON.parse(feature_Loop.state || '{}');
      return typeof s.open_tool === 'string' ? s.open_tool : '';
    } catch (e) {
      return '';
    }
  },

  // a button's identity across a repaint: the event it sends, or the control
  // it names. A button with neither cannot be recognised on the other side of
  // the swap, so it is left out of the glide entirely — never guessed at,
  // never faded.
  key(el) {
    return el.getAttribute('data-ev') || el.getAttribute('data-ctl') || '';
  },

  reduced() {
    try {
      return !!(window.matchMedia &&
                window.matchMedia('(prefers-reduced-motion: reduce)').matches);
    } catch (e) {
      return false;
    }
  },

  // where every identified button is, right now, in viewport coordinates —
  // the toolbar is `position: fixed`, so a fixed ghost placed at these numbers
  // sits exactly where the button was. Read of a button that is mid-motion this
  // gives the place it has travelled to and how far its fade has got, which is
  // what a continuation needs; the opacity is only read while something is
  // actually running, so a still row costs nothing but rectangles.
  snap() {
    const now = this.now();
    const busy = now < this.moveUntil || now < this.fadeUntil;
    const was = new Map();
    for (const b of document.querySelectorAll('.toolbar .tool-button')) {
      const k = this.key(b);
      if (!k || was.has(k)) continue;
      let o = 1;
      if (busy) {
        const v = parseFloat(getComputedStyle(b).opacity);
        if (v === v) o = v;
      }
      was.set(k, { el: b, r: b.getBoundingClientRect(), o: o });
    }
    return was;
  },

  // a fresh level change owns the screen: whatever was still fading from the
  // last one is taken away now rather than left to finish over the new row.
  clearGhosts() {
    for (const g of this.ghosts.splice(0)) g.remove();
  },

  // hold the inline state only as long as the motion lasts. The element is
  // replaced by the next paint anyway; this is for the paint that does not
  // come. `animation` is deliberately NOT cleared — restoring it would let the
  // stylesheet's mount slide start over, on a button that has just arrived.
  settle(b, props, ms) {
    let done = false;
    const end = () => {
      if (done) return;
      done = true;
      for (const p of props) b.style[p] = '';
      b.style.transition = '';
      b.classList.remove('fm-glide-move', 'fm-glide-in');
    };
    b.addEventListener('transitionend', end, { once: true });
    setTimeout(end, ms + 80);
  },

  // one button's whole journey: back to where it was (`dx`/`dy`) and as faint
  // as it was (`o`), then forward to its place at full strength over `ms`.
  // Position and opacity ride one transition, so a button that both moves and
  // arrives does not have its transition overwritten halfway through being set
  // up. Its mount slide is already cancelled — `run` does that to the whole row
  // before it measures anything.
  travel(b, dx, dy, o, ms) {
    const props = [];
    b.style.transition = 'none';
    if (Math.abs(dx) > 0.5 || Math.abs(dy) > 0.5) {
      b.style.transform = 'translate(' + dx + 'px, ' + dy + 'px)';
      props.push('transform');
    }
    if (o < 0.99) {
      b.style.opacity = String(o);
      props.push('opacity');
    }
    if (!props.length) {
      b.style.transition = '';
      return;
    }
    void b.offsetWidth;                       // the old place is now a real frame
    if (props.indexOf('transform') >= 0) b.classList.add('fm-glide-move', 'fm-glide-moved');
    if (props.indexOf('opacity') >= 0) b.classList.add('fm-glide-in');
    b.style.transition = props.map(function (p) {
      return p + ' ' + ms + 'ms ' + feature_Glide.EASE;
    }).join(', ');
    if (props.indexOf('transform') >= 0) b.style.transform = 'translate(0px, 0px)';
    if (props.indexOf('opacity') >= 0) b.style.opacity = '1';
    this.settle(b, props, ms);
  },

  // a button that is gone: a clone of it fades where it stood. The clone is
  // inert — `pointer-events: none` from the stylesheet, and its `data-ev` is
  // taken off, so /loop's one delegated listener can never be reached through
  // a ghost even if something manages to click one.
  //
  // `position` is set inline, not left to the stylesheet: `/dictate` styles
  // `.tool-button.ctrl { position: relative }`, which outranks a one-class
  // rule of ours and made the first ghosts appear hundreds of pixels from
  // where the button had been (found in the rig, 2026-09-02). Inline beats
  // every author rule, so the ghost stands where it is told.
  leave(old, ms) {
    const g = old.el.cloneNode(true);
    g.removeAttribute('data-ev');
    g.removeAttribute('data-ctl');
    g.className = old.el.className + ' fm-glide-ghost';
    g.style.position = 'fixed';
    g.style.margin = '0';
    g.style.left = old.r.left + 'px';
    g.style.top = old.r.top + 'px';
    g.style.width = old.r.width + 'px';
    g.style.height = old.r.height + 'px';
    g.style.animation = 'none';
    g.style.transition = 'none';
    g.style.opacity = String(old.o);
    document.body.appendChild(g);
    void g.offsetWidth;
    g.style.transition = 'opacity ' + ms + 'ms ' + this.EASE;
    g.style.opacity = '0';
    this.ghosts.push(g);
    const self = this;
    setTimeout(function () {
      const i = self.ghosts.indexOf(g);
      if (i >= 0) self.ghosts.splice(i, 1);
      g.remove();
    }, ms + 80);
  },

  // the whole of the work, with the freshly painted row in front of it and the
  // old row's rectangles in hand.
  run(was) {
    const now = this.now();
    const level = this.openTool();
    const climbed = this.level !== undefined && level !== this.level;
    const moving = now < this.moveUntil;      // a level glide from an earlier paint
    const fading = now < this.fadeUntil;      // a fade from an earlier paint
    this.level = level;

    const bar = document.querySelector('.toolbar');
    // no row drawn at all (the gate withholds it, the veil is down): the
    // buttons did not go anywhere, so nothing is ghosted.
    if (!bar) {
      this.moveUntil = 0;
      this.fadeUntil = 0;
      this.clearGhosts();
      return;
    }
    // nothing to glide FROM — boot, or the first paint after /world-cache has
    // held the seam shut: the stylesheet's mount slide is the right animation
    // for a row arriving out of nothing, and it is left to play.
    if (!was.size) return;

    const fresh = [];
    for (const b of bar.querySelectorAll('.tool-button')) {
      const k = this.key(b);
      if (k) fresh.push([b, k]);
    }
    const here = new Set();
    let arrivals = 0;
    for (const pair of fresh) {
      here.add(pair[1]);
      if (!was.has(pair[1])) arrivals++;
    }
    let departures = 0;
    for (const k of was.keys()) if (!here.has(k)) departures++;

    // whether buttons may be repositioned at all. /steady's rule, kept
    // exactly: a paint that stays on one level never moves a button — only
    // a level change does, and only for the 220 ms that change lasts.
    const move = climbed || moving;
    // a paint with nothing arriving, nothing leaving, nothing still in
    // flight and no change of level is left completely alone.
    if (!move && !fading && !arrivals && !departures) return;

    // the mount slide has to go before anything is measured. `bar-slide`'s
    // first frame translates every fresh button 14px left, and a rectangle
    // read through it is 14px wrong — a glide that starts in the wrong place
    // (found in the rig, 2026-09-02). A CSS animation also outranks an inline
    // `transform` in the cascade, so this is also what makes the FLIP visible.
    for (const pair of fresh) pair[0].style.animation = 'none';

    if (climbed) {
      this.clearGhosts();
      this.moveUntil = now + this.MS;
    }
    // a continuation finishes the motion it joined rather than starting a new
    // one, so three paints 3 ms apart are one 220 ms journey, not three.
    const moveMs = climbed ? this.MS : Math.max(1, Math.round(this.moveUntil - now));
    const opens = climbed || arrivals || departures;   // a fade begins here
    if (opens) this.fadeUntil = now + this.MS;
    const fadeMs = opens ? this.MS : Math.max(1, Math.round(this.fadeUntil - now));

    for (const pair of fresh) {
      const b = pair[0], k = pair[1];
      const old = was.get(k);
      if (old) {
        was.delete(k);                        // a second button on one key is new
        let dx = 0, dy = 0;
        if (move) {
          const r = b.getBoundingClientRect();
          dx = old.r.left - r.left;
          dy = old.r.top - r.top;
        }
        this.travel(b, dx, dy, old.o, move ? moveMs : fadeMs);
      } else {
        this.travel(b, 0, 0, 0, fadeMs);      // an arrival: fade in where it lands
      }
    }
    for (const old of was.values()) this.leave(old, fadeMs);
  },
};

{
  // /loop's paint seam, taken by replacing the property at load — /keep's and
  // /map's idiom, NOT a timer-installed wrapper (notes.md, "the apply-wrapper
  // race"). This link is the newest on the seam, so its `before` is measured
  // ahead of every other wrapper's work and its `after` behind all of it: the
  // rectangles compared are the ones a finger saw.
  if (typeof feature_Loop !== 'undefined') {
    const fm_glidePaint = feature_Loop.paint;
    feature_Loop.paint = function (html) {
      const off = feature_Glide.reduced();
      let was = null;
      try {
        if (!off) was = feature_Glide.snap();
      } catch (e) {
        was = null;
      }
      fm_glidePaint.call(this, html);
      if (off) {
        feature_Glide.level = feature_Glide.openTool();
        feature_Glide.moveUntil = 0;
        feature_Glide.fadeUntil = 0;
        feature_Glide.clearGhosts();
        return;
      }
      try {
        feature_Glide.run(was || new Map());
      } catch (e) {
        // a glide that throws must never cost the paint that already happened
      }
    };
  }
}
