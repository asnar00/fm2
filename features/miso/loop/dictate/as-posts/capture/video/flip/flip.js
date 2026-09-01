const feature_Flip = {
  // the camera the next recording asks for. The var is bridged, so the answer
  // is on the state the page already has — no extra road, and a device that
  // has never flipped reads the default and asks for the back camera exactly
  // as /capture/video always did.
  facing() {
    let s = {};
    try { s = JSON.parse(feature_Loop.state || '{}'); } catch (e) {}
    return s.facing === 'front' ? 'user' : 'environment';
  },

  // /capture/video asks the camera for what `constraints()` says, and this is
  // the one thing this node changes. Redefined rather than wrapped: there is
  // one answer, not a chain of them.
  install() {
    if (typeof feature_Video === 'undefined' || feature_Video.fm_flipWrapped) return;
    feature_Video.fm_flipWrapped = true;
    feature_Video.constraints = function () {
      return { video: { facingMode: feature_Flip.facing() }, audio: true };
    };
  },
};
const fm_flipInit = setInterval(() => {
  if (typeof feature_Video !== 'undefined' && typeof feature_Loop !== 'undefined') {
    clearInterval(fm_flipInit);
    feature_Flip.install();
  }
}, 100);
