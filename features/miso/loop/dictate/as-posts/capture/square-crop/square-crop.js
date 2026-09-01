const feature_SquareCrop = {
  // the central square of whatever was shot: the shorter edge is the side, and
  // the crop is centred on both axes, so a landscape loses its ends and a
  // portrait loses its top and bottom. One answer for every road that stores a
  // picture — a photo, a poster frame, a picture chosen from the roll — because
  // they all ask /cards the same question.
  //
  // The side is also the size: a square smaller than EDGE is never blown up,
  // and a bigger one comes down to EDGE, which is /cards' own sizing rule with
  // one edge instead of two.
  frameOf(w, h) {
    const side = Math.min(w, h) || 1;
    const edge = this.EDGE || 256;
    const d = Math.max(1, Math.round(Math.min(side, edge)));
    return { sx: Math.round((w - side) / 2), sy: Math.round((h - side) / 2),
             sw: side, sh: side, dw: d, dh: d };
  },

  // redefined rather than wrapped: there is one answer to "which pixels", not
  // a chain of them. Guarded so the fragment survives /cards being toggled off.
  install() {
    if (typeof feature_Cards === 'undefined' || feature_Cards.fm_squareCropOn) return;
    feature_Cards.fm_squareCropOn = true;
    feature_Cards.frameOf = feature_SquareCrop.frameOf;
  },
};
const fm_squareCropInit = setInterval(() => {
  if (typeof feature_Cards !== 'undefined') {
    clearInterval(fm_squareCropInit);
    feature_SquareCrop.install();
  }
}, 100);
