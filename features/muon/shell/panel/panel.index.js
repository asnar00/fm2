const feature_Panel = {
  lastWho: null,
  async open() {
    const live = typeof feature_Watch !== 'undefined'
      ? await feature_Watch.check() : null;
    const up = typeof feature_Update !== 'undefined' ? feature_Update : null;
    const running = up ? up.running : '?';
    const upgrade = up && up.newer();
    const status = upgrade ? ` → ${up.server} available`
      : (typeof feature_Honest !== 'undefined'
         ? feature_Honest.statusText(live) : '');
    const who = feature_Panel.lastWho;
    $('who').textContent =
      (who && who.name ? 'logged in as ' + who.name : 'not logged in')
      + ' · build ' + running + status;
    $('updateBtn').style.display = upgrade ? '' : 'none';

    if (typeof feature_Passkey !== 'undefined') feature_Passkey.offerEnrol();
    if (typeof feature_Push !== 'undefined') feature_Push.offerEnrol();

    // seam: what fills the area under the who-line. A newer feature may
    // claim it (the chooser mounts the feature list); the default is the
    // recent-changes teaser, so unticking the occupant restores it.
    if (typeof feature_Chooser !== 'undefined' && feature_Chooser.mount) {
      await feature_Chooser.mount();
    } else {
      const changes = await fetch('changes.json', { cache: 'no-store' })
        .then((r) => r.ok ? r.json() : []).catch(() => []);
      $('changes').innerHTML = changes.slice(0, 6).map((c) =>
        '<div class="change"><b>' + c.build + '</b> '
        + String(c.text).replace(/</g, '&lt;') + '</div>').join('');
    }
    $('shade').style.display = 'block';
    $('panel').style.display = 'block';
  },
  close() {
    $('shade').style.display = 'none';
    $('panel').style.display = 'none';
  },
};
// seam: what the corner button's tap does (default: open the panel);
// a later feature may redefine it without touching this file
feature_Panel.buttonTap = () => feature_Panel.open();
const fm_buildBtn = $('build');
if (fm_buildBtn) fm_buildBtn.onclick = () => feature_Panel.buttonTap();
$('shade').onclick = () => feature_Panel.close();
$('logoutBtn').onclick = async () => {
  await fetch('auth/logout', { method: 'POST' }).catch(() => {});
  location.reload();
};
$('updateBtn').onclick = async () => {
  if (typeof feature_Update !== 'undefined' && feature_Update.server)
    localStorage.muonVersion = feature_Update.server;
  if (typeof feature_Update !== 'undefined') await feature_Update.evict();
  else await caches.delete('muon');
  location.reload();
};
