// the two rows on the second welcome page: each settles — enabled, not
// possible here, or declined after a real try — and got it waits for both
const feature_SetUp = {
  state: { passkey: '', push: '' },   // '' pending; 'on' | 'no' | 'off' settled

  standalone() {
    return matchMedia('(display-mode: standalone)').matches || navigator.standalone === true;
  },
  // what the device makes possible, asked once per apply
  look() {
    if (!this.state.passkey) {
      if (typeof feature_Passkey === 'undefined' || !window.PublicKeyCredential) this.state.passkey = 'no';
      else if (localStorage.misoPasskey) this.state.passkey = 'on';
    }
    if (!this.state.push) {
      if (typeof feature_Push === 'undefined' || !('PushManager' in window) || !this.standalone()) this.state.push = 'no';
      else if (localStorage.misoPush) this.state.push = 'on';
    }
  },
  words(which) {
    const s = this.state[which];
    if (s === 'on') return '✓';
    if (s === 'off') return 'not now';
    if (s === 'no') return which === 'push' && !this.standalone() ? 'home-screen app only' : 'not on this device';
    return 'enable';
  },
  apply() {
    const sheet = document.getElementById('greetSheet');
    if (!sheet) return;
    this.look();
    for (const row of sheet.querySelectorAll('.greet-row')) {
      const which = row.getAttribute('data-setup');
      const s = this.state[which];
      row.classList.toggle('settled', !!s);
      row.classList.toggle('on', s === 'on');
      const btn = row.querySelector('.greet-do');
      if (btn) btn.textContent = this.words(which);
    }
    const rows = sheet.querySelectorAll('.greet-row');
    let all = true;
    for (const row of rows) if (!this.state[row.getAttribute('data-setup')]) all = false;
    sheet.classList.toggle('setup-wait', !all);
  },
  async enable(which) {
    if (this.state[which]) return;
    try {
      if (which === 'passkey') await feature_Passkey.enrol();
      else await feature_Push.subscribe();
      this.state[which] = 'on';
    } catch (e) {
      this.state[which] = 'off';
      if (typeof feature_Diag !== 'undefined')
        feature_Diag.report({ error: 'set-up ' + which + ': ' + (e && e.message ? e.message : String(e)) });
    }
    this.apply();
  },
};
document.addEventListener('click', (e) => {
  const btn = e.target && e.target.closest ? e.target.closest('#greetSheet .greet-row .greet-do') : null;
  if (!btn) return;
  e.preventDefault(); e.stopPropagation();
  feature_SetUp.enable(btn.parentElement.getAttribute('data-setup'));
}, true);
{
  const fm_setupInit = setInterval(() => {
    if (typeof feature_Loop !== 'undefined' && feature_Loop.state !== null) {
      clearInterval(fm_setupInit);
      const fm_setupApply = feature_Loop.apply;
      feature_Loop.apply = function (p) {
        fm_setupApply.call(this, p);
        feature_SetUp.apply();
      };
      feature_SetUp.apply();
    }
  }, 100);
}
