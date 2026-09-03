{
  if (typeof feature_Reel !== 'undefined') {
    const fm_pplShowing = feature_Reel.showing;
    feature_Reel.showing = function () {
      if (fm_pplShowing.call(this)) return true;
      if (!document.getElementById('mapData')) return false;
      return !!document.querySelector('.toolbar .tool-button.sel[data-ev="tool_account"]');
    };
    feature_Reel.onPeople = function () {
      return !!document.querySelector('.toolbar .tool-button.sel[data-ev="tool_account"]');
    };
    const fm_pplPosts = feature_Reel.posts;
    feature_Reel.posts = function () {
      const out = fm_pplPosts.call(this);
      const data = document.getElementById('mapData');
      // a place from the card's own block, wherever the pins came from
      let cards = [];
      try { cards = JSON.parse(JSON.parse(feature_Loop.state || '{}').cards || '[]'); } catch (e) { cards = []; }
      const byId = {};
      for (const c of cards) if (c && c.id) byId[c.id] = c;
      const placeOf = (c) => {
        for (const b of (c && Array.isArray(c.blocks) ? c.blocks : [])) {
          if (b && b.kind === 'location' && isFinite(b.lat) && isFinite(b.lon)) return [b.lat, b.lon];
        }
        return null;
      };
      if (!this.onPeople()) {
        for (const p of out) if (!p.at) p.at = placeOf(byId[p.id]);
        return out;
      }
      // the people map: the posts set, not the people the map drew
      const ids = data ? data.getAttribute('data-post-ids') : null;
      if (ids === null) return out;
      const allowed = new Set(ids.split(',').filter((x) => x));
      const posts = [];
      for (const c of cards) {
        if (!c || c.type !== 'post' || !allowed.has(c.id)) continue;
        const blocks = Array.isArray(c.blocks) ? c.blocks : [];
        const pic = blocks.find((b) => b && b.kind === 'picture' && typeof b.data === 'string' && b.data);
        const text = blocks.find((b) => b && b.kind === 'text' && b.text);
        const title = blocks.find((b) => b && b.kind === 'title' && b.text);
        posts.push({
          id: c.id, owner: c.owner || '',
          t: (typeof c.when === 'number' && c.when) || c.created || 0,
          face: pic ? pic.data : '',
          words: (title && title.text ? title.text + ' — ' : '') + (text ? text.text : ''),
          at: placeOf(c),
        });
      }
      posts.sort((a, b) => b.t - a.t);
      return posts;
    };
  }
}
