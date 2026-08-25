const feature_Invite = {
  // the two fields, kept out of the DOM: #app is repainted wholesale by the
  // loop, so a half-typed invite would vanish under any other event
  draft: { name: '', phone: '' },
  asked: false,
  busy: false,
  said: '',

  // ask the server who the caller is allowed to be. `may` is the whole of the
  // member seam: a member gets {may:false, list:[]} and the renderer draws
  // nothing at all.
  async pull() {
    let d = null;
    try {
      const r = await fetch('users/invited', { cache: 'no-store' });
      d = await r.json();
    } catch (e) {
      d = null;
    }
    if (!d || !d.ok) d = { may: false, list: [] };
    if (this.said) {
      d.error = this.said;
      this.said = '';
    }
    if (typeof feature_Loop === 'undefined' || !feature_Loop.state) return;
    feature_Loop.send({ type: 'InviteList', data: d });
  },

  async send() {
    if (this.busy) return;
    const n = document.querySelector('.invite-name');
    const p = document.querySelector('.invite-phone');
    const name = ((n && n.value) || this.draft.name || '').trim();
    const phone = ((p && p.value) || this.draft.phone || '').trim();
    this.busy = true;
    try {
      const r = await fetch('users/invite', {
        method: 'POST', cache: 'no-store',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ name, phone }),
      });
      const d = await r.json().catch(() => null);
      if (d && d.ok) this.draft = { name: '', phone: '' };
      else this.said = (d && d.error) || "that invite didn't land";
    } catch (e) {
      this.said = "that invite didn't land";
    }
    this.busy = false;
    await this.pull();
  },

  async remove(phone) {
    if (this.busy || !phone) return;
    this.busy = true;
    try {
      const r = await fetch('users/uninvite', {
        method: 'POST', cache: 'no-store',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ phone }),
      });
      const d = await r.json().catch(() => null);
      if (!d || !d.ok) this.said = (d && d.error) || "that didn't work";
    } catch (e) {
      this.said = "that didn't work";
    }
    this.busy = false;
    await this.pull();
  },

  // the card page appearing is what this node reacts to, read from the DOM
  // rather than by wrapping feature_Loop.apply — that idiom races and orphans
  // other fragments' wrappers (notes.md, "the apply-wrapper race").
  look() {
    if (!document.querySelector('.card-page')) {
      this.asked = false;
      return;
    }
    if (!this.asked) {
      this.asked = true;
      this.pull();
    }
    // setting .value changes no child nodes, so this cannot re-fire the
    // observer that called it
    const n = document.querySelector('.invite-name');
    const p = document.querySelector('.invite-phone');
    if (n && !n.value && this.draft.name) n.value = this.draft.name;
    if (p && !p.value && this.draft.phone) p.value = this.draft.phone;
  },
};

{
  // the fields carry no data-ev, so the loop's own delegated click never fires
  // for them and typing never repaints the page out from under the caret
  document.addEventListener('input', (e) => {
    const el = e.target;
    if (!el || !el.classList) return;
    if (el.classList.contains('invite-name')) feature_Invite.draft.name = el.value;
    if (el.classList.contains('invite-phone')) feature_Invite.draft.phone = el.value;
  });

  document.addEventListener('click', (e) => {
    if (!e.target || !e.target.closest) return;
    if (e.target.closest('[data-invite="send"]')) {
      feature_Invite.send();
      return;
    }
    const x = e.target.closest('[data-invite="x"]');
    if (x) feature_Invite.remove(x.getAttribute('data-phone'));
  });

  const fm_invWatch = setInterval(() => {
    const app = document.getElementById('app');
    if (!app) return;
    clearInterval(fm_invWatch);
    const look = () => feature_Invite.look();
    new MutationObserver(look).observe(app, { childList: true, subtree: true });
    look();
  }, 100);
}
