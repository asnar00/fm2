const feature_CreditsButton = {
  shown: false,   // the whole of the state: is the fold open

  // ---- the restructure --------------------------------------------------
  // /quiet-credits' show() has just written the section. The word `credits`
  // it wrote as a head becomes the button; everything it wrote after the
  // head becomes the fold's contents. No head means it drew nothing (no
  // tile route, a failed fetch, an answer that was not a credit) — then
  // there is nothing to credit and nothing to put a button on.
  fold(box) {
    if (!box) return;
    if (box.querySelector('#creditsBtn')) { this.render(); return; }  // re-entrant
    const head = box.querySelector('.credit-head');
    if (!head) return;

    const lines = document.createElement('div');
    lines.id = 'creditLines';
    let n = head.nextSibling;
    while (n) { const next = n.nextSibling; lines.appendChild(n); n = next; }

    const btn = document.createElement('button');
    btn.id = 'creditsBtn';
    btn.type = 'button';
    btn.textContent = (head.textContent || '').trim() || 'credits';
    btn.onclick = () => feature_CreditsButton.toggle();

    const row = document.createElement('div');
    row.id = 'creditsRow';
    row.appendChild(btn);

    head.replaceWith(row);
    box.appendChild(lines);
    this.render();
  },

  render() {
    const lines = document.getElementById('creditLines');
    if (lines) lines.style.display = this.shown ? '' : 'none';
    const btn = document.getElementById('creditsBtn');
    if (btn) {
      btn.classList.toggle('on', this.shown);
      btn.setAttribute('aria-expanded', this.shown ? 'true' : 'false');
    }
  },

  toggle() { this.shown = !this.shown; this.render(); },
};

{
  // /quiet-credits' own function, taken at load by property replacement —
  // not a timer (notes.md, "the apply-wrapper race"). With that node
  // unticked this whole block is absent from the composition anyway; the
  // guard is for the linker's sake and costs nothing.
  if (typeof feature_QuietCredits !== 'undefined') {
    const fm_cbShow = feature_QuietCredits.show.bind(feature_QuietCredits);
    // shown is cleared BEFORE the redraw, so a show() that throws between
    // the two leaves the flag folded rather than half-open. The whole body
    // is caught: /quiet-credits starts this un-awaited, so a rejection here
    // would be a silent unhandled one.
    feature_QuietCredits.show = async function () {
      feature_CreditsButton.shown = false;
      try {
        await fm_cbShow();
      } finally {
        try { feature_CreditsButton.fold(document.getElementById('credits')); }
        catch (e) { }
      }
    };
  }

  // every open of the sheet starts folded (/engineer's idiom). The redraw
  // above already folds it; this covers the one case it cannot — a show()
  // that rejected before writing anything, leaving the previous open's
  // unfolded markup standing.
  if (typeof feature_Panel !== 'undefined') {
    const fm_cbOpen = feature_Panel.open.bind(feature_Panel);
    feature_Panel.open = async function () {
      await fm_cbOpen();
      feature_CreditsButton.shown = false;
      try { feature_CreditsButton.fold(document.getElementById('credits')); }
      catch (e) { }
      feature_CreditsButton.render();
    };
  }
}
