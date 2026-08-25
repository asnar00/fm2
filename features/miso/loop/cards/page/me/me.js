const feature_Me = {
  wasOpen: false,
  who: null,

  // the name lives behind the cookie, not in anyone's world. The shell's
  // loader already asked; ask again only if the panel is not composed.
  async name() {
    if (typeof feature_Panel !== 'undefined' && feature_Panel.lastWho)
      return feature_Panel.lastWho.name || '';
    if (this.who) return this.who.name || '';
    try {
      this.who = await fetch('auth/whoami', { cache: 'no-store' }).then((r) => r.json());
    } catch (e) {
      this.who = null;
    }
    return (this.who && this.who.name) || '';
  },

  // has this instance's world arrived yet? An ensure sent before the join
  // lands reads an empty world and makes a SECOND card, which last-write then
  // sends over the first. /veil already answers the question for the whole
  // page — `fm-joined` on the body, set by the join or by its timeout — and
  // with /veil unticked nobody can answer it, so we act at once.
  ready() {
    if (typeof feature_Veil === 'undefined') return true;
    return !!feature_Veil.joined || document.body.classList.contains('fm-joined');
  },

  // one ensure per transition into the tool, and never before the world is
  // here: /cards makes the card if there isn't one and does nothing if there
  // is, so a repeat is free — but only against a world that has arrived.
  async ensure() {
    for (let i = 0; i < 100 && !this.ready(); i++)
      await new Promise((r) => setTimeout(r, 100));
    const owner = await this.name();
    feature_Loop.send({ type: 'CardEnsure',
      data: { owner, type: 'profile', t: Date.now() } });
  },

  watch() {
    let s = {};
    try { s = JSON.parse(feature_Loop.state || '{}'); } catch (e) {}
    const isOpen = (s.open_tool === 'account');
    if (isOpen && !this.wasOpen) this.ensure();
    this.wasOpen = isOpen;
  },

  init() {
    const fm_meApply = feature_Loop.apply;
    const self = this;
    feature_Loop.apply = function (p) {
      fm_meApply.call(this, p);
      self.watch();
    };
    self.watch();
  },
};

// take /account's open seam: the card page is drawn by the render chain, so
// the tool needs no sheet. Unticked, /account's own pair opens the panel again.
if (typeof feature_Account !== 'undefined') {
  feature_Account.openTool = function () {};
  feature_Account.closeTool = function () {};
}

const fm_meInit = setInterval(() => {
  if (typeof feature_Loop !== 'undefined' && feature_Loop.state !== null) {
    clearInterval(fm_meInit);
    feature_Me.init();
  }
}, 100);
