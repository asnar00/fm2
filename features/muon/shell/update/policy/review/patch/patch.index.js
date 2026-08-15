const feature_Patch = {
  async swap(build) {
    const res = await fetch('client.wasm', { cache: 'no-store' });
    if (!res.ok) return false;
    const { instance } = await WebAssembly.instantiate(await res.arrayBuffer(), {});
    if (!instance || !instance.exports.fm_entry) return false;
    feature_Loop.instance = instance;
    await feature_Delta.quiet(build);
    // one render through the new logic; the state was simply never touched
    feature_Loop.send({ type: 'patch_resume' });
    return true;
  },
};
if (typeof feature_Review !== 'undefined' && typeof feature_Delta !== 'undefined') {
  const fm_patchApply = feature_Review.apply.bind(feature_Review);
  feature_Review.apply = async function (build) {
    try {
      const fresh = await feature_Delta.fetchLive();
      const old = feature_Delta.stored();
      if (fresh && old) {
        const changedCode = feature_Delta.diff(old, fresh)
          .filter((p) => feature_Delta.code(p));
        if (changedCode.length && changedCode.every((p) => p === 'client.wasm')
            && await feature_Patch.swap(build)) {
          return; // patched in live — the wrapped chain (stash, reload) rests
        }
      }
    } catch (e) {}
    await fm_patchApply(build);
  };
}
