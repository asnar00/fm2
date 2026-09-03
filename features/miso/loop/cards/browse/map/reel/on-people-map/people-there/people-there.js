{
  if (typeof feature_Reel !== 'undefined') {
    const fm_therePosts = feature_Reel.posts;
    feature_Reel.posts = function () {
      if (typeof this.onPeople !== 'function' || !this.onPeople()) return fm_therePosts.call(this);
      const data = document.getElementById('mapData');
      const ids = data ? data.getAttribute('data-ids') : null;
      if (ids === null) return fm_therePosts.call(this);
      const allowed = new Set(ids.split(',').filter((x) => x));
      let cards = [];
      try { cards = JSON.parse(JSON.parse(feature_Loop.state || '{}').cards || '[]'); } catch (e) { cards = []; }
      const out = [];
      for (const c of cards) {
        if (!c || !c.id || !allowed.has(c.id)) continue;
        const blocks = Array.isArray(c.blocks) ? c.blocks : [];
        const pic = blocks.find((b) => b && b.kind === 'picture' && typeof b.data === 'string' && b.data);
        const text = blocks.find((b) => b && b.kind === 'text' && b.text);
        const title = blocks.find((b) => b && b.kind === 'title' && b.text);
        const place = blocks.find((b) => b && b.kind === 'location' && isFinite(b.lat) && isFinite(b.lon));
        out.push({
          id: c.id, owner: c.owner || '',
          t: (place && place.t) || c.edited || c.created || 0,
          face: pic ? pic.data : '',
          words: (title && title.text ? title.text : (c.owner || '')) + (text ? ' — ' + text.text : ''),
          at: place ? [place.lat, place.lon] : null,
        });
      }
      out.sort((a, b) => b.t - a.t);
      return out;
    };
  }
}
