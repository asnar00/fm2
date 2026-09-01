// the staleness judge, pure so it can be tested off-device: given a
// notification body and the server's current build, answer how to treat it.
// {update:false}                     — not an update notice; pass through
// {update:true, build:N, stale:bool} — tag it, and drop it if stale
function fm_onlyNewsJudge(body, current) {
  const m = /^updated to build (\d+)/.exec(String(body || ''));
  if (!m) return { update: false };
  const build = parseInt(m[1], 10);
  const cur = parseInt(current, 10);
  return { update: true, build: build,
           stale: !isNaN(cur) && build < cur };
}

{
  // outermost wrap on the display point (composed after /attention's fork):
  // staleness is judged before visibility.
  const fm_onShow = self.registration.showNotification.bind(self.registration);
  self.registration.showNotification = function (title, opts) {
    const body = (opts && opts.body) || '';
    if (!/^updated to build \d+/.test(body)) {
      return fm_onShow(title, opts);
    }
    const tagged = Object.assign({}, opts, { tag: 'miso-update' });
    return fetch('version', { cache: 'no-store' })
      .then((r) => (r.ok ? r.text() : ''))
      .then((v) => {
        const j = fm_onlyNewsJudge(body, (v || '').trim());
        if (j.stale) return undefined;   // superseded: not news
        return fm_onShow(title, tagged);
      })
      // offline or refused: fail toward ringing, /attention's own rule
      .catch(() => fm_onShow(title, tagged));
  };
}
