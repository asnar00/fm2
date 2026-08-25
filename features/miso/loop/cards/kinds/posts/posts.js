const feature_Posts = {
  // the author's name lives behind the cookie and never in anyone's world, so
  // making a post is the page half's act: /me's own lookup, in the same order
  // — the shell's loader already asked, then /me's cached answer, then the
  // server. Every step guarded, so this works with any of them unticked.
  async name() {
    if (typeof feature_Panel !== 'undefined' && feature_Panel.lastWho)
      return feature_Panel.lastWho.name || '';
    if (typeof feature_Me !== 'undefined' && typeof feature_Me.name === 'function') {
      try { return (await feature_Me.name()) || ''; } catch (e) { /* ask below */ }
    }
    try {
      const w = await fetch('auth/whoami', { cache: 'no-store' }).then((r) => r.json());
      return (w && w.name) || '';
    } catch (e) {
      return '';
    }
  },

  // /new's one door, with the type this node's surface shows. The event opens
  // the card's page itself, so by the time send() returns the post is on the
  // screen and the caret can go into its words.
  async make() {
    if (typeof feature_Loop === 'undefined' || !feature_Loop.state) return;
    const owner = await this.name();
    feature_Loop.send({ type: 'CardNew',
      data: { owner, type: 'post', title: '', t: Date.now() } });
    this.caret();
    // the world answers a moment later — the op comes back, the place arrives
    // — and a repaint that lands between the caret and the first keystroke can
    // leave the words with nobody in them. Put it back, once, and only into a
    // post that is still empty with nothing else focused: after that the
    // caret is the writer's, and /keep carries it through every later repaint.
    setTimeout(() => feature_Posts.settle(), 400);
  },

  settle() {
    const el = document.querySelector('.card-page.post .card-text[contenteditable="true"]');
    if (!el || el === document.activeElement) return;
    if ((el.innerText || '').trim()) return;
    const at = document.activeElement;
    if (at && at.getAttribute && at.getAttribute('contenteditable') === 'true') return;
    this.caret();
  },

  // the caret into the post's words, at the end of them — a new post is empty,
  // so that is the start. /keep's rules take over from here: what is typed is
  // kept, and a repaint puts the caret back where it was.
  caret() {
    const el = document.querySelector('.card-page.post .card-text[contenteditable="true"]');
    if (!el) return;
    el.focus();
    try {
      const r = document.createRange();
      r.selectNodeContents(el);
      r.collapse(false);
      const s = document.getSelection();
      s.removeAllRanges();
      s.addRange(r);
    } catch (e) {
      /* a selection API that throws is one we do without */
    }
  },
};

{
  // the new button's tap, taken in the CAPTURE phase so /loop's own delegated
  // click never sends it on: the name is not in the world, so the event this
  // control fires has to be built here rather than in the update chain, and a
  // click that did both would make one post and waste one round trip.
  document.addEventListener('click', (e) => {
    if (!e.target || !e.target.closest) return;
    if (!e.target.closest('[data-ev="posts_new"]')) return;
    e.stopPropagation();
    e.preventDefault();
    feature_Posts.make();
  }, true);
}
