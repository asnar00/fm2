const feature_Replay = {
  active: /(^|[?&])replay=/.test(location.search),
  speed: 1,
  badge() {
    const d = document.createElement('div');
    d.textContent = 'REPLAY';
    d.style.cssText = 'position:fixed;top:calc(env(safe-area-inset-top,0px) + 10px);'
      + 'left:12px;z-index:99;padding:4px 10px;border-radius:999px;'
      + 'background:#3a1114;border:1px solid #7a2c31;color:#ff9a9a;'
      + 'font-size:11px;letter-spacing:.08em;';
    document.body.appendChild(d);
  },
  async start() {
    const data = await fetch('/replay.json', { cache: 'no-store' })
      .then((r) => r.ok ? r.json() : null).catch(() => null);
    if (!data || !data.entries || !data.entries.length) return;
    const m = location.search.match(/speed=([0-9.]+)/);
    if (m) this.speed = parseFloat(m[1]) || 1;
    const first = data.entries[0].t;
    let kf = null;
    for (const k of (data.keyframes || []))
      if (k.t <= first && (!kf || k.t > kf.t)) kf = k;
    feature_Loop.state = kf ? kf.state : '{}';
    feature_Loop.send({ type: 'replay-seed' });   // harmless: renders the seed
    for (const e of data.entries)
      setTimeout(() => feature_Loop.send(e.event), (e.t - first) / this.speed);
  },
};
if (feature_Replay.active && typeof feature_Loop !== 'undefined') {
  if (typeof feature_Blackbox !== 'undefined') feature_Blackbox.paused = true;
  // a replay is not a user session: don't bounce the ghost to login
  if (typeof feature_Gate !== 'undefined')
    feature_Gate.redirectIfLoggedOut = () => false;
  const fm_replayWait = setInterval(() => {
    if (feature_Loop.instance) {
      clearInterval(fm_replayWait);
      feature_Replay.badge();
      feature_Replay.start();
    }
  }, 100);
}
