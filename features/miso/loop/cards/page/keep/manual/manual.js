// no saving while you type: the save-as-you-type timer is switched off, so
// nothing repaints under your fingers. Your words are kept when you tap
// away — /cards' own rule — and a `save` pill on the card says so while a
// block is being edited (tapping it is a tap-away). /keep's rescue of
// words whose block vanished stays: an unsaved draft is never lost.
if (typeof feature_Keep !== 'undefined') {
  feature_Keep.soon = function () {};

  const fm_savePill = document.createElement('div');
  fm_savePill.id = 'cardSave';
  fm_savePill.textContent = 'save';
  document.body.appendChild(fm_savePill);
  const fm_place = () => {
    const el = document.activeElement;
    const editing = el && el.getAttribute && el.getAttribute('contenteditable') === 'true' && el.getAttribute('data-block') !== null;
    fm_savePill.classList.toggle('show', !!editing);
  };
  document.addEventListener('focusin', () => setTimeout(fm_place, 0));
  document.addEventListener('focusout', () => setTimeout(fm_place, 50));
  // pointerdown, not click: a click would follow the blur that the tap
  // itself causes, and the pill would already be gone
  fm_savePill.addEventListener('pointerdown', (e) => {
    e.preventDefault();
    const el = document.activeElement;
    if (el && el.blur) el.blur();   // the blur IS the save (/cards' focusout)
  });
}
