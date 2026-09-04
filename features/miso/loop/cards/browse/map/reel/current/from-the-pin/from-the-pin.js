// a tap on a pin makes that post the current one: the band scrolls to its
// lozenge and the mark follows. It does not open the post — opening is the
// lozenge's own tap, one more finger away.
const feature_FromThePin = {
  // the lozenge for a post, in the band as it stands
  lozenge(id) {
    if (typeof feature_Reel === 'undefined' || !feature_Reel.list || !id) return null;
    for (const el of feature_Reel.list.querySelectorAll('.reel-post')) {
      if ((el.getAttribute('data-ev') || '') === 'browse_open:' + id) return el;
    }
    return null;
  },

  // true when this node has answered the tap. The band is scrolled so the
  // lozenge is the one at the left edge, which is what /current calls current
  // and what /on-the-pin rings on the map; the mark is set by hand as well as
  // by the scroll, because a lozenge already at the edge moves nothing and
  // fires no scroll event.
  claim(id) {
    const el = this.lozenge(id);
    if (!el) return false;
    const list = feature_Reel.list;
    const to = el.offsetLeft;
    try {
      if (list.scrollTo) list.scrollTo({ left: to, behavior: 'smooth' });
      else list.scrollLeft = to;
    } catch (e) { list.scrollLeft = to; }
    if (typeof feature_Reel.mark === 'function') {
      try { feature_Reel.mark(); } catch (e) { /* no band yet */ }
    }
    return true;
  },
};

{
  if (typeof feature_Map !== 'undefined') {
    // /map's own seam for what a tap on a pin means. A post the band does not
    // list has no lozenge to go to, so that tap still opens the post — which
    // is what every pin did before this node.
    const fm_pinTap = feature_Map.pinTap;
    feature_Map.pinTap = function (p) {
      let claimed = false;
      try { claimed = feature_FromThePin.claim(p && p.id); } catch (e) { claimed = false; }
      if (claimed) return;
      return fm_pinTap.call(this, p);
    };
  }
}
