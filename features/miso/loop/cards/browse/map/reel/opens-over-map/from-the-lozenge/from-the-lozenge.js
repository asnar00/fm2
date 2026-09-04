// the page grows out of the lozenge that was tapped: it starts at that
// rectangle, clipped to the strip the lozenge stood for, and opens to its own
// size — so the lozenge and the page read as one thing.
const feature_FromTheLozenge = {
  MS: 240,
  from: null,     // {id, left, top, width, height} — the lozenge of the last tap
  pageId: '',     // the post standing over the map at the last paint
  run: null,      // the opening in flight
  carry: null,    // it, held across a repaint

  // the lozenge's rectangle is read at the tap, not at the opening:
  // /opens-over-map hides the reel the moment the page is up, and a hidden
  // element has no rectangle left to read.
  mark(el) {
    const ev = (el && el.getAttribute('data-ev')) || '';
    if (ev.indexOf('browse_open:') !== 0) { this.from = null; return; }
    const r = el.getBoundingClientRect();
    if (!r.width || !r.height) { this.from = null; return; }
    this.from = { id: ev.slice('browse_open:'.length),
                  left: r.left, top: r.top, width: r.width, height: r.height };
  },

  quiet() {
    return !!(window.matchMedia
              && window.matchMedia('(prefers-reduced-motion: reduce)').matches);
  },

  // the page standing over the map right now, or null
  over() {
    if (!document.body.classList.contains('fm-map-behind')) return null;
    return document.querySelector('.card-page');
  },

  // the two keyframes: the lozenge's rectangle, then the page's own. The scale
  // is uniform — a page squashed into the lozenge's shape is not the same
  // thing seen smaller — and transform and opacity are the whole of it,
  // because those two are the only properties a browser will run off the main
  // thread. A `clip-path` leg (a band widening into a page, which reads even
  // better) was measured on the rig and froze for 226 ms with the main thread,
  // which is exactly as busy as it ever is in the moment a post opens.
  frames(page, from) {
    const c = page.getBoundingClientRect();
    if (!c.width || !c.height || !from.width) return null;
    const s = Math.min(1, from.width / c.width);
    if (!(s > 0)) return null;
    return [
      { transform: 'translate(' + (from.left - c.left) + 'px, ' + (from.top - c.top)
                 + 'px) scale(' + s + ')', opacity: 0.55 },
      { transform: 'none', opacity: 1 },
    ];
  },

  // `began` is the wall clock of the opening, not the animation's own — a
  // fresh animation reads currentTime 0 until the browser has resolved its
  // start time, so carrying that number across a burst of paints restarted the
  // motion from the top each time and the card sat still for a quarter second
  // before it moved (measured on the rig).
  play(page, frames, began) {
    const at = Math.max(0, Math.min(this.MS, Date.now() - began));
    page.style.transformOrigin = 'top left';
    page.classList.add('fm-lozenge-open');
    const a = page.animate(frames, { duration: this.MS, easing: 'ease-out', fill: 'both' });
    if (at) { try { a.currentTime = at; } catch (e) { /* from the start, then */ } }
    this.run = { id: page.getAttribute('data-card') || '', frames: frames,
                 anim: a, page: page, began: began };
    const settle = () => this.settle(page);
    a.finished.then(settle).catch(settle);
  },

  // the page keeps none of it: a transform left behind makes the card a
  // containing block for everything inside it, and a clip that outlived its
  // animation would cut the card off at the fold.
  // ...and the suppression of /opens-over-map's own grow is handed straight
  // from one class to the other: a rule going from `animation: none` back to a
  // named animation STARTS it, so simply dropping the open class made the card
  // pulse from 96% the moment the growth finished (measured on the rig).
  settle(page) {
    page.classList.add('fm-loz-settled');
    page.classList.remove('fm-lozenge-open');
    page.style.transformOrigin = '';
    // and the animation is cancelled, not merely finished: an animation left
    // filling goes on applying its last frame for the life of the element,
    // and that beats any inline transform a later gesture puts there (it beat
    // /carries-the-card's carry, which is how this was found). The last frame
    // is the page's own place, so cancelling it changes nothing on screen.
    if (this.run && this.run.page === page && this.run.anim) {
      try { this.run.anim.cancel(); } catch (e) { /* already gone */ }
    }
    if (this.run && this.run.page === page) this.run = null;
  },
};

{
  if (typeof feature_Loop !== 'undefined') {
    // one tap, one opening: a tap anywhere else forgets the lozenge, so a pin
    // tapped after a lozenge does not open from the lozenge's old place.
    document.addEventListener('click', (e) => {
      if (!e.target || !e.target.closest) return;
      feature_FromTheLozenge.mark(e.target.closest('#mapReel .reel-post'));
    }, true);

    // /loop's paint seam, taken by replacing the property at load (/keep's
    // idiom). This wrapper is outside /map's, which is outside
    // /opens-over-map's, so by the time the second half runs the map has been
    // shown and the body marked — and the page has not been drawn on screen
    // yet, so the opening starts on the frame the page arrives on.
    const fm_lozPaint = feature_Loop.paint;
    feature_Loop.paint = function (html) {
      const F = feature_FromTheLozenge;
      // an opening in flight belongs to a page this paint is about to throw
      // away. Its progress is kept and put on the page that comes back: four
      // paints land in the seconds after a post opens, and without this every
      // one of them would snap the card to full size mid-motion.
      F.carry = null;
      try {
        if (F.run && F.run.anim && Date.now() - F.run.began < F.MS) {
          F.carry = { id: F.run.id, frames: F.run.frames, began: F.run.began };
          F.run.anim.cancel();
        }
      } catch (e) { F.carry = null; }
      fm_lozPaint.call(this, html);
      try {
        const page = F.over();
        const id = page ? (page.getAttribute('data-card') || '') : '';
        const wasId = F.pageId;
        F.pageId = id;
        if (!page) { F.run = null; return; }
        if (F.carry && F.carry.id === id) { F.play(page, F.carry.frames, F.carry.began); return; }
        // a repaint is not an arrival. Every paint makes a new element, so
        // /opens-over-map's grow-from-96% ran again on each one and the card
        // pulsed under the reader; from the second paint of a post onwards it
        // is off.
        if (id === wasId) { page.classList.add('fm-loz-settled'); return; }
        if (F.quiet()) return;
        const from = F.from;
        if (!from || from.id !== id) return;
        F.from = null;
        const frames = F.frames(page, from);
        if (frames) F.play(page, frames, Date.now());
      } catch (e) { /* the page arrives as /opens-over-map draws it */ }
    };
  }
}
