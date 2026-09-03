{
  if (typeof feature_Reel !== 'undefined' && typeof feature_Live !== 'undefined') {
    feature_Reel.liveRows = [];
    const fm_lvPosts = feature_Reel.posts;
    feature_Reel.posts = function () {
      if (typeof this.onPeople !== 'function' || !this.onPeople()) return fm_lvPosts.call(this);
      let cards = [];
      try { cards = JSON.parse(JSON.parse(feature_Loop.state || '{}').cards || '[]'); } catch (e) { cards = []; }
      const byId = {};
      for (const c of cards) if (c && c.id) byId[c.id] = c;
      const out = [];
      for (const r of this.liveRows) {
        if (!r || typeof r.lat !== 'number' || typeof r.lon !== 'number') continue;
        const c = r.id ? byId[r.id] : null;
        const blocks = c && Array.isArray(c.blocks) ? c.blocks : [];
        const pic = blocks.find((b) => b && b.kind === 'picture' && typeof b.data === 'string' && b.data);
        const text = blocks.find((b) => b && b.kind === 'text' && b.text);
        const title = blocks.find((b) => b && b.kind === 'title' && b.text);
        out.push({
          id: r.id || ('live:' + r.name), owner: r.name || '',
          t: r.t || Date.now(),
          face: (pic && pic.data) || r.face || '',
          words: (title && title.text ? title.text : (r.name || '')) + (text ? ' — ' + text.text : ''),
          at: [r.lat, r.lon],
        });
      }
      out.sort((a, b) => b.t - a.t);
      return out;
    };
    // the live pin carries its card's id, so the ring finds it
    const fm_lvPin = feature_Live.pinHtml;
    if (typeof fm_lvPin === 'function') {
      feature_Live.pinHtml = function (p) {
        const html = fm_lvPin.call(this, p);
        const open = '<div class="map-pin';
        if (!p || !p.id || html.indexOf(open) !== 0) return html;
        const end = html.indexOf('>');
        return html.slice(0, end) + ' data-id="' + String(p.id).replace(/"/g, '&quot;') + '"' + html.slice(end);
      };
    }
    const fm_lvSig = (rows) => rows.map((r) => r.id || r.name).sort().join(',');
    const fm_lvDraw = feature_Live.draw;
    feature_Live.draw = function (rows) {
      fm_lvDraw.call(this, rows);
      try {
        const before = fm_lvSig(feature_Reel.liveRows);
        feature_Reel.liveRows = Array.isArray(rows) ? rows.slice() : [];
        if (!feature_Reel.onPeople || !feature_Reel.onPeople()) return;
        if (fm_lvSig(feature_Reel.liveRows) !== before) {
          feature_Reel.sig = '';
          feature_Reel.render();
          return;
        }
        // only places moved: the lozenges follow, the mark is renewed
        if (!feature_Reel.list) return;
        for (const r of feature_Reel.liveRows) {
          const el = feature_Reel.list.querySelector('.reel-post[data-ev="browse_open:' + (r.id || 'live:' + r.name) + '"]');
          if (el) { el.setAttribute('data-lat', r.lat); el.setAttribute('data-lon', r.lon); }
        }
        if (typeof feature_Reel.mark === 'function') feature_Reel.mark();
      } catch (e) { /* the pins stand */ }
    };
    const fm_lvClear = feature_Live.clear;
    feature_Live.clear = function () {
      fm_lvClear.call(this);
      feature_Reel.liveRows = [];
      try { if (feature_Reel.onPeople && feature_Reel.onPeople()) { feature_Reel.sig = ''; feature_Reel.render(); } } catch (e) { /* as it was */ }
    };
  }
}
