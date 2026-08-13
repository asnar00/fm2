const feature_Keyframes = {
  everyMs: 30 * 1000,
  everyEvents: 50,
  lastT: 0,
  sinceLast: 0,
};
if (typeof feature_Blackbox !== 'undefined' && typeof feature_Loop !== 'undefined') {
  const fm_kfRecord = feature_Blackbox.record;
  feature_Blackbox.record = function (event, stateAfter) {
    fm_kfRecord.call(this, event, stateAfter);
    feature_Keyframes.sinceLast += 1;
    const now = Date.now();
    if (now - feature_Keyframes.lastT >= feature_Keyframes.everyMs
        || feature_Keyframes.sinceLast >= feature_Keyframes.everyEvents) {
      this.log.keyframes.push({ t: now, state: stateAfter });
      feature_Keyframes.lastT = now;
      feature_Keyframes.sinceLast = 0;
      this.save();
    }
  };
}
