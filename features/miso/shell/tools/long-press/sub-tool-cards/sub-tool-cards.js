const feature_SubToolCards = {
  isControl(el) {
    return el && el.closest ? el.closest('.tool-button.ctrl[data-ev]') : null;
  },
};
{
  if (typeof feature_LongPress !== 'undefined') {
    // redefinition + saved original: controls resolve via the subtools
    // stamp; tool_ buttons pass through to the parent's resolution
    const existingContentFor = feature_LongPress.contentFor.bind(feature_LongPress);
    feature_LongPress.contentFor = async (btn) => {
      const ev = btn.getAttribute('data-ev') || '';
      if (ev.startsWith('tool_')) return existingContentFor(btn);
      const fallback = btn.getAttribute('title') || ev;
      if (typeof feature_Chooser !== 'undefined') {
        try {
          await feature_Chooser.load();
          const n = feature_Chooser.flat.find((x) => (x.subtools || []).includes(ev));
          if (n) return { name: n.name, intro: n.intro || n.purpose || '' };
        } catch (e) {}
      }
      return { name: fallback, intro: '' };
    };
    // controls arm the parent's own timer/state, so its drift-cancel,
    // release-disarm, and dismiss listeners govern the hold unchanged
    document.addEventListener('pointerdown', (e) => {
      const btn = feature_SubToolCards.isControl(e.target);
      if (!btn) return;
      feature_LongPress.disarm();
      feature_LongPress.fired = false;
      feature_LongPress.armed = btn;
      feature_LongPress.x = e.clientX;
      feature_LongPress.y = e.clientY;
      feature_LongPress.timer = setTimeout(() => feature_LongPress.show(btn), 500);
    });
    // a long press reads; it must not also fire the control
    document.addEventListener('click', (e) => {
      if (feature_LongPress.fired && feature_SubToolCards.isControl(e.target)) {
        e.stopPropagation();
        e.preventDefault();
        feature_LongPress.fired = false;
      }
    }, true);
  }
}
