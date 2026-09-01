const feature_Reports = {
  // the two fields are kept OUT of the DOM: #app is repainted wholesale by the
  // loop, so a half-typed question would vanish under any other event
  // (/invite's reason, and its idiom).
  draft: { name: '', ask: '' },
  asked: false,
  busy: false,

  // may I, and has the box a key to do it with? Asked once at load, because
  // the toolbar has to know before any page is open — /invite-tool's move. A
  // 403 is not an error on the page: it is the answer "no", and the glyph
  // simply never appears.
  async pull() {
    let d = null;
    try {
      const r = await fetch('reports/may', { cache: 'no-store' });
      d = await r.json();
    } catch (e) {
      d = null;
    }
    if (!d || !d.ok) d = { may: false, key: false };
    if (typeof feature_Loop === 'undefined' || !feature_Loop.state) return;
    feature_Loop.send({ type: 'ReportsMay', data: d });
  },

  // the owner's name lives behind the cookie and never in anyone's world, so
  // the page half looks it up — /posts' order, every step guarded so this
  // works with any of them unticked.
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

  ready() {
    const go = document.querySelector('.rep-make .rep-go');
    if (go) go.classList.toggle('off', !this.draft.ask.trim());
  },

  // make it, then run it. The card's id is `<owner>.<t>` for the `t` sent with
  // the event, so the id is known here without waiting for a paint.
  async make() {
    if (this.busy) return;
    if (typeof feature_Loop === 'undefined' || !feature_Loop.state) return;
    const ask = this.draft.ask.trim();
    if (!ask) return;
    this.busy = true;
    const owner = await this.name();
    const t = Date.now();
    feature_Loop.send({ type: 'ReportNew',
      data: { owner, title: this.draft.name.trim(), query: ask, t } });
    this.draft = { name: '', ask: '' };
    this.busy = false;
    this.runSoon((owner || 'you') + '.' + t);
  },

  // the card reaches the server as an ordinary op a moment after the event, so
  // a run sent immediately can arrive first and be answered "no such report".
  // Five tries over about seven seconds, and then it is left alone: the report
  // is on the phone either way, saying "not run yet", with a button on it.
  runSoon(id, tries) {
    const n = tries || 0;
    setTimeout(async () => {
      const ok = await this.run(id);
      if (!ok && n < 4) this.runSoon(id, n + 1);
    }, n === 0 ? 500 : 1000 * n);
  },

  // returns whether the server took it. `{ok:false}` with a reason — no key,
  // already working — is a taken answer: the card carries the state and the
  // page is already showing it.
  async run(id) {
    if (!id) return true;
    try {
      const r = await fetch('reports/run', {
        method: 'POST', cache: 'no-store',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ id }),
      });
      if (r.status === 404) return false;
      await r.json().catch(() => null);
      return true;
    } catch (e) {
      return false;
    }
  },

  // the fields are re-made by every repaint, so what was typed goes back in
  // after one. Setting .value changes no child nodes, so this cannot re-fire
  // the observer that called it (/invite's own note).
  look() {
    const n = document.querySelector('.rep-name');
    const a = document.querySelector('.rep-ask');
    if (n && !n.value && this.draft.name) n.value = this.draft.name;
    if (a && !a.value && this.draft.ask) a.value = this.draft.ask;
    if (a) this.ready();
  },
};

{
  // the toolbar needs to know whether you may report BEFORE any page is open,
  // so ask once at load — /invite-tool's own loader.
  const fm_repInit = setInterval(() => {
    if (typeof feature_Loop === 'undefined' || !feature_Loop.state) return;
    clearInterval(fm_repInit);
    feature_Reports.pull();
  }, 100);

  // the fields carry no data-ev, so the loop's own delegated click never fires
  // for them and typing never repaints the page out from under the caret
  document.addEventListener('input', (e) => {
    const el = e.target;
    if (!el || !el.classList) return;
    if (el.classList.contains('rep-name')) feature_Reports.draft.name = el.value;
    if (el.classList.contains('rep-ask')) {
      feature_Reports.draft.ask = el.value;
      feature_Reports.ready();
    }
  });

  document.addEventListener('keydown', (e) => {
    if (e.key !== 'Enter') return;
    const el = e.target;
    if (!el || !el.classList) return;
    if (!el.classList.contains('rep-ask') && !el.classList.contains('rep-name')) return;
    e.preventDefault();
    feature_Reports.make();
  });

  // every control carries data-rep rather than data-ev, so the loop does not
  // repaint #app out from under a half-typed question
  document.addEventListener('click', (e) => {
    if (!e.target || !e.target.closest) return;
    const el = e.target.closest('[data-rep]');
    if (!el) return;
    e.stopPropagation();
    const what = el.getAttribute('data-rep');
    if (what === 'make') {
      feature_Reports.make();
      return;
    }
    if (what === 'run') {
      el.classList.add('off');
      feature_Reports.run(el.getAttribute('data-id'));
    }
  }, true);

  const fm_repWatch = setInterval(() => {
    const app = document.getElementById('app');
    if (!app) return;
    clearInterval(fm_repWatch);
    const look = () => feature_Reports.look();
    new MutationObserver(look).observe(app, { childList: true, subtree: true });
    look();
  }, 100);
}
