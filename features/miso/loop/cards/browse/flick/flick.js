// a vertical sweep on a card page at the end it is already at: next (up at
// the bottom) or previous (down at the top). In between it is a scroll.
// The measure and the send are seams: arm(x, y, target) at the start of a
// gesture, release(x, y) at its end, go(dir) sends once — so a second road
// for the same gesture (/on-touch) shares the rule and cannot double-send.
const feature_Flick = {
  down: null,
  last: 0,
  arm(x, y, target) {
    this.down = null;
    const page = target && target.closest ? target.closest('.card-page') : null;
    if (!page) return;
    if (target.closest('.frame-win, #frameSheet, #greetSheet')) return;
    const a = document.activeElement;
    if (a && a.getAttribute && a.getAttribute('contenteditable') === 'true') return;
    const atTop = page.scrollTop <= 1;
    const atBottom = page.scrollTop + page.clientHeight >= page.scrollHeight - 1;
    this.down = { y, x, t: Date.now(), atTop, atBottom };
  },
  release(x, y) {
    const d = this.down; this.down = null;
    if (!d) return;
    const dy = y - d.y, dx = x - d.x;
    if (Date.now() - d.t > 600 || Math.abs(dy) < 60 || Math.abs(dx) >= 40) return;
    if (dy < 0 && d.atBottom) this.go('next');
    else if (dy > 0 && d.atTop) this.go('prev');
  },
  go(dir) {
    const now = Date.now();
    if (now - this.last < 400) return;
    this.last = now;
    if (typeof feature_Loop === 'undefined') return;
    feature_Loop.send({ type: 'click', ev: 'browse_' + dir });
  },
};
{
  let fm_flId = null;
  document.addEventListener('pointerdown', (e) => {
    if (!e.isPrimary) return;
    fm_flId = e.pointerId;
    feature_Flick.arm(e.clientX, e.clientY, e.target);
  }, true);
  document.addEventListener('pointerup', (e) => {
    if (e.pointerId !== fm_flId) return;
    feature_Flick.release(e.clientX, e.clientY);
  }, true);
}
