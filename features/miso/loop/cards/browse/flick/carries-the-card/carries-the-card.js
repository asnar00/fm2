// the sweep carries the card: it moves with the finger, and past /flick's own
// threshold it slides off while the card it lands on slides in from the other
// side. The rule stays /flick's throughout — this node only gives it a body.
const feature_CarriesTheCard = {
  OUT: 140,        // the card leaving, from wherever the finger let go
  IN: 190,         // the card arriving, from the far side
  BACK: 190,       // the spring back, when the sweep was not enough
  GRAB: 8,         // the finger has to mean it before the card moves
  STUCK: 1500,     // a switch that never finished lets go of the gesture

  at: null,        // the sweep in progress
  busy: 0,         // when the switch started; 0 when nothing is running
  run: null,       // the animation in flight
  carry: null,     // it, held across a repaint

  page() { return document.querySelector('.card-page'); },

  quiet() {
    return !!(window.matchMedia
              && window.matchMedia('(prefers-reduced-motion: reduce)').matches);
  },

  running() { return !!this.busy && Date.now() - this.busy < this.STUCK; },

  canCarry() { return !this.quiet() && !!this.page(); },

  // ---- the finger ----------------------------------------------------------
  // /flick has already armed by the time these run — its own listeners were
  // registered first — so `feature_Flick.down` says where the sweep began and
  // which end of the card it began at. Nothing here decides anything: the card
  // is carried only in the direction and at the end /flick would act on.

  begin() {
    this.at = null;
    if (this.running() || this.quiet()) return;
    if (typeof feature_Flick === 'undefined' || !feature_Flick.down) return;
    if (!this.page()) return;
    this.at = { d: feature_Flick.down, on: false, dy: 0 };
  },

  // true once the card is being carried, so the caller can keep the browser's
  // own scroll out of a gesture that is no longer a scroll
  move(x, y) {
    const a = this.at;
    if (!a) return false;
    const dy = y - a.d.y, dx = x - a.d.x;
    if (!a.on) {
      if (Math.abs(dy) < this.GRAB || Math.abs(dy) <= Math.abs(dx)) return false;
      // the ends are /flick's: up at the bottom, down at the top. Anywhere else
      // the sweep is a scroll and the card must not move.
      if (!((dy < 0 && a.d.atBottom) || (dy > 0 && a.d.atTop))) { this.at = null; return false; }
      a.on = true;
    }
    a.dy = dy;
    // the page is looked up again on every move: a repaint mid-sweep replaces
    // the element, and the finger is still on the card either way
    const page = this.page();
    if (page) {
      page.classList.add('fm-carried');
      page.style.transform = 'translateY(' + Math.round(dy) + 'px)';
    }
    return true;
  },

  // /flick's release has run by now. If it called `go`, the switch is under
  // way and the card is already leaving; if it did not, the sweep was short of
  // the threshold and the card goes back where it was.
  end() {
    const a = this.at;
    this.at = null;
    if (!a || !a.on || this.running()) return;
    const page = this.page();
    if (!page) return;
    this.play(page, [{ transform: 'translateY(' + Math.round(a.dy) + 'px)' },
                     { transform: 'none' }], this.BACK, 'fm-carried', Date.now(), null);
  },

  // ---- the switch ----------------------------------------------------------
  // the card leaves towards the sweep, /flick's own event goes, and the card
  // that arrives comes in from the far side.
  out(dir, send) {
    const page = this.page();
    const a = this.at;
    const from = a && a.on ? a.dy : 0;
    const off = (dir === 'next' ? -1 : 1) * window.innerHeight;
    const was = page.getAttribute('data-card') || '';
    this.busy = Date.now();
    this.play(page, [{ transform: 'translateY(' + Math.round(from) + 'px)', opacity: 1 },
                     { transform: 'translateY(' + off + 'px)', opacity: 0.4 }],
              this.OUT, 'fm-carried', Date.now(), () => {
      send();
      const now = this.page();
      if (!now) { this.busy = 0; return; }
      // nothing switched — the sweep was at the end of the list — so the same
      // card comes back the way it went rather than being left off the screen
      const back = (now.getAttribute('data-card') || '') === was;
      this.play(now, [{ transform: 'translateY(' + (back ? off : -off) + 'px)', opacity: 0.4 },
                      { transform: 'none', opacity: 1 }],
                this.IN, 'fm-carry-in', Date.now(), () => { this.busy = 0; });
    });
  },

  // ---- one animation at a time, carried across the repaints -----------------
  play(page, frames, dur, cls, began, done) {
    page.classList.add(cls);
    const a = page.animate(frames, { duration: dur, easing: 'ease-out', fill: 'both' });
    const at = Math.max(0, Math.min(dur, Date.now() - began));
    if (at) { try { a.currentTime = at; } catch (e) { /* from the start, then */ } }
    const run = { id: page.getAttribute('data-card') || '', frames: frames, dur: dur,
                  began: began, cls: cls, anim: a, page: page, done: done };
    this.run = run;
    const self = this;
    a.finished.then(() => {
      if (self.run !== run) return;             // a later sweep took over
      self.run = null;
      self.clear(page, cls, a);
      if (done) done();
    }).catch(() => { /* cancelled: carried onto the element that came back */ });
  },

  // the animation is cancelled, not merely left finished: one left filling
  // goes on applying its last frame for the life of the element, and that
  // beats the inline transform the next carry puts there. Its last frame is
  // the card's own place, so cancelling it changes nothing on screen.
  clear(page, cls, a) {
    if (a) { try { a.cancel(); } catch (e) { /* already gone */ } }
    page.classList.remove(cls);
    page.style.transform = '';
    page.style.opacity = '';
  },
};

