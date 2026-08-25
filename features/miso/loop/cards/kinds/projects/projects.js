const feature_Projects = {
  sheet: null,
  who: null,
  role: null,
  addBtn: null,
  card: null,
  pick: null,
  named: null,

  // the world as the page sees it: the bridged cards list, which lags the
  // store by one turn. That is fine for a chooser — a person's card is not
  // going to stop existing between the paint and the tap — and it is why
  // nothing here fetches.
  cards() {
    try {
      return JSON.parse(String(JSON.parse(feature_Loop.state || '{}').cards || '[]')) || [];
    } catch (e) {
      return [];
    }
  },

  // the owner's name, which lives behind the cookie and not in the world.
  // Their own profile card carries it (it is the card with no `from` on it),
  // so the usual answer costs nothing; /me's fetch is the fallback for a
  // world whose 👤 has never been opened.
  async name() {
    for (const c of this.cards()) {
      if (c && c.type === 'profile' && !c.from && c.owner) return c.owner;
    }
    if (typeof feature_Me !== 'undefined' && feature_Me.name) {
      try { return (await feature_Me.name()) || ''; } catch (e) { /* fall through */ }
    }
    try {
      const w = await fetch('auth/whoami', { cache: 'no-store' }).then((r) => r.json());
      return (w && w.name) || '';
    } catch (e) {
      return '';
    }
  },

  // the one door for making a card of a type (/new); the surface it lands on
  // is chosen there too, so there is nothing to open here.
  async make() {
    if (typeof feature_Loop === 'undefined' || !feature_Loop.state) return;
    const owner = await this.name();
    feature_Loop.send({ type: 'CardNew',
      data: { owner, type: 'project', title: '', t: Date.now() } });
  },

  // the people you hold: every profile card in your world, yourself included
  // — ash adds himself as lead dev, which is the seed (#p14).
  people() {
    const out = [];
    for (const c of this.cards()) {
      if (!c || c.type !== 'profile') continue;
      let title = c.owner || '';
      for (const b of (c.blocks || [])) {
        if (b && b.kind === 'title' && b.text) { title = b.text; break; }
      }
      out.push({ id: c.id, name: title, mine: !c.from });
    }
    out.sort((a, b) => (a.mine === b.mine ? 0 : (a.mine ? -1 : 1)));
    return out;
  },

  open(cardId) {
    this.card = cardId;
    this.pick = null;
    this.named = null;
    this.role.value = '';
    const rows = this.people();
    this.who.innerHTML = '';
    if (!rows.length) {
      const none = document.createElement('div');
      none.className = 'proj-none';
      none.textContent = 'nobody yet — invite someone first';
      this.who.appendChild(none);
    }
    for (const p of rows) {
      const row = document.createElement('div');
      row.className = 'crow proj-pick';
      row.setAttribute('data-id', p.id);
      const name = document.createElement('div');
      name.className = 'ctext';
      name.textContent = p.name;
      row.appendChild(name);
      row.addEventListener('click', () => {
        for (const el of this.who.querySelectorAll('.proj-pick'))
          el.classList.remove('on');
        row.classList.add('on');
        this.pick = p.id;
        this.named = p.name;
        this.ready();
      });
      this.who.appendChild(row);
    }
    this.ready();
    this.sheet.classList.add('show');
    setTimeout(() => { try { this.role.focus(); } catch (e) {} }, 0);
  },

  // a control that does nothing must not look like a control (/taste 7): add
  // is dim until there is a person and a word for what they do.
  ready() {
    const ok = !!this.pick && !!this.role.value.trim();
    this.addBtn.classList.toggle('off', !ok);
    return ok;
  },

  close() {
    this.sheet.classList.remove('show');
    this.card = null;
    this.pick = null;
  },

  // taking a role away. It is sent from here rather than as a data-ev click
  // for the same reason `add` is: the event carries the time, and the half of
  // the loop that runs in the browser has no clock to stamp it with.
  drop(card, to) {
    if (!card || !to) return;
    if (typeof feature_Loop === 'undefined' || !feature_Loop.state) return;
    feature_Loop.send({ type: 'RoleDrop',
      data: { card, to, t: Date.now() } });
  },

  add() {
    if (!this.ready() || !this.card) return;
    feature_Loop.send({ type: 'RoleAdd', data: {
      card: this.card, to: this.pick, name: this.named || '',
      role: this.role.value.trim(), t: Date.now() } });
    this.close();
  },
};

