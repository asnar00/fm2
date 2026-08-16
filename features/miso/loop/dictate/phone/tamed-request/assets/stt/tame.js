// the taming module (see features/miso/loop/dictate/phone/tamed-request):
// clamp any GPUAdapter.requestDevice to exactly what the adapter offers —
// haze's recipe. Where the adapter grants everything this changes nothing;
// where it doesn't (iOS Safari), a doomed request becomes a grantable one.
// Optionally imported by engine.js; absence is the unticked state.

let installed = false;

export function prepare() {
  if (!installed && typeof GPUAdapter !== 'undefined') {
    const orig = GPUAdapter.prototype.requestDevice;
    GPUAdapter.prototype.requestDevice = function (desc) {
      try {
        const d = { ...(desc || {}) };
        if (d.requiredFeatures) {
          d.requiredFeatures = [...d.requiredFeatures]
            .filter((f) => this.features.has(f));
        }
        if (d.requiredLimits) {
          const clamped = {};
          for (const [k, v] of Object.entries(d.requiredLimits)) {
            if (v === undefined) continue;
            const cap = this.limits[k];
            if (cap === undefined) continue;          // unknown limit: drop
            if (k.startsWith('min')) {
              clamped[k] = v < cap ? cap : v;          // alignments clamp up
            } else {
              clamped[k] = v > cap ? cap : v;          // maxima clamp down
            }
          }
          d.requiredLimits = clamped;
        }
        return orig.call(this, d);
      } catch (e) {
        return orig.call(this, desc);                  // shim bug ≠ lost GPU
      }
    };
    installed = true;
  }
  // a wasm pin from the pre-tame fallback earns ONE fresh webgpu audition;
  // if the audition fails the engine re-pins, and the marker stops repeats
  try {
    if (localStorage.muonSttDevice === 'wasm' && !localStorage.muonSttShimTried) {
      localStorage.muonSttShimTried = '1';
      delete localStorage.muonSttDevice;
    }
  } catch (e) { /* storage walled off: nothing to retire */ }
}
