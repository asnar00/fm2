// a tap-away that changed nothing sends nothing. /cards saves on every
// focusout, and a save is a repaint, and a repaint between a tap's mousedown
// and its mouseup replaces the button under the finger — so the first tap
// on a toolbar button after merely looking at a block did nothing (found by
// /people's rig). Capture-phase, ahead of /cards' listener: if the block's
// text is what the store already holds, stop the event here.
{
  const fm_storedText = (id, at) => {
    try {
      const list = JSON.parse(String(JSON.parse(feature_Loop.state || '{}').cards || '[]'));
      for (const c of list) if (c && c.id === id && c.blocks && c.blocks[at]) return String(c.blocks[at].text || '');
    } catch (e) {}
    return null;
  };
  document.addEventListener('focusout', (e) => {
    const el = e.target;
    if (!el || !el.getAttribute || el.getAttribute('contenteditable') !== 'true') return;
    const at = el.getAttribute('data-block');
    if (at === null || typeof feature_Cards === 'undefined' || !feature_Cards.textOf) return;
    const stored = fm_storedText(el.getAttribute('data-card'), Number(at));
    if (stored !== null && feature_Cards.textOf(el) === stored) e.stopImmediatePropagation();
  }, true);
}
