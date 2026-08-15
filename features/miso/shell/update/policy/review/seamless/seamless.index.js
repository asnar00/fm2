const feature_Seamless = {
  deferred: 0, // a build accepted while mid-task, waiting for idle

  // the elders that can be busy, each possibly absent from the composition
  busy() {
    if (typeof feature_Dictate !== 'undefined'
        && (feature_Dictate.active || feature_Dictate.playingId)) return true;
    if (typeof feature_Phone !== 'undefined' && feature_Phone.busy) return true;
    return false;
  },

  // resume: merge a matching stash beneath the fresh boot state — stashed
  // values return, keys the stash never knew keep their fresh defaults
  rehydrate() {
    let stash = null;
    try { stash = JSON.parse(localStorage.misoStash || 'null'); } catch (e) {}
    delete localStorage.misoStash; // consumed once, matching or not
    if (!stash || String(stash.v) !== String(localStorage.misoVersion)) return;
    let fresh = {}, old = {};
    try { fresh = JSON.parse(feature_Loop.state || '{}'); } catch (e) {}
    try { old = JSON.parse(stash.state || '{}'); } catch (e) {}
    feature_Loop.state = JSON.stringify(Object.assign({}, fresh, old));
    // nudge a re-render so the screen shows the resumed state (the event is
    // unknown to every update chain — a pass-through)
    setTimeout(() => feature_Loop.send({ type: 'seamless_resume' }), 0);
  },
};
{
  if (typeof feature_Review !== 'undefined') {
    const fm_seamlessApply = feature_Review.apply.bind(feature_Review);
    feature_Review.apply = async function (build) {
      if (feature_Seamless.busy()) {
        feature_Seamless.deferred = build; // the task finishes first
        return;
      }
      try {
        localStorage.misoStash = JSON.stringify(
          { v: build, state: feature_Loop.state });
      } catch (e) {}
      await fm_seamlessApply(build);
    };
  }
  const fm_seamlessLoopApply = feature_Loop.apply;
  feature_Loop.apply = function (p) {
    const firstBoot = feature_Loop.state === null;
    fm_seamlessLoopApply.call(this, p);
    if (firstBoot) feature_Seamless.rehydrate();
    if (feature_Seamless.deferred && !feature_Seamless.busy()
        && typeof feature_Review !== 'undefined') {
      const b = feature_Seamless.deferred;
      feature_Seamless.deferred = 0;
      feature_Review.apply(b);
    }
  };
}
