// the edit and save pills come off the page: one control in the toolbar's
// row does both jobs — a pencil while an own card is locked, a tick while it
// is being edited. It sits before undo, wearing the open tool's colour like
// /delete's bin. Placed from the DOM on every paint, the way /editing locks.
if (typeof feature_Editing !== 'undefined') {
  const fm_pencil = '<svg class="icon-svg" viewBox="0 0 24 24" aria-hidden="true">'
    + '<path d="M4 20l4.2-1 10.3-10.3a2 2 0 0 0 0-2.8l-0.4-0.4a2 2 0 0 0-2.8 0L5 15.8z" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linejoin="round"/>'
    + '<path d="M13.5 7.5l3 3" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round"/></svg>';
  const fm_tick = '<svg class="icon-svg" viewBox="0 0 24 24" aria-hidden="true">'
    + '<path d="M5 12.5l4.5 4.5L19 7.5" fill="none" stroke="currentColor" stroke-width="2.6" stroke-linecap="round" stroke-linejoin="round"/></svg>';

  const fm_applyWas = feature_Editing.apply.bind(feature_Editing);
  feature_Editing.apply = function () {
    fm_applyWas();
    const bar = document.querySelector('.toolbar');
    const page = this.page();
    let b = bar && bar.querySelector('[data-ctl="card_edit"]');
    if (!bar || !page) { if (b) b.remove(); return; }
    const editing = !!this.open[this.id(page)];
    if (!b) {
      b = document.createElement('div');
      b.className = 'tool-button ctrl';
      b.setAttribute('data-ctl', 'card_edit');
      // the open tool's colour, read off its own lit button
      const sel = bar.querySelector('.tool-button.sel.tinted');
      const colour = sel ? sel.style.getPropertyValue('--tool-colour') : '';
      if (colour) { b.classList.add('tinted'); b.style.setProperty('--tool-colour', colour); }
      const before = bar.querySelector('[data-ev="posts_delete"]') || bar.querySelector('[data-ev="ctx_undo"]');
      if (before) bar.insertBefore(b, before); else bar.appendChild(b);
    }
    const face = editing ? 'save' : 'edit';
    if (b.getAttribute('data-face') !== face) {
      b.setAttribute('data-face', face);
      b.setAttribute('title', face);
      b.innerHTML = editing ? fm_tick : fm_pencil;
    }
  };

  // pointerdown, not click — the pills' own reason: the tap's blur saves,
  // and the save repaints the toolbar, so by the time a click would fire the
  // button it was aimed at is gone (rig-found, 2026-08-26). preventDefault
  // keeps the block focused until the blur below, which IS the save
  // (/cards' focusout); the lock follows once it has landed. The click that
  // follows is swallowed in the capture phase so /loop never sees it.
  document.addEventListener('pointerdown', (e) => {
    const hit = e.target && e.target.closest ? e.target.closest('[data-ctl="card_edit"]') : null;
    if (!hit) return;
    e.preventDefault(); e.stopPropagation();
    if (hit.getAttribute('data-face') === 'save') {
      const el = document.activeElement;
      if (el && el.blur) el.blur();
      setTimeout(() => feature_Editing.lock(), 120);
    } else {
      feature_Editing.edit();
    }
  }, true);
  // the click that follows the pointerdown is swallowed WHEREVER it lands:
  // on the phone, edit() focuses the words, the keyboard rises, the toolbar
  // moves up under the finger, and the click hit-tests the ground instead —
  // which /backdrop reads as "close the card" (#p140). So the pointerdown arms
  // a swallow for the one click that follows it, for half a second.
  // Not a time window: a 600 ms one was still too short on the phone, where
  // the keyboard's rise delays the click (#p158). The press arms; the next
  // click anywhere is the one consumed; a press anywhere else disarms.
  let fm_swallow = false;
  document.addEventListener('pointerdown', (e) => {
    fm_swallow = !!(e.target && e.target.closest && e.target.closest('[data-ctl="card_edit"]'));
  }, true);
  document.addEventListener('click', (e) => {
    const own = e.target && e.target.closest && e.target.closest('[data-ctl="card_edit"]');
    if (own || fm_swallow) { fm_swallow = false; e.stopPropagation(); e.preventDefault(); }
  }, true);

  feature_Editing.apply();
}
