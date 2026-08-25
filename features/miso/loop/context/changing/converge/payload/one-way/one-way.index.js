// the complaint, printed where the fragment that caused it can see it. Once per
// key per page: a lost write is a bug in one line of one fragment, and saying
// it every paint would bury it.
const feature_OneWay = {
  said: {},
  say(keys) {
    for (const k of keys || []) {
      if (this.said[k]) continue;
      this.said[k] = 1;
      console.warn('miso: the page wrote state["' + k + '"] and the context '
        + 'overwrote it. That key is published BY the context and never read '
        + 'back — send a CtxOp instead (see loop/context/changing/converge/payload).');
    }
  },
};
{
  const fm_oneWayApply = feature_Loop.apply;
  feature_Loop.apply = function (p) {
    fm_oneWayApply.call(this, p);
    try {
      feature_OneWay.say(JSON.parse(feature_Loop.state || '{}')._bridge_lost);
    } catch (e) { /* a state we cannot read is not a complaint */ }
  };
}
