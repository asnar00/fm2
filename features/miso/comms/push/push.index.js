const feature_Push = {
  // panel-side: offer enrolment asking the REAL subscription state, not a
  // cached flag (a stale flag once left this button showing)
  async offerEnrol() {
    const standalone = typeof feature_Standalone !== 'undefined' && feature_Standalone.standalone();
    let enrolled = !!localStorage.misoPush;
    if (!enrolled && 'PushManager' in window) {
      try {
        const reg = await navigator.serviceWorker.ready;
        if (await reg.pushManager.getSubscription()) {
          enrolled = true;
          localStorage.misoPush = '1';
        }
      } catch (e) {}
    }
    if (!$('pushRow')) return;
    $('pushRow').style.display =
      ('PushManager' in window && standalone && !enrolled) ? '' : 'none';
  },
  async subscribe() {
    const reg = await navigator.serviceWorker.ready;
    const key = await fetch('push/vapid-key').then((r) => r.text());
    const sub = await reg.pushManager.subscribe({
      userVisibleOnly: true, applicationServerKey: fm_b64uToBuf(key.trim()) });
    const r = await fetch('push/subscribe', { method: 'POST',
      body: JSON.stringify({
        endpoint: sub.endpoint,
        p256dh: fm_bufToB64u(sub.getKey('p256dh')),
        auth: fm_bufToB64u(sub.getKey('auth')) }) }).then((x) => x.json());
    if (!r.ok) throw new Error(r.error || 'subscribe failed');
    localStorage.misoPush = '1';
  },
};
const fm_pushBtn = $('pushBtn');
if (fm_pushBtn) fm_pushBtn.onclick = async () => {
  try {
    await feature_Push.subscribe();
    $('pushBtn').textContent = 'notifications enabled ✓';
    setTimeout(() => { $('pushRow').style.display = 'none'; }, 1500);
  } catch (e) {
    if (typeof feature_Diag !== 'undefined')
      feature_Diag.report({ error: 'push enrol: ' + (e && e.message ? e.message : String(e)) });
    $('pushBtn').textContent = 'notifications blocked or unavailable';
  }
};
