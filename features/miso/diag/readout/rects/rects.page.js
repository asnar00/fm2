// the readout says where things are, not only what: every visible node
// carries its rectangle in CSS pixels — on an iPhone those are screen points,
// so a test can put a real finger on a selector (#p164a). data-ctl, the
// pencil's face and contenteditable ride along, and the root carries the
// visual viewport (the keyboard's shift) and the scroll.
if (typeof feature_Readout !== 'undefined') {
  const fm_rectsCapture = feature_Readout.capture;
  feature_Readout.capture = function (node) {
    const out = fm_rectsCapture.call(this, node);
    if (!out || !node.getBoundingClientRect) return out;
    if (!out.hidden) {
      const b = node.getBoundingClientRect();
      if (b.width || b.height) out.r = [Math.round(b.left), Math.round(b.top), Math.round(b.width), Math.round(b.height)];
    }
    const ctl = node.getAttribute('data-ctl'); if (ctl) out.ctl = ctl;
    const face = node.getAttribute('data-face'); if (face) out.face = face;
    if (node.getAttribute('contenteditable') === 'true') out.ce = true;
    if (node === document.body) {
      const vv = window.visualViewport;
      out.vv = vv ? [Math.round(vv.offsetTop), Math.round(vv.height)] : null;
      out.sy = Math.round(window.scrollY);
      // the web view is not the screen: a standalone app sits under the status
      // bar, so a finger needs screen.height - innerHeight added to y
      out.screen = [screen.width, screen.height, window.innerWidth, window.innerHeight];
      out.focus = document.activeElement && document.activeElement !== document.body
        ? (document.activeElement.tagName.toLowerCase() + (document.activeElement.className ? '.' + String(document.activeElement.className).split(' ')[0] : '')) : '';
    }
    return out;
  };
}
// where things are changes without the DOM changing: the keyboard's rise and
// fall, a scroll, a focus — each reposts the readout so a finger never aims
// by a stale viewport
if (typeof feature_Readout !== 'undefined') {
  const fm_rectsRepost = () => { if (feature_Readout.active) feature_Readout.schedule(); };
  if (window.visualViewport) {
    visualViewport.addEventListener('resize', fm_rectsRepost);
    visualViewport.addEventListener('scroll', fm_rectsRepost);
  }
  window.addEventListener('scroll', fm_rectsRepost, { passive: true });
  document.addEventListener('focusin', fm_rectsRepost);
  document.addEventListener('focusout', fm_rectsRepost);
}
