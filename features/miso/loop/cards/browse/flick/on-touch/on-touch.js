// the phone's own road: touchstart/touchend survive a scroll; pointerup does not
if (typeof feature_Flick !== 'undefined') {
  document.addEventListener('touchstart', (e) => {
    const t = e.touches && e.touches[0];
    if (!t || (e.touches && e.touches.length > 1)) { feature_Flick.down = null; return; }
    feature_Flick.arm(t.clientX, t.clientY, e.target);
  }, { capture: true, passive: true });
  document.addEventListener('touchend', (e) => {
    const t = e.changedTouches && e.changedTouches[0];
    if (!t) return;
    feature_Flick.release(t.clientX, t.clientY);
  }, { capture: true, passive: true });
}
