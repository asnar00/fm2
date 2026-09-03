// the band is everyone on the map: /live-only's rows, then the map's set
{
  if (typeof feature_Reel !== 'undefined') {
    const fm_evPosts = feature_Reel.posts;
    feature_Reel.posts = function () {
      const live = fm_evPosts.call(this);
      if (typeof this.onPeople !== 'function' || !this.onPeople()) return live;
      const data = document.getElementById('mapData');
      const ids = data ? data.getAttribute('data-ids') : null;
      if (ids === null) return live;
      const allowed = new Set(ids.split(',').filter((x) => x));
      let cards = [];
      try { cards = JSON.parse(JSON.parse(feature_Loop.state || '{}').cards || '[]'); } catch (e) { cards = []; }
      const seen = new Set(live.map((r) => r.id));
      const out = live.slice();
      for (const c of cards) {
        if (!c || !c.id || !allowed.has(c.id) || seen.has(c.id)) continue;
        const blocks = Array.isArray(c.blocks) ? c.blocks : [];
        const pic = blocks.find((b) => b && b.kind === 'picture' && typeof b.data === 'string' && b.data);
        const text = blocks.find((b) => b && b.kind === 'text' && b.text);
        const title = blocks.find((b) => b && b.kind === 'title' && b.text);
        const place = blocks.find((b) => b && b.kind === 'location' && isFinite(b.lat) && isFinite(b.lon));
        if (!place) continue;
        out.push({
          id: c.id, owner: c.owner || '',
          t: (place && place.t) || c.edited || c.created || 0,
          face: pic ? pic.data : '',
          words: (title && title.text ? title.text : (c.owner || '')) + (text ? ' — ' + text.text : ''),
          at: [place.lat, place.lon],
        });
      }
      out.sort((a, b) => b.t - a.t);
      return out;
    };
  }
}
