const feature_Region = {
  // the region, drawn twice: a second ground under a mask shaped like the
  // chosen polygon, and a page of pills to choose it with. Everything about
  // WHICH region comes from /boundaries' geojson and from the `#misoRegion`
  // marker the Rust half writes — nothing here knows a ward's name.
  PANE: 'misoRegionTiles',
  URL: 'tiles/outdoors/{z}/{x}/{y}.png',
  // 'css' — clip-path: path() straight on the pane. 'svg' — the same outline
  // as an SVG <clipPath> the pane points at. Measured against each other on
  // WebKit at DPR 3 before this constant was set; see region.md.
  CLIP: 'css',
  ID: 'misoRegionClip',

  layer: null,
  built: null,      // the region code the layer's bounds were made for
  data: null,       // our own copy of the file, only if /boundaries has none
  loading: false,
  key: '',          // zoom + pixel origin + region, so a pan does no work
  svg: null,
  path: null,

  // ---- what the page says ------------------------------------------------
  // the marker /region's render writes on every paint. Not a bridged state
  // key: one of those is republished at /payload's older link and would be a
  // turn behind a write made from this node (misses.md, "navigation from the
  // wrong side").

  chosen() {
    const el = document.getElementById('misoRegion');
    return el ? (el.getAttribute('data-code') || '') : '';
  },

  // /boundaries holds the parsed file once the map has painted. The region
  // page can be reloaded straight into with no map behind it, so this node
  // can fetch the file itself — into its OWN slot, never into a sibling's.
  file() {
    if (typeof feature_Boundaries !== 'undefined' && feature_Boundaries.data) {
      return feature_Boundaries.data;
    }
    return this.data;
  },

  load() {
    if (this.loading || this.data) return;
    if (typeof feature_Boundaries === 'undefined') return;
    this.loading = true;
    fetch(feature_Boundaries.FILE)
      .then((r) => (r.ok ? r.json() : null))
      .then((d) => {
        this.loading = false;
        if (!d || !d.features || !d.features.length) return;
        this.data = d;
        this.fill();
        this.safe();      // the file has landed: build the ground now, not at
                          // whatever repaint happens to come next
      })
      .catch(() => { this.loading = false; });
  },

  // the chosen feature, or the constituency: an empty code means the whole
  // patch, and so does a code the file has never heard of — a ward that was
  // renamed out of the data must not leave the map with no ground at all.
  featureFor(code) {
    const d = this.file();
    if (!d || !d.features) return null;
    let whole = null;
    let hit = null;
    for (const f of d.features) {
      const p = f.properties || {};
      if (p.kind === 'constituency' && !whole) whole = f;
      if (code && p.code === code) hit = f;
    }
    return hit || whole;
  },

  // ---- the second ground -------------------------------------------------

  ensure() {
    if (typeof L === 'undefined') return;
    if (typeof feature_Map === 'undefined' || !feature_Map.map) return;
    const map = feature_Map.map;
    if (!map.getPane(this.PANE)) {
      // Leaflet: tiles at 200, paths at 400, markers at 600. The second
      // ground goes just above the first and under the boundary lines, so the
      // line that marks the region is drawn ON the region, never under it.
      const pane = map.createPane(this.PANE);
      pane.style.zIndex = 250;
      pane.style.pointerEvents = 'none';
      pane.style.display = 'none';        // nothing is shown before it is cut
      // the map's own beats, taken once with the pane, and calling ensure()
      // rather than cut(): when the pane is made the geojson may still be on
      // its way, so a gesture has to be able to BUILD the second ground and
      // not only re-cut one that already exists. Without this the ground waits
      // for the page's next repaint, which on a quiet map may not come — the
      // rig caught exactly that: a map opened, panned and zoomed with no
      // outdoors squares on it at all until an unrelated tap repainted.
      map.on('zoomend', () => this.safe());
      map.on('moveend', () => this.safe());
      map.on('viewreset', () => this.safe());
    }
    const f = this.featureFor(this.chosen());
    if (!f) { this.off(); this.load(); return; }
    // no box means no rings means nothing to cut, and a tile layer made with
    // a null `bounds` is an UNBOUNDED one: Leaflet's own check reads a falsy
    // bounds as "every tile is valid", so a malformed geometry would ask
    // Stadia for a screenful of the world before cut() could take the layer
    // away again. The box is therefore found before the layer is made.
    const box = this.boundsOf(f);
    if (!box) { this.off(); return; }
    const code = (f.properties || {}).code || '';
    if (!this.layer) {
      // the layer is made only once a region is known, and it is BOUNDED to
      // that region: Leaflet asks for no square outside the box, so a mask the
      // size of one ward costs one ward's worth of a metered tile budget
      // rather than a screenful.
      let url = this.URL;
      if (typeof feature_FreshTiles !== 'undefined' && feature_FreshTiles.TAG) {
        url += '?' + feature_FreshTiles.TAG;
      }
      this.layer = L.tileLayer(url, {
        pane: this.PANE,
        maxZoom: 19,
        bounds: box,
        keepBuffer: 1,
        updateWhenIdle: true,
      }).addTo(map);
      this.built = code;
    } else if (code !== this.built) {
      // a different region: the box moves with it, and redraw() drops the
      // squares of the old one and asks _isValidTile about the new
      this.layer.options.bounds = box;
      this.built = code;
      this.layer.redraw();
    }
    this.cut(f);
  },

  safe() {
    try {
      this.ensure();
    } catch (e) {
    }
  },

  boundsOf(f) {
    let lo = null;
    let hi = null;
    this.rings(f).forEach((ring) => {
      for (const c of ring) {
        if (!lo) { lo = [c[1], c[0]]; hi = [c[1], c[0]]; continue; }
        if (c[1] < lo[0]) lo[0] = c[1];
        if (c[0] < lo[1]) lo[1] = c[0];
        if (c[1] > hi[0]) hi[0] = c[1];
        if (c[0] > hi[1]) hi[1] = c[0];
      }
    });
    if (!lo) return null;
    return L.latLngBounds(lo, hi);
  },

  // a Polygon's rings, or every ring of every part of a MultiPolygon — the
  // file carries both shapes, and a ward with a detached part or a hole in it
  // is an ordinary ward, not a special case. Ring 0 of each part is its
  // outside and the rest are its holes; the even-odd rule below cuts them out
  // without this code having to know which is which.
  rings(f) {
    const g = (f || {}).geometry || {};
    if (g.type === 'Polygon') return g.coordinates || [];
    if (g.type === 'MultiPolygon') {
      let out = [];
      for (const part of g.coordinates || []) out = out.concat(part);
      return out;
    }
    return [];
  },

  // ---- the mask ----------------------------------------------------------
  // The outline is written in the PANE's own coordinate space, which is
  // exactly what latLngToLayerPoint returns. That space is why this works at
  // all: Leaflet carries a pan as a translate on the map pane and a zoom
  // animation as a translate plus a scale, and a clip-path applies before an
  // ancestor's transform — so the cut travels and scales with the very tiles
  // it is cutting, and only has to be rewritten when the pixel origin moves.
  // Hence the key: a drag recomputes nothing.

  cut(f) {
    const map = feature_Map.map;
    const pane = map.getPane(this.PANE);
    if (!pane) return;
    f = f || this.featureFor(this.chosen());
    if (!f) { this.off(); return; }
    const o = map.getPixelOrigin();
    const key = map.getZoom() + '/' + o.x + '/' + o.y + '/'
      + ((f.properties || {}).code || '');
    if (key === this.key) return;
    const d = this.outline(f);
    if (!d) { this.off(); return; }
    this.key = key;
    if (this.CLIP === 'svg') {
      this.svgCut(pane, d);
    } else {
      pane.style.clipPath = 'path(evenodd, "' + d + '")';
      pane.style.webkitClipPath = 'path(evenodd, "' + d + '")';
    }
    pane.style.display = '';
  },

  outline(f) {
    const map = feature_Map.map;
    let d = '';
    for (const ring of this.rings(f)) {
      if (!ring || ring.length < 3) continue;
      for (let i = 0; i < ring.length; i++) {
        const p = map.latLngToLayerPoint(L.latLng(ring[i][1], ring[i][0]));
        d += (i ? 'L' : 'M') + p.x.toFixed(1) + ' ' + p.y.toFixed(1);
      }
      d += 'Z';
    }
    return d;
  },

  // the same outline as a real SVG clipPath in user space. Measured against
  // the CSS road on WebKit at DPR 3: byte-identical output, so the choice was
  // made on cost and the CSS one ships. Kept because if WebKit ever changes
  // its mind about one of them, the other is the CLIP constant away.
  svgCut(pane, d) {
    if (!this.svg) {
      const NS = 'http://www.w3.org/2000/svg';
      const svg = document.createElementNS(NS, 'svg');
      svg.setAttribute('width', '0');
      svg.setAttribute('height', '0');
      svg.style.position = 'absolute';
      const cp = document.createElementNS(NS, 'clipPath');
      cp.setAttribute('id', this.ID);
      cp.setAttribute('clipPathUnits', 'userSpaceOnUse');
      const p = document.createElementNS(NS, 'path');
      p.setAttribute('clip-rule', 'evenodd');
      cp.appendChild(p);
      svg.appendChild(cp);
      document.body.appendChild(svg);
      this.svg = svg;
      this.path = p;
    }
    this.path.setAttribute('d', d);
    pane.style.clipPath = 'url(#' + this.ID + ')';
    pane.style.webkitClipPath = 'url(#' + this.ID + ')';
  },

  // no region, no file, no outline: the second ground comes off entirely and
  // the map is the map it was. The layer is REMOVED rather than hidden — a
  // hidden Leaflet layer still asks for its squares, and these are metered.
  off() {
    if (typeof feature_Map === 'undefined' || !feature_Map.map) return;
    const map = feature_Map.map;
    const pane = map.getPane(this.PANE);
    if (pane) { pane.style.display = 'none'; }
    if (this.layer && map.hasLayer(this.layer)) map.removeLayer(this.layer);
    this.layer = null;
    this.built = null;
    this.key = '';
  },

  // ---- the pills ---------------------------------------------------------
  // The container is the Rust half's; the rows are the file's. `render` is
  // compiled to wasm as well as to the server and cannot read a file at all,
  // which is the honest reason the names are drawn here — and the same reason
  // /boundaries gives for the file being the seam.

  fill() {
    const box = document.getElementById('regionPills');
    if (!box) return;
    const d = this.file();
    if (!d || !d.features) { this.load(); return; }
    const now = this.featureFor(this.chosen());
    const at = ((now || {}).properties || {}).code || '';
    const rows = [];
    for (const f of d.features) {
      const p = f.properties || {};
      if (p.kind === 'constituency') {
        rows.push({ pick: '', code: p.code || '', name: p.name || 'the whole patch',
                    whole: true });
      }
    }
    const wards = [];
    for (const f of d.features) {
      const p = f.properties || {};
      if (p.kind === 'ward' && p.code) {
        wards.push({ pick: p.code, code: p.code, name: p.name || p.code });
      }
    }
    wards.sort((a, b) => a.name.localeCompare(b.name));
    let html = '';
    for (const r of rows.concat(wards)) {
      html += '<div class="region-pill' + (r.whole ? ' whole' : '')
        + (r.code === at ? ' sel' : '') + '" data-ev="region_pick:'
        + this.esc(r.pick) + '">' + this.esc(r.name) + '</div>';
    }
    if (html === box.innerHTML) return;   // a repaint that changes nothing
    box.innerHTML = html;
  },

  esc(s) {
    return String(s).replace(/&/g, '&amp;').replace(/</g, '&lt;')
      .replace(/>/g, '&gt;').replace(/"/g, '&quot;');
  },

  paint() {
    this.fill();
    this.ensure();
  },
};

{
  // no pencil on the region page. /editing/toolbar draws its edit control for
  // whatever feature_Editing.page() answers, and the region page is a
  // .card-page too — it borrows the card's ground, its insets and its scroll
  // rather than restating them. Answer nothing for it, and the toolbar's own
  // apply() takes the button away on the next paint; card pages are untouched.
  // This is /doors' idiom for the invite page, one node along (#p70): the node
  // that owns the page is the node that opts it out.
  if (typeof feature_Editing !== 'undefined' && feature_Editing.page) {
    const fm_rPageWas = feature_Editing.page.bind(feature_Editing);
    feature_Editing.page = function () {
      const p = fm_rPageWas();
      return (p && p.classList.contains('region-page')) ? null : p;
    };
  }
}

{
  // the long-press word, put into /tool-words' own table from here rather
  // than into its file: a tool's line belongs with the tool, and this way it
  // arrives and leaves with the node. `region` is keyed in TOOLS because the
  // button's event is `tool_region` — it opens a level, it does not act.
  if (typeof feature_ToolWords !== 'undefined' && feature_ToolWords.TOOLS) {
    feature_ToolWords.TOOLS.region = {
      name: 'region',
      intro: 'The patch you are working: the whole constituency, or one ward. It is drawn in outdoor colours so you can see where you are.',
    };
  }
}

{
  // /loop's paint seam by property replacement at load — /map's own idiom, and
  // taken AFTER /map so the map is mounted and synced by the time this runs.
  // With /loop or /map absent there is nothing to draw on and nothing happens.
  if (typeof feature_Loop !== 'undefined') {
    const fm_rPaint = feature_Loop.paint;
    feature_Loop.paint = function (html) {
      fm_rPaint.call(this, html);
      try {
        feature_Region.paint();
      } catch (e) {
      }
    };
  }
}
