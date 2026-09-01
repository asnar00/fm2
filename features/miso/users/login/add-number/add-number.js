const feature_AddNumber = {
  // the fields live out of the DOM: #app is repainted wholesale by the loop, so
  // a half-typed number would vanish under any other event
  draft: { phone: '', pin: '' },
  asked: false,
  busy: false,
  said: '',
  sent: false,

  async pull() {
    let d = null;
    try {
      const r = await fetch('users/number', { cache: 'no-store' });
      d = await r.json();
    } catch (e) {
      d = null;
    }
    if (!d || !d.ok) d = { ok: false };
    d.sent = this.sent;
    if (this.said) {
      d.error = this.said;
      this.said = '';
    }
    if (typeof feature_Loop === 'undefined' || !feature_Loop.state) return;
    feature_Loop.send({ type: 'MyNumber', data: d });
  },

  async send() {
    if (this.busy) return;
    const el = document.querySelector('.addnum-phone');
    const phone = ((el && el.value) || this.draft.phone || '').trim();
    if (!phone) return;
    this.busy = true;
    try {
      const r = await fetch('users/number/request', {
        method: 'POST', cache: 'no-store',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ phone }),
      });
      const d = await r.json().catch(() => null);
      if (d && d.ok) {
        this.sent = true;
        this.draft.phone = phone;
        this.draft.pin = '';
      } else {
        this.said = (d && d.error) || "that didn't work";
      }
    } catch (e) {
      this.said = "that didn't work";
    }
    this.busy = false;
    await this.pull();
  },

  async confirm() {
    if (this.busy) return;
    const el = document.querySelector('.addnum-pin');
    const pin = ((el && el.value) || this.draft.pin || '').trim();
    if (!pin) return;
    this.busy = true;
    try {
      const r = await fetch('users/number/confirm', {
        method: 'POST', cache: 'no-store',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ phone: this.draft.phone, pin }),
      });
      const d = await r.json().catch(() => null);
      if (d && d.ok) {
        this.sent = false;
        this.draft = { phone: '', pin: '' };
      } else {
        this.said = (d && d.error) || "that didn't work";
        this.draft.pin = '';
      }
    } catch (e) {
      this.said = "that didn't work";
    }
    this.busy = false;
    await this.pull();
  },

  // the card page appearing is what this node reacts to, read from the DOM
  // rather than by wrapping feature_Loop.apply — that idiom races and orphans
  // other fragments' wrappers (notes.md, "the apply-wrapper race")
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
    const p = document.querySelector('.addnum-phone');
    const c = document.querySelector('.addnum-pin');
    if (p && !p.value && this.draft.phone) p.value = this.draft.phone;
    if (c && !c.value && this.draft.pin) c.value = this.draft.pin;
  },
};

{
  document.addEventListener('input', (e) => {
    const el = e.target;
    if (!el || !el.classList) return;
    if (el.classList.contains('addnum-phone')) feature_AddNumber.draft.phone = el.value;
    if (el.classList.contains('addnum-pin')) feature_AddNumber.draft.pin = el.value;
  });

  document.addEventListener('click', (e) => {
    if (!e.target || !e.target.closest) return;
    const hit = e.target.closest('[data-addnum]');
    if (!hit) return;
    const what = hit.getAttribute('data-addnum');
    if (what === 'send') feature_AddNumber.send();
    if (what === 'confirm') feature_AddNumber.confirm();
  });

  const fm_addnumWatch = setInterval(() => {
    const app = document.getElementById('app');
    if (!app) return;
    clearInterval(fm_addnumWatch);
    const look = () => feature_AddNumber.look();
    new MutationObserver(look).observe(app, { childList: true, subtree: true });
    look();
  }, 100);
}
