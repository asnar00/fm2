// a card the finger has touched keeps its arrival behind it: the grow that
// /opens-over-map plays when a card comes up must never play again on a card
// that is already up, and the end of a carry is where it was being restarted.
const feature_StaysPut = {
  // one class, added when a gesture takes the card and never taken off again.
  // Not a toggle: the element is thrown away by the next paint anyway, and a
  // class that comes off is a class that can restart what it was holding back
  // — which is the whole of this bug.
  mark() {
    const p = document.querySelector('#app .card-page');
    if (p) p.classList.add('fm-put');
  },
};

{
  if (typeof feature_CarriesTheCard !== 'undefined') {
    const fm_spBegin = feature_CarriesTheCard.begin;
    feature_CarriesTheCard.begin = function () {
      fm_spBegin.call(this);
      if (this.at) {
        try { feature_StaysPut.mark(); } catch (e) { /* the card is as it was */ }
      }
    };
  }
}
