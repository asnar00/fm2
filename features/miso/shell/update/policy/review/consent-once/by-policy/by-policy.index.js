const feature_ByPolicy = {
  inflight: 0,                        // the build whose stamp is being sent
  verdict: { key: '', ok: false },    // /policy's answer, per (build, policy)

  server() {
    return typeof feature_Review !== 'undefined' ? feature_Review.server() : 0;
  },
  running() {
    return typeof feature_Review !== 'undefined' ? feature_Review.running() : 0;
  },
  accepted() {
    return typeof feature_ConsentOnce !== 'undefined'
      ? feature_ConsentOnce.accepted() : 0;
  },

  // may this build proceed without review? /policy's own question — auto
  // (and the empty string a new user carries, which /policy reads as auto)
  // says yes, fixes says yes when every pending change is a fix, consent
  // never. Remembered per (build, policy): under fixes the answer costs a
  // fetch and cannot change for a given build.
  async allowed(server) {
    if (typeof feature_Policy === 'undefined') return false;
    const key = server + ':' + feature_Policy.current();
    if (this.verdict.key !== key) {
      const ok = !(await feature_Policy.consentNeeded());
      this.verdict = { key: key, ok: ok };
    }
    return this.verdict.ok;
  },

  // the acceptance, stamped by the instance the way the update button stamps
  // it: the one key, recorded — every apply path downstream runs unchanged
  async accept() {
    if (typeof feature_Update === 'undefined' || !feature_Update.newer()) return;
    if (typeof feature_Replay !== 'undefined' && feature_Replay.active) return;
    if (feature_Loop.state === null || !feature_Loop.instance) return;
    const server = this.server();
    if (!server || server <= this.running()) return;
    if (this.accepted() >= server || this.inflight === server) return;
    this.inflight = server;
    try {
      if (await this.allowed(server) && this.accepted() < server)
        feature_Loop.send({ type: 'AcceptUpdate', data: { build: server } });
    } finally {
      this.inflight = 0;
    }
  },

  // the handle's pulse means "a build is waiting for you"; a build the policy
  // lets through waits for nobody. Under auto the answer needs no fetch, so
  // the class comes off in the same task /watch put it on — no frame paints
  // it; under fixes it comes off once the verdict is in and the build is
  // accepted
  async quiet() {
    const server = this.server();
    if (!server || typeof feature_Policy === 'undefined') return;
    const auto = feature_Policy.current() === 'auto';
    if (!auto && (this.accepted() < server || !(await this.allowed(server)))) return;
    const handle = $('build');
    if (handle) handle.classList.remove('update');
  },
};
{
  if (typeof feature_Watch !== 'undefined') {
    const fm_byPolicyCheck = feature_Watch.check.bind(feature_Watch);
    feature_Watch.check = async function () {
      const v = await fm_byPolicyCheck();
      feature_ByPolicy.quiet();          // synchronous up to its first await
      await feature_ByPolicy.accept();
      await feature_ByPolicy.quiet();
      return v;
    };
  }

  // the launch decline and a policy picked mid-session both arrive as a
  // state change; a stamp that waited for the loop to boot goes now
  const fm_byPolicyApply = feature_Loop.apply;
  feature_Loop.apply = function (p) {
    fm_byPolicyApply.call(this, p);
    feature_ByPolicy.accept();
  };

  // a push that found this window visible is /attention's page message; when
  // it carries /push's build notice the build is known — check now rather
  // than at the next poll
  if (navigator.serviceWorker && navigator.serviceWorker.addEventListener
      && typeof feature_Watch !== 'undefined') {
    navigator.serviceWorker.addEventListener('message', (e) => {
      if (e && e.data && e.data.fm === 'attention'
          && /^updated to build \d+/.test(String(e.data.body || '')))
        feature_Watch.check();
    });
  }
}
