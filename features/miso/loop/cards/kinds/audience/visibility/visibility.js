// the pick, sent from here because the write needs a clock and there is none
// inside `update` (misses.md, the clock in wasm). /audience's own promote is
// sent this way for the same reason, in the same capture phase, and this
// listener is that one's twin.
//
// CAPTURE, and the tap stops here: /loop's delegated listener would otherwise
// send `vis_lvl_<role>` through the Rust chain as a plain click, where the
// generic "anything else closes the panel" rule would shut it before the
// PostSetFloor arrived — one tap would then close the panel and change nothing.
{
  document.addEventListener('click', (e) => {
    const hit = e.target && e.target.closest
      ? e.target.closest('.armed-pop [data-ev^="vis_lvl_"]') : null;
    if (!hit) return;
    e.stopPropagation();
    e.preventDefault();
    if (typeof feature_Loop === 'undefined' || !feature_Loop.state) return;
    const floor = (hit.getAttribute('data-ev') || '').slice('vis_lvl_'.length);
    if (!floor) return;                       // the "same as me" row, if one is drawn
    const page = document.querySelector('.card-page[data-card]');
    const id = page ? page.getAttribute('data-card') : '';
    if (!id) return;
    feature_Loop.send({ type: 'PostSetFloor', data: { id, floor, t: Date.now() } });
  }, true);
}
