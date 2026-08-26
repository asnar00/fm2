// the finger, recorded: every touch, pointer, click and focus event, with
// what was under it and where the visual viewport sat — so a tap that went
// wrong on a phone can be read back from the black box instead of guessed
// at from a laptop (#p159). Entries are {type:'ui', ...}; the loop ignores
// the type if /replay ever re-sends one.
if (typeof feature_Blackbox !== 'undefined') {
  const fm_touchesName = (el) => {
    if (!el || !el.tagName) return String(el && el.nodeName || '?');
    let s = el.tagName.toLowerCase();
    if (el.id) s += '#' + el.id;
    if (typeof el.className === 'string' && el.className) s += '.' + el.className.trim().split(/\s+/).slice(0, 3).join('.');
    const ev = el.closest && el.closest('[data-ev],[data-ctl]');
    if (ev) s += '[' + (ev.getAttribute('data-ev') || ev.getAttribute('data-ctl')) + ']';
    return s;
  };
  const fm_touchesNote = (e) => {
    const p = (e.touches && e.touches[0]) || (e.changedTouches && e.changedTouches[0]) || e;
    const vv = window.visualViewport;
    const x = Math.round(p.clientX || 0), y = Math.round(p.clientY || 0);
    const under = (x || y) ? document.elementFromPoint(x, y) : null;
    feature_Blackbox.record({ type: 'ui', kind: e.type, target: fm_touchesName(e.target), under: fm_touchesName(under),
      x, y, vv: vv ? [Math.round(vv.offsetTop), Math.round(vv.height)] : null, sy: Math.round(window.scrollY),
      focus: fm_touchesName(document.activeElement) });
  };
  for (const k of ['touchstart', 'touchend', 'touchcancel', 'pointerdown', 'pointerup', 'pointercancel', 'click', 'focusin', 'focusout'])
    document.addEventListener(k, fm_touchesNote, { capture: true, passive: true });
  if (window.visualViewport) {
    const fm_touchesVV = () => feature_Blackbox.record({ type: 'ui', kind: 'viewport', vv: [Math.round(visualViewport.offsetTop), Math.round(visualViewport.height)], sy: Math.round(window.scrollY) });
    visualViewport.addEventListener('resize', fm_touchesVV); visualViewport.addEventListener('scroll', fm_touchesVV);
  }
}
