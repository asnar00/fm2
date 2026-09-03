{
  // /fan-out grows a fanned pin's stem from the old base (50 px pin, 12 px
  // stem); the bigger pin grows from its own, so the tip stays on the place
  if (typeof feature_FanOut !== 'undefined') {
    const fm_bigTurn = feature_FanOut.turn;
    feature_FanOut.turn = function (pin, deg, extra) {
      fm_bigTurn.call(this, pin, deg, extra);
      if (!deg) return;
      extra = extra || 0;
      pin.style.height = (75 + extra) + 'px';
      const stem = pin.querySelector('.map-pin-stem');
      if (stem) stem.style.borderTopWidth = (18 + extra) + 'px';
    };
  }
}
