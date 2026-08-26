// the tap that saves your words still lands. A tap is a pointerdown and then
// a click; the pointerdown blurs the block you were writing in, the blur is
// /keep's save, the save repaints the screen — and the button under your
// finger is a new element by the time the click arrives, so the click lands
// on nothing (#p150: the ‹ and the grid/list/map picker needed a second press).
// The pointerdown remembers the button's event; a click that finds no button
// within the same tap sends it, and is stopped before /backdrop reads the
// bare ground as "close the card".
{
  // no time window (#p158: the keyboard's rise can delay the click past any
  // budget) — the press remembers, the next click consumes, a new press
  // overwrites
  let fm_downEv = '';
  document.addEventListener('pointerdown', (e) => {
    const el = e.target && e.target.closest ? e.target.closest('[data-ev]') : null;
    fm_downEv = el ? (el.getAttribute('data-ev') || '') : '';
  }, true);
  document.addEventListener('click', (e) => {
    const ev = fm_downEv; fm_downEv = '';
    if (!ev) return;
    if (e.target && e.target.closest && e.target.closest('[data-ev]')) return;   // it landed: /loop sends it
    if (typeof feature_LongPress !== 'undefined' && feature_LongPress.fired) return;   // a hold, not a tap
    if (typeof feature_Loop === 'undefined' || !feature_Loop.state) return;
    e.stopPropagation(); e.preventDefault();
    feature_Loop.send({ type: 'click', ev });
  }, true);
}