{
  if (typeof feature_Flick !== 'undefined' && typeof feature_Loop !== 'undefined') {
    const C = feature_CarriesTheCard;

    // ---- the seam ----------------------------------------------------------
    // `go` is /flick's own send-once. The send is deferred behind the card
    // leaving, and `last` is put back to zero first so the parent's 400 ms rule
    // reads the deferred send as this gesture's rather than a second one — the
    // zeroing happens ONLY on that road, so the immediate road below still has
    // the parent's own dedupe. A sweep arriving while a switch runs is dropped
    // here, which is what keeps /flick's pointer road and /on-touch's touch
    // road from starting two switches for one gesture.
    const fm_carryGo = feature_Flick.go;
    feature_Flick.go = function (dir) {
      if (C.running()) return;
      const self = this;
      if (!C.canCarry()) { fm_carryGo.call(self, dir); return; }
      C.out(dir, () => { self.last = 0; fm_carryGo.call(self, dir); });
    };

    // ---- the finger --------------------------------------------------------
    // registered after /flick's and /on-touch's, so `feature_Flick.down` is
    // already set when begin() runs and release() has already run when end()
    // does. Touch is the phone's road; the pointer road is the desktop's, and
    // a pointer event of type touch is left to the touch road — iOS cancels
    // those the moment it takes a gesture for a scroll (/on-touch's lesson).
    document.addEventListener('touchstart', (e) => {
      if (e.touches && e.touches.length > 1) { C.at = null; return; }
      C.begin();
    }, { capture: true, passive: true });

    document.addEventListener('touchmove', (e) => {
      const t = e.touches && e.touches[0];
      if (!t) return;
      // not passive, and acted on only for a gesture already claimed:
      // preventDefault is what stops the browser turning the carry into a
      // rubber-band on the page behind it
      if (C.move(t.clientX, t.clientY) && e.cancelable) e.preventDefault();
    }, { capture: true, passive: false });

    for (const fm_carryEnd of ['touchend', 'touchcancel']) {
      document.addEventListener(fm_carryEnd, () => C.end(), { capture: true, passive: true });
    }

    document.addEventListener('pointerdown', (e) => {
      if (e.isPrimary && e.pointerType !== 'touch') C.begin();
    }, true);
    document.addEventListener('pointermove', (e) => {
      if (e.pointerType === 'touch') return;
      C.move(e.clientX, e.clientY);
    }, true);
    for (const fm_carryUp of ['pointerup', 'pointercancel']) {
      document.addEventListener(fm_carryUp, (e) => {
        if (e.pointerType !== 'touch') C.end();
      }, true);
    }

    // ---- across the repaints ------------------------------------------------
    // every paint throws the card element away; an animation in flight is
    // measured by the wall clock and put on the element that comes back.
    const fm_carryPaint = feature_Loop.paint;
    feature_Loop.paint = function (html) {
      C.carry = null;
      try {
        if (C.run && Date.now() - C.run.began < C.run.dur) {
          C.carry = C.run;
          C.run = null;
          C.carry.anim.cancel();
        }
      } catch (e) { C.carry = null; }
      fm_carryPaint.call(this, html);
      try {
        const k = C.carry;
        C.carry = null;
        if (!k) return;
        const page = C.page();
        // the card went away under the animation — the post was closed, the
        // tool left. Let the gesture go rather than holding it for ever.
        if (!page || (page.getAttribute('data-card') || '') !== k.id) {
          C.busy = 0;
          if (k.done) k.done();
          return;
        }
        C.play(page, k.frames, k.dur, k.cls, k.began, k.done);
      } catch (e) { C.busy = 0; }
    };
  }
}
