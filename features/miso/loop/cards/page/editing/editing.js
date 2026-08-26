// a card opens read-only, even your own: an `edit` pill (author only)
// unlocks its blocks and the picture; `save` — /manual's pill, or a tap
// away — locks them again. Locking is done on every paint from the DOM
// (the renderer still emits contenteditable; this node removes it), which
// is also what keeps the locked state honest across repaints.
const feature_Editing = {
  open: {},   // card id -> true while being edited
  pill: null,

  page() { return document.querySelector('.card-page:not(.foreign)'); },
  id(page) { return page && (page.getAttribute('data-card') || ''); },

  apply() {
    const page = this.page();
    const editing = page && !!this.open[this.id(page)];
    if (page) {
      page.classList.toggle('locked', !editing);
      for (const el of page.querySelectorAll('.card-title, .card-text')) {
        if (editing) el.setAttribute('contenteditable', 'true');
        else el.removeAttribute('contenteditable');
      }
    }
    if (this.pill) this.pill.classList.toggle('show', !!page && !editing);
  },

  edit() {
    const page = this.page();
    if (!page) return;
    this.open[this.id(page)] = true;
    this.apply();
    const first = page.querySelector('.card-text') || page.querySelector('.card-title');
    if (first) { first.focus(); }
  },

  lock() {
    const page = this.page();
    if (page) delete this.open[this.id(page)];
    this.apply();
  },
};
{
  const fm_editPill = document.createElement('div');
  fm_editPill.id = 'cardEdit';
  fm_editPill.textContent = 'edit';
  document.body.appendChild(fm_editPill);
  feature_Editing.pill = fm_editPill;
  fm_editPill.addEventListener('pointerdown', (e) => { e.preventDefault(); feature_Editing.edit(); });

  // /manual's save pill locks again, once the blur that saves has happened
  document.addEventListener('pointerdown', (e) => {
    if (e.target && e.target.id === 'cardSave') setTimeout(() => feature_Editing.lock(), 120);
  }, true);

  // the picture is not for choosing or removing while locked
  document.addEventListener('click', (e) => {
    if (!e.target || !e.target.closest) return;
    const pic = e.target.closest('.card-page.locked .card-pic');
    if (pic) e.stopImmediatePropagation();
  }, true);
  document.addEventListener('pointerdown', (e) => {
    if (!e.target || !e.target.closest) return;
    if (e.target.closest('.card-page.locked .card-pic')) e.stopImmediatePropagation();
  }, true);

  const fm_editWatch = new MutationObserver(() => feature_Editing.apply());
  const fm_editInit = setInterval(() => {
    const app = document.getElementById('app');
    if (!app) return;
    clearInterval(fm_editInit);
    fm_editWatch.observe(app, { childList: true, subtree: true });
    feature_Editing.apply();
  }, 100);
}
