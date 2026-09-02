// the page half of the gate: the render says whether it stands (the
// sentence's id is the marker); this reads it on every apply. Three jobs.
// It brings a fresh person to their card by sending the two taps a finger
// would send — 👤, then the own tile — so every frame is painted from a
// real turn (/restore's idiom; a write made past /payload's link would
// paint one stale frame). It withholds the toolbar by a body class. And it
// opens the card for writing through /editing's own mark, once per card
// id, dropping the mark again when the gate lifts.
const feature_ProfileFirst = {
  opened: '',   // the card id this node marked open in /editing
  sent: '',     // the last navigation event sent, so a slow turn is not doubled

  gated() {
    return !!document.getElementById('profileFirst');
  },

  // the own profile card's id, from the bridged cards list — the first
  // profile card, /people's own reading of "me"
  ownId() {
    try {
      const list = JSON.parse(String(JSON.parse(feature_Loop.state || '{}').cards || '[]'));
      // the own card is the profile card with no `from` on it — a copy /exchange
      // handed over carries the sender's name there (the toolbar review, 2026-09-02:
      // a member holding copies was sent to a copy and left on the grid)
      for (const c of list) if (c && c.type === 'profile' && !c.from) return c.id || '';
    } catch (e) {}
    return '';
  },

  send(ev) {
    if (this.sent === ev) return;
    this.sent = ev;
    feature_Loop.send({ type: 'click', ev });
  },

  apply() {
    const on = this.gated();
    document.body.classList.toggle('fm-profile-first', on);
    if (on) {
      let s = {};
      try { s = JSON.parse(feature_Loop.state || '{}'); } catch (e) {}
      const page = document.querySelector('.card-page');
      if (s.open_tool !== 'account') this.send('tool_account');
      else if (!page) {
        const id = this.ownId();
        if (id) this.send('browse_open:' + id);
      } else this.sent = '';
    } else this.sent = '';
    if (typeof feature_Editing === 'undefined') return;
    const page = feature_Editing.page();
    const id = page ? feature_Editing.id(page) : '';
    if (on && id && this.opened !== id) {
      feature_Editing.open[id] = true;
      feature_Editing.apply();
      this.opened = id;
    }
    if (!on && this.opened) {
      delete feature_Editing.open[this.opened];
      this.opened = '';
      feature_Editing.apply();
    }
  },

  init() {
    const fm_pfApply = feature_Loop.apply;
    const self = this;
    feature_Loop.apply = function (p) {
      fm_pfApply.call(this, p);
      self.apply();
    };
    self.apply();
  },
};

// while the gate stands the card is always open: a save (the tick, or a tap
// away) keeps the words and locks the card, and the lock is undone here at
// once, so the tick keeps saying save and nothing needs a pencil to go on.
// Property replacement at load — /me's idiom on /account's seam.
if (typeof feature_Editing !== 'undefined') {
  const fm_pfLock = feature_Editing.lock.bind(feature_Editing);
  feature_Editing.lock = function () {
    fm_pfLock();
    if (!feature_ProfileFirst.gated()) return;
    const page = this.page();
    const id = page ? this.id(page) : '';
    if (!id) return;
    this.open[id] = true;
    feature_ProfileFirst.opened = id;
    this.apply();
  };
}

const fm_pfInit = setInterval(() => {
  if (typeof feature_Loop !== 'undefined' && feature_Loop.state !== null) {
    clearInterval(fm_pfInit);
    feature_ProfileFirst.init();
  }
}, 100);
