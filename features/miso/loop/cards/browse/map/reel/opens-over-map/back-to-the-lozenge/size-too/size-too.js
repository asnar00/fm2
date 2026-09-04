// the card closes onto the lozenge's whole rectangle — its height as well as
// its width — and it does so on every road out, the sideways flick included.
const feature_SizeToo = {
  early: null,   // the card whose shrink this node started on the flick's road

  // the lozenge's rectangle, not merely its width: the scale is taken on both
  // axes, so the card ends exactly the size and place of the lozenge it came
  // from. The fade is deeper and front-loaded, because a card squeezed to a
  // lozenge's height is a squashed card for the last of those milliseconds and
  // should be nearly gone by then.
  frames(page, r) {
    const c = page.getBoundingClientRect();
    if (!c.width || !c.height || !r.width || !r.height) return null;
    const sx = r.width / c.width, sy = r.height / c.height;
    if (!(sx > 0) || !(sy > 0)) return null;
    return [
      { transform: 'none', opacity: 1, offset: 0 },
      { opacity: 0.55, offset: 0.5 },
      { transform: 'translate(' + (r.left - c.left) + 'px, ' + (r.top - c.top)
                 + 'px) scale(' + sx + ', ' + sy + ')', opacity: 0.2, offset: 1 },
    ];
  },

  // the flick's road: /swipe-away marks the card and then sends, a quarter of
  // a second later, when its own animation would have finished. That animation
  // is off while this node is ticked (its rule is in this node's css), so the
  // card is still where it was and the shrink can start on the gesture instead
  // of waiting for the send. `going` is left set so /back-to-the-lozenge's own
  // interception lets that send straight through when it comes.
  catchFlick() {
    const B = feature_BackToTheLozenge;
    if (B.running()) return;
    const page = B.page();
    if (!page) return;
    if (!page.classList.contains('fm-swipe-left')
        && !page.classList.contains('fm-swipe-right')) return;
    const id = page.getAttribute('data-card') || '';
    const r = B.aim(id);
    if (!r || B.quiet()) return;
    const frames = B.frames(page, r);
    if (!frames) return;
    this.early = id;
    B.going = Date.now();
    B.play(page, id, frames, Date.now(), () => { /* the send is /swipe-away's, in a moment */ });
  },
};

{
  if (typeof feature_BackToTheLozenge !== 'undefined' && typeof feature_Loop !== 'undefined') {
    const S = feature_SizeToo;
    const B = feature_BackToTheLozenge;

    // the shape of the closing, on both axes
    B.frames = function (page, r) { return S.frames(page, r); };

    // and every road shrinks now, the sideways flick included (ash's word,
    // #p34): its own sideways motion is off, so this node owns the whole of
    // the closing rather than laying a second motion over the first.
    B.shrinks = function () { return true; };

    // the flick's mark goes on in /swipe-away's own pointerup, which is
    // registered before this one, so by the time this runs the card is marked.
    document.addEventListener('pointerup', () => {
      try { S.catchFlick(); } catch (e) { /* the send road still closes it */ }
    }, true);

    // the send that follows an early shrink passes straight through
    // /back-to-the-lozenge (its `going` is set), and this is where that flag is
    // let go — after the send, never before, or the card would close twice.
    // The band is aimed again afterwards for the same reason the parent does
    // it: /reel redraws it when the map's set comes back and puts the scroll
    // at the head while doing so.
    const fm_sizeSend = feature_Loop.send;
    feature_Loop.send = function (event) {
      const id = S.early;
      const out = fm_sizeSend.call(this, event);
      if (id) {
        S.early = null;
        B.going = 0;
        try { B.aim(id); } catch (e) { /* the band is where /reel left it */ }
      }
      return out;
    };
  }
}
