// a tap is decided on release, not on the browser's click. On the phone a
// press held past ~120 ms never produces a click (the black box, 2026-09-03:
// every lost press was down 127 ms or more, every landed one 114 or less;
// the simulator repeats it). pointerup comes for every press, so the tap is
// read there, and the browser's own click — when it does come — is the same
// tap twice and is stopped at the window before anyone reads it.
{
  const DRIFT = 12;                  // /long-press's own: further is a scroll
  let fm_relDown = null;             // {ev, x, y, id, t} for the press in progress
  let fm_relStop = false;            // the next trusted click is this tap's echo

  const fm_relButton = (el) => (el && el.closest ? el.closest('[data-ev]') : null);

  document.addEventListener('pointerdown', (e) => {
    fm_relStop = false;              // a new press: the last echo is not coming
    fm_relDown = null;
    if (!e.isPrimary || e.button !== 0) return;
    const b = fm_relButton(e.target);
    if (!b) return;
    fm_relDown = { ev: b.getAttribute('data-ev') || '', x: e.clientX, y: e.clientY,
                   id: e.pointerId, t: Date.now() };
  }, true);

  document.addEventListener('pointercancel', () => { fm_relDown = null; }, true);

  document.addEventListener('pointerup', (e) => {
    const d = fm_relDown; fm_relDown = null;
    if (!d || e.pointerId !== d.id) return;
    let target = null;
    // a hold /long-press has read: its click still goes, at the button it was
    // armed on, so the swallow those nodes hold can eat it and reset — on the
    // phone that click never came, and the swallow stayed armed for the next tap
    const held = typeof feature_LongPress !== 'undefined' && feature_LongPress.fired;
    if (held) {
      target = feature_LongPress.armed || null;
    } else {
      if (Math.hypot(e.clientX - d.x, e.clientY - d.y) > DRIFT) return;   // a scroll
      // by point, not e.target: a touch pointer's pointerup targets the element
      // it went down on, even one a repaint has since replaced
      const under = document.elementFromPoint(e.clientX, e.clientY);
      const b = fm_relButton(under);
      if (!b || (b.getAttribute('data-ev') || '') !== d.ev) return;         // came up off it
      target = b;
    }
    if (!target) return;
    fm_relStop = true;
    target.dispatchEvent(new MouseEvent('click', {
      bubbles: true, cancelable: true, view: window,
      clientX: e.clientX, clientY: e.clientY }));
  }, true);

  // the browser's click for the same press: stopped before any listener.
  // window capture runs ahead of every document listener, whatever order the
  // fragments loaded in. Only a trusted click — /drive's el.click() is not one.
  window.addEventListener('click', (e) => {
    if (!fm_relStop || !e.isTrusted) return;
    fm_relStop = false;
    e.stopImmediatePropagation();
    e.preventDefault();
  }, true);
}
