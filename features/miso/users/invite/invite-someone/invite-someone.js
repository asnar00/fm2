// the form waits behind one button: "invite someone". Tap it and the name and
// phone boxes appear; send (or tap the button again) folds them away. The
// open flag lives here, not in the DOM, because the loop repaints #app
// wholesale — so it is re-applied on every appearance of the rows.
const feature_InviteSomeone = {
  open: false,

  apply() {
    const box = document.querySelector('.invite');
    if (!box) return;
    let row = box.querySelector('.invite-someone');
    if (!row) {
      row = document.createElement('div');
      row.className = 'crow invite-someone';
      row.innerHTML = '<span class="invite-someone-btn" data-invite="someone">invite someone</span>';
      box.insertBefore(row, box.firstChild);
    }
    box.classList.toggle('invite-open', this.open);
    if (this.open) {
      const n = box.querySelector('.invite-name');
      if (n && document.activeElement !== n && !n.value
          && !(box.querySelector('.invite-phone') || {}).value) n.focus();
    }
  },

  toggle() {
    this.open = !this.open;
    this.apply();
  },
};

{
  // fold the form after a successful send: the row list changing under an
  // empty form is the signal, and it costs no hook into /invite's send
  document.addEventListener('click', (e) => {
    if (!e.target || !e.target.closest) return;
    if (e.target.closest('[data-invite="someone"]')) {
      e.stopPropagation();
      feature_InviteSomeone.toggle();
    }
  }, true);

  const fm_someoneWatch = new MutationObserver(() => feature_InviteSomeone.apply());
  const fm_someoneInit = setInterval(() => {
    const app = document.getElementById('app');
    if (!app) return;
    clearInterval(fm_someoneInit);
    fm_someoneWatch.observe(app, { childList: true, subtree: true });
    feature_InviteSomeone.apply();
  }, 100);

  // after /invite reports a successful send its draft empties; fold then
  if (typeof feature_Invite !== 'undefined' && feature_Invite.send) {
    const fm_someoneSend = feature_Invite.send.bind(feature_Invite);
    feature_Invite.send = async function () {
      await fm_someoneSend();
      const n = document.querySelector('.invite-name');
      const p = document.querySelector('.invite-phone');
      if ((!n || !n.value) && (!p || !p.value)) {
        feature_InviteSomeone.open = false;
        feature_InviteSomeone.apply();
      }
    };
  }
}
