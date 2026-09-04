// the mark on the band's lozenge. The reel is drawn in JS, outside the loop's
// own html, so /shows-progress' render cannot reach it: the world says which
// recordings are working and this puts that on the lozenge of each one's post.
const feature_OnTheReel = {
  // {cardId: 'on'|'stuck'} for every post whose words are still coming
  waiting() {
    const out = {};
    if (typeof feature_Loop === 'undefined' || !feature_Loop.state) return out;
    let s = {};
    try { s = JSON.parse(feature_Loop.state) || {}; } catch (e) { return out; }
    const w = s.dict_working || {};
    const working = Array.isArray(w.working) ? w.working : [];
    const stuck = Array.isArray(w.stuck) ? w.stuck : [];
    if (!working.length && !stuck.length) return out;
    let cards = [];
    try { cards = JSON.parse(s.cards || '[]') || []; } catch (e) { return out; }
    const kind = {};
    for (const id of working) kind[id] = 'on';
    for (const id of stuck) kind[id] = 'stuck';        // stuck outranks working
    for (const c of cards) {
      if (!c || !c.id || !c.rec) continue;
      if (kind[c.rec]) out[c.id] = kind[c.rec];
    }
    return out;
  },

  // set on the lozenges that are waiting and taken off the ones that are not,
  // so the mark goes the moment the words land without anything to clear it
  mark() {
    if (typeof feature_Reel === 'undefined' || !feature_Reel.list) return;
    const want = this.waiting();
    for (const el of feature_Reel.list.querySelectorAll('.reel-post')) {
      const ev = el.getAttribute('data-ev') || '';
      const id = ev.indexOf('browse_open:') === 0 ? ev.slice('browse_open:'.length) : '';
      const k = id && want[id];
      if (k) el.setAttribute('data-work', k);
      else el.removeAttribute('data-work');
    }
  },
};

{
  // after /map's sync, which is where /reel draws its row — so the mark is put
  // on a band that has just been rebuilt as well as on one that has not, and
  // a change in the world with no change in the set still moves the mark.
  if (typeof feature_Map !== 'undefined') {
    const fm_otrSync = feature_Map.sync;
    feature_Map.sync = function () {
      fm_otrSync.call(this);
      try { feature_OnTheReel.mark(); } catch (e) { /* the band is as /reel drew it */ }
    };
  }
}
