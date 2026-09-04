// a tap on bare ground puts the popover away.
//
// The Rust half already closes it on any tap that carries a `data-ev` — that
// is /one-add's rule for its own picker, and it covers every button on the
// screen. What it cannot see is a tap on nobody's ground, because /loop only
// sends events for elements that carry one. /backdrop has the same job for a
// card page and cannot do it here: it returns early unless a `.card-page` is
// open, and the popover is not one.
//
// So this listener sends `armed_close`, which the Rust half treats as any
// other tap: it closes the popover and does nothing else. Sent rather than
// hidden in the DOM, because the popover is drawn by `render` and a repaint
// would bring back anything this half took away.
{
  const fm_inPlaceOwned = '.armed-pop, .toolbar, .browse-picker, #build, #panel, #shade, [data-ev], [contenteditable], input, button';
  document.addEventListener('click', (e) => {
    if (!e.target || !e.target.closest) return;
    if (!document.querySelector('.armed-pop')) return;      // nothing open
    if (e.target.closest(fm_inPlaceOwned)) return;          // that tap is somebody's
    if (typeof feature_Loop === 'undefined' || !feature_Loop.state) return;
    feature_Loop.send({ type: 'click', ev: 'armed_close' });
  });
}
