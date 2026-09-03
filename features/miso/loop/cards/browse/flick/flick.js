// a vertical sweep on a card page at the end it is already at: next (up at
// the bottom) or previous (down at the top). In between it is a scroll.
{
  let fm_flDown = null;
  document.addEventListener('pointerdown', (e) => {
    fm_flDown = null;
    if (!e.isPrimary) return;
    const page = e.target && e.target.closest ? e.target.closest('.card-page') : null;
    if (!page) return;
    if (e.target.closest('.frame-win, #frameSheet, #greetSheet')) return;
    const a = document.activeElement;
    if (a && a.getAttribute && a.getAttribute('contenteditable') === 'true') return;
    const atTop = page.scrollTop <= 1;
    const atBottom = page.scrollTop + page.clientHeight >= page.scrollHeight - 1;
    fm_flDown = { y: e.clientY, x: e.clientX, t: Date.now(), id: e.pointerId, atTop, atBottom };
  }, true);
  document.addEventListener('pointerup', (e) => {
    const d = fm_flDown; fm_flDown = null;
    if (!d || e.pointerId !== d.id) return;
    const dy = e.clientY - d.y, dx = e.clientX - d.x;
    if (Date.now() - d.t > 600 || Math.abs(dy) < 60 || Math.abs(dx) >= 40) return;
    if (typeof feature_Loop === 'undefined') return;
    if (dy < 0 && d.atBottom) feature_Loop.send({ type: 'click', ev: 'browse_next' });
    else if (dy > 0 && d.atTop) feature_Loop.send({ type: 'click', ev: 'browse_prev' });
  }, true);
}
