// /patch swaps the wasm module in place; the world lives inside it. Carry
// the newest records across the swap and rejoin, so the first frame the new
// module draws is the world the person had.
const feature_WorldAlong = {
  last: null,   // the newest world records a payload carried
};
if (typeof feature_Patch !== 'undefined') {
  const fm_waApply = feature_Loop.apply;
  feature_Loop.apply = function (payloadJson) {
    fm_waApply.call(this, payloadJson);
    try {
      const w = JSON.parse(payloadJson).world;
      if (w && w.length) feature_WorldAlong.last = w;
    } catch (e) { /* a payload we cannot read carries nothing to remember */ }
  };

  const fm_waSwap = feature_Patch.swap.bind(feature_Patch);
  feature_Patch.swap = async function (build) {
    const cache = typeof feature_WorldCache !== 'undefined' ? feature_WorldCache : null;
    const ctx = feature_WorldAlong.last;
    if (cache && ctx) cache.hold();
    let ok = false;
    try {
      ok = await fm_waSwap(build);
    } finally {
      if (cache && ctx) cache.release();
    }
    if (!ok) return false;
    if (ctx) feature_Loop.send({ type: 'WorldHydrate', data: { ctx } });
    if (typeof feature_Resume !== 'undefined') feature_Resume.join();
    return true;
  };
}
