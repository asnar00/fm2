// the toolbar needs to know whether you may invite BEFORE any page is open,
// so ask once at load; /invite's own look() keeps it fresh from then on
const fm_inviteToolInit = setInterval(() => {
  if (typeof feature_Loop === 'undefined' || !feature_Loop.state) return;
  clearInterval(fm_inviteToolInit);
  if (typeof feature_Invite !== 'undefined' && feature_Invite.pull) feature_Invite.pull();
}, 100);