{
  // furniture made at load and living OUTSIDE #app, so a repaint of the
  // loop's html while the sheet is open cannot take it away — /frame's
  // sheet and #cardToast are the precedents.
  const fm_projSheet = document.createElement('div');
  fm_projSheet.id = 'projSheet';

  const fm_projBox = document.createElement('div');
  fm_projBox.id = 'projBox';

  const fm_projHead = document.createElement('div');
  fm_projHead.className = 'proj-sheet-head';
  fm_projHead.textContent = 'people';
  fm_projBox.appendChild(fm_projHead);

  const fm_projWho = document.createElement('div');
  fm_projWho.id = 'projWho';
  fm_projBox.appendChild(fm_projWho);

  const fm_projRole = document.createElement('input');
  fm_projRole.id = 'projRole';
  fm_projRole.type = 'text';
  fm_projRole.placeholder = 'role';
  fm_projRole.autocomplete = 'off';
  fm_projBox.appendChild(fm_projRole);

  const fm_projBar = document.createElement('div');
  fm_projBar.id = 'projBar';
  const fm_projCancel = document.createElement('button');
  fm_projCancel.id = 'projCancel';
  fm_projCancel.type = 'button';
  fm_projCancel.textContent = 'cancel';
  const fm_projAdd = document.createElement('button');
  fm_projAdd.id = 'projAdd';
  fm_projAdd.type = 'button';
  fm_projAdd.textContent = 'add';
  fm_projBar.appendChild(fm_projCancel);
  fm_projBar.appendChild(fm_projAdd);
  fm_projBox.appendChild(fm_projBar);

  fm_projSheet.appendChild(fm_projBox);
  document.body.appendChild(fm_projSheet);

  feature_Projects.sheet = fm_projSheet;
  feature_Projects.who = fm_projWho;
  feature_Projects.role = fm_projRole;
  feature_Projects.addBtn = fm_projAdd;

  // no data-ev on any of them, so the loop's own delegated click never fires
  // for the sheet — the rule cards.js follows for .card-pic.
  fm_projCancel.addEventListener('click', (e) => {
    e.preventDefault();
    feature_Projects.close();
  });
  fm_projAdd.addEventListener('click', (e) => {
    e.preventDefault();
    feature_Projects.add();
  });
  fm_projRole.addEventListener('input', () => feature_Projects.ready());
  fm_projRole.addEventListener('keydown', (e) => {
    if (e.key === 'Enter') {
      e.preventDefault();
      feature_Projects.add();
    }
  });
  // tapping the dark outside is the way out of a sheet everywhere else
  fm_projSheet.addEventListener('click', (e) => {
    if (e.target === fm_projSheet) feature_Projects.close();
  });

  // the two controls the render chain draws: **new** in the toolbar and
  // **add** under a project. Both carry data-proj rather than data-ev, so
  // the loop does not repaint #app out from under the sheet.
  document.addEventListener('click', (e) => {
    if (!e.target || !e.target.closest) return;
    const el = e.target.closest('[data-proj]');
    if (!el) return;
    const what = el.getAttribute('data-proj');
    if (what === 'new') {
      feature_Projects.make();
      return;
    }
    if (what === 'add') {
      feature_Projects.open(el.getAttribute('data-card'));
      return;
    }
    if (what === 'drop') {
      feature_Projects.drop(el.getAttribute('data-card'),
                            el.getAttribute('data-to'));
    }
  });
}
