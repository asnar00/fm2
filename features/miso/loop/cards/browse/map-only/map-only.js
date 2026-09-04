// the projects map gets the band. /reel shows on the posts tool and
// /on-people-map widened it to 👤; the projects tool was in neither gate, and
// with the grid gone a projects map with no band is a dark rectangle holding
// a tool you cannot reach. The ids are already on the page — /reel writes
// data-ids on EVERY map surface — so this is a reading, not new data, and it
// is the same reading /people-there makes on 👤.
//
// Everything here wraps from this file. Nothing under /reel is edited: the
// idiom is /on-people-map's and /people-there's, which wrap the same two
// functions from their own files.
const feature_MapOnly = {
  // read from the screen and not from the state mirror: `open_tool` is
  // rewritten at the tool's own link after the mirror was published, so the
  // frame after a way-back tap says "" while the row already shows the tool
  // selected (/reel's own rule, housekeeping #p19).
  onProjects() {
    if (typeof document === 'undefined') return false;
    return !!document.querySelector('.toolbar .tool-button.sel[data-ev="tool_projects"]');
  },

  cards() {
    try {
      return JSON.parse(JSON.parse(feature_Loop.state || '{}').cards || '[]') || [];
    } catch (e) {
      return [];
    }
  },

  placeOf(c) {
    const blocks = c && Array.isArray(c.blocks) ? c.blocks : [];
    for (const b of blocks) {
      if (b && b.kind === 'location' && isFinite(b.lat) && isFinite(b.lon)) return [b.lat, b.lon];
    }
    return null;
  },

  // /reel's row shape, built from the surface's own set: id, owner, time,
  // face, words, place. The set is #mapData's data-ids — the whole set the
  // map was handed, placed or not — so a project with no location is in the
  // band and one tap from opening, which is the whole point of this node.
  rows() {
    const data = typeof document !== 'undefined' ? document.getElementById('mapData') : null;
    const ids = data ? data.getAttribute('data-ids') : null;
    if (ids === null) return [];
    const allowed = new Set(ids.split(',').filter((x) => x));
    const out = [];
    for (const c of this.cards()) {
      if (!c || !c.id || !allowed.has(c.id)) continue;
      const blocks = Array.isArray(c.blocks) ? c.blocks : [];
      const pic = blocks.find((b) => b && b.kind === 'picture' && typeof b.data === 'string' && b.data);
      const title = blocks.find((b) => b && b.kind === 'title' && b.text);
      const text = blocks.find((b) => b && b.kind === 'text' && b.text);
      out.push({
        id: c.id,
        owner: c.owner || '',
        t: c.edited || c.created || 0,
        face: pic ? pic.data : '',
        words: (title && title.text ? title.text : '') + (text && text.text ? ' — ' + text.text : ''),
        at: this.placeOf(c),
      });
    }
    // newest first, as every band in this tree is (/learned 8)
    out.sort((a, b) => b.t - a.t);
    return out;
  },

  // whatever the chain answered, plus the cards in the set it left out. On the
  // people map /everyone keeps the rows with a place or a live beacon — "the
  // pins are the band", written while the grid was still there to hold the
  // rest. With the grid gone the rest has nowhere to be, so the band takes it:
  // a person with no place is a lozenge with no pan, which is what a placeless
  // post has always been on the posts band.
  andTheRest(out) {
    const have = new Set(out.map((r) => r.id));
    const rest = this.rows().filter((r) => !have.has(r.id));
    if (!rest.length) return out;
    return out.concat(rest).sort((a, b) => b.t - a.t);
  },
};

{
  if (typeof feature_Reel !== 'undefined') {
    // the band shows on the projects tool's map too; every other surface is
    // answered by whatever was already in the chain.
    const fm_moShowing = feature_Reel.showing;
    feature_Reel.showing = function () {
      if (feature_MapOnly.onProjects()) return !!document.getElementById('mapData');
      return fm_moShowing.call(this);
    };
    // on the projects tool the rows ARE the set's own cards; everywhere else
    // the chain answers and this adds back whatever it left out of the set.
    const fm_moPosts = feature_Reel.posts;
    feature_Reel.posts = function () {
      if (feature_MapOnly.onProjects()) return feature_MapOnly.rows();
      return feature_MapOnly.andTheRest(fm_moPosts.call(this));
    };
  }
}
