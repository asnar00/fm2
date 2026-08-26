const feature_Delete = {
  // the question is the page half's and lives in no var: it is a thing being
  // asked, not a thing the world knows. Three seconds, and then the bin is
  // just a bin again.
  armed: false,
  timer: null,
  ticker: null,
  glyph: '',

  button() {
    return document.querySelector('.tool-button[data-ev="posts_delete"]');
  },

  // the id of the post on screen. /browse draws exactly one .card-page and
  // stamps it with the card's id, which is the same id every card event uses.
  openId() {
    const page = document.querySelector('.card-page[data-card]');
    return page ? page.getAttribute('data-card') : '';
  },

  ask() {
    const b = this.button();
    if (!b) return;
    if (!this.armed) this.glyph = b.innerHTML;
    this.armed = true;
    this.wear();
    if (this.timer) clearTimeout(this.timer);
    this.timer = setTimeout(() => feature_Delete.stand(), 3000);
    // every loop event repaints #app wholesale, and the toolbar is inside it —
    // a repaint that landed mid-question would answer it by putting the bin
    // back. Cheap for the three seconds it runs, and stopped the moment the
    // question is over.
    if (!this.ticker) this.ticker = setInterval(() => feature_Delete.wear(), 100);
  },

  wear() {
    if (!this.armed) return;
    const b = this.button();
    if (!b || b.getAttribute('data-sure') === '1') return;
    b.setAttribute('data-sure', '1');
    b.setAttribute('title', 'delete — tap again');
    b.textContent = 'sure?';
  },

  stand() {
    this.armed = false;
    if (this.timer) clearTimeout(this.timer);
    this.timer = null;
    if (this.ticker) clearInterval(this.ticker);
    this.ticker = null;
    const b = this.button();
    if (!b) return;
    b.removeAttribute('data-sure');
    b.setAttribute('title', 'delete');
    if (this.glyph) b.innerHTML = this.glyph;
  },

  // the second tap. /cards' own door, with the time on the event: no clock
  // inside update, so the tap carries it.
  go() {
    const id = this.openId();
    this.stand();
    if (!id) return;
    if (typeof feature_Loop === 'undefined' || !feature_Loop.state) return;
    feature_Loop.send({ type: 'CardDelete', data: { id, t: Date.now() } });
  },
};

{
  // taken in the CAPTURE phase so /loop's delegated click never sends it on:
  // one tap of this control is a question and not an event, and the second is
  // an event this half builds itself.
  //
  // Both answers come out of ONE listener because `wear()` empties the button
  // to put the word in it, which orphans the very element the tap landed on:
  // a second listener asking `closest` again would find nothing above a
  // detached <svg> and read the tap as "somewhere else" — which is exactly
  // what a two-listener first draft did, disarming itself on every tap
  // (rig-found, 2026-08-26). The target is read once, before anything moves.
  // A tap anywhere else withdraws the question: an unanswered "sure?" sitting
  // on the toolbar while you did something else is a control waiting to go off.
  document.addEventListener('click', (e) => {
    const hit = e.target && e.target.closest
      ? e.target.closest('[data-ev="posts_delete"]')
      : null;
    if (!hit) {
      if (feature_Delete.armed) feature_Delete.stand();
      return;
    }
    e.stopPropagation();
    e.preventDefault();
    if (feature_Delete.armed) {
      feature_Delete.go();
    } else {
      feature_Delete.ask();
    }
  }, true);
}
