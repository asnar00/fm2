const feature_Reel = {
  host: null, list: null, sig: '', settle: null, HEIGHT: 108,

  make() {
    if (this.host) return;
    const h = document.createElement('div');
    h.id = 'mapReel';
    h.innerHTML = '<div class="reel-list"></div>';
    document.body.appendChild(h);
    this.host = h;
    this.list = h.querySelector('.reel-list');
    const self = this;
    this.list.addEventListener('scroll', () => {
      clearTimeout(self.settle);
      self.settle = setTimeout(() => self.follow(), 140);
    }, { passive: true });
  },

  // the posts tool's map view, and nothing else — read from the screen, not
  // the state mirror: `open_tool` is rewritten at the tool's own link after
  // the mirror was published, so the frame after a way-back tap says "" while
  // the row already shows posts selected (the band vanished on that frame,
  // #p19; the tour learned the same, one-level review)
  showing() {
    if (!document.getElementById('mapData')) return false;
    return !!document.querySelector('.toolbar .tool-button.sel[data-ev="tool_posts"]');
  },

  // the world's posts, newest first, with the place /map gave each pin
  posts() {
    let cards = [];
    try { cards = JSON.parse(JSON.parse(feature_Loop.state || '{}').cards || '[]'); } catch (e) { cards = []; }
    const places = {};
    try {
      const data = document.getElementById('mapData');
      for (const p of JSON.parse(data.getAttribute('data-pins') || '[]')) {
        if (typeof p.lat === 'number' && typeof p.lon === 'number') places[p.id] = [p.lat, p.lon];
      }
    } catch (e) { /* no places */ }
    // the tool's own set, when the page says which — a post the current
    // project sifts out of the map is not in the band either (#p22)
    let allowed = null;
    try {
      const data = document.getElementById('mapData');
      const ids = data ? data.getAttribute('data-ids') : null;
      if (ids !== null) allowed = new Set(ids.split(',').filter((x) => x));
    } catch (e) { allowed = null; }
    const out = [];
    for (const c of cards) {
      if (!c || c.type !== 'post') continue;
      if (allowed && !allowed.has(c.id)) continue;
      const blocks = Array.isArray(c.blocks) ? c.blocks : [];
      const pic = blocks.find((b) => b && b.kind === 'picture' && typeof b.data === 'string' && b.data);
      const text = blocks.find((b) => b && b.kind === 'text' && b.text);
      const title = blocks.find((b) => b && b.kind === 'title' && b.text);
      out.push({
        id: c.id, owner: c.owner || '',
        t: (typeof c.when === 'number' && c.when) || c.created || 0,
        face: pic ? pic.data : '',
        words: (title && title.text ? title.text + ' — ' : '') + (text ? text.text : ''),
        at: places[c.id] || null,
      });
    }
    out.sort((a, b) => b.t - a.t);
    return out;
  },

  when(t) {
    if (!t) return '';
    const d = new Date(t), now = new Date();
    const hm = d.getHours() + ':' + String(d.getMinutes()).padStart(2, '0');
    const sameDay = d.toDateString() === now.toDateString();
    if (sameDay) return hm;
    const days = (now - d) / 86400000;
    if (days < 6) return d.toLocaleDateString(undefined, { weekday: 'short' }) + ' ' + hm;
    return d.toLocaleDateString(undefined, { day: 'numeric', month: 'short' }) + ' ' + hm;
  },

  esc(s) {
    return String(s).replace(/&/g, '&amp;').replace(/</g, '&lt;')
      .replace(/>/g, '&gt;').replace(/"/g, '&quot;');
  },

  render() {
    this.make();
    const on = this.showing();
    const posts = on ? this.posts() : [];
    document.body.classList.toggle('fm-reel', on && posts.length > 0);
    this.host.style.display = on && posts.length ? 'block' : 'none';
    if (!on || !posts.length) { this.sig = ''; return; }
    const sig = posts.map((p) => p.id).join(',');
    if (sig === this.sig) return;
    this.sig = sig;
    let html = '';
    for (const p of posts) {
      const face = p.face
        ? '<img src="' + this.esc(p.face) + '" alt="">'
        : '<span>' + this.esc((p.owner || '?').slice(0, 1).toUpperCase()) + '</span>';
      html += '<div class="reel-post" data-ev="browse_open:' + this.esc(p.id) + '"'
        + (p.at ? ' data-lat="' + p.at[0] + '" data-lon="' + p.at[1] + '"' : '') + '>'
        + '<div class="reel-face">' + face + '</div>'
        + '<div class="reel-body"><div class="reel-words">' + this.esc(p.words) + '</div>'
        + '<div class="reel-meta">' + this.esc(p.owner) + ' · ' + this.esc(this.when(p.t)) + '</div></div></div>';
    }
    this.list.innerHTML = html;
    this.list.scrollLeft = 0;
    if (typeof feature_Map !== 'undefined' && feature_Map.map) {
      try { feature_Map.map.invalidateSize(); } catch (e) { /* not yet */ }
    }
  },

  // the lozenge at the left edge is the current one
  current() {
    const left = this.list.scrollLeft;
    let best = null, dist = Infinity;
    for (const el of this.list.querySelectorAll('.reel-post')) {
      const d = Math.abs(el.offsetLeft - left);
      if (d < dist) { dist = d; best = el; }
    }
    return best;
  },

  follow() {
    const el = this.current();
    if (!el) return;
    const lat = parseFloat(el.getAttribute('data-lat')), lon = parseFloat(el.getAttribute('data-lon'));
    if (!isFinite(lat) || !isFinite(lon)) return;
    this.pan(lat, lon);
  },

  // where the map goes for a place: the seam a node that aims differently
  // redefines, leaving follow (and what wraps it) alone
  pan(lat, lon) {
    if (typeof feature_Map === 'undefined' || !feature_Map.map) return;
    try { feature_Map.map.panTo([lat, lon], { animate: true, duration: 0.45 }); } catch (e) { /* mid-mount */ }
  },
};

{
  // after /map's own sync, which is where the map is shown or hidden
  if (typeof feature_Map !== 'undefined') {
    const fm_reelSync = feature_Map.sync;
    feature_Map.sync = function () {
      fm_reelSync.call(this);
      try { feature_Reel.render(); } catch (e) { /* the map is untouched */ }
    };
  }
}
