const feature_Stocked = {
  // ---- the plan ------------------------------------------------------------
  // the whole area at these zooms, and one small box at the centre a step
  // deeper: a canvasser with no signal gets the district sharp and the town
  // centre at street level. Per-area zoom plans are the anticipated next ask
  // ("stock my ward at street level"): a second entry in zoomsFor().
  ZOOMS: [12, 13, 14, 15],
  CENTRE_ZOOM: 16,
  CENTRE_KM: 1.5,          // half a side: a 3 x 3 km box at the area's centre
  CAP: 1500,               // squares per run; ~12 KB each on the dark ground
  LANES: 4,                // in flight at once
  PAUSE_MS: 300,           // between batches
  KEY: 'miso.stocked',     // the localStorage record: keys and counts, never a position
  // the last resort: Sevenoaks District. Over the cap at zoom 15, so a run
  // on this box stops at CAP with the low zooms whole and 15 partial.
  BOX: { w: 0.03, s: 51.13, e: 0.35, n: 51.44 },

  running: false,
  plan: null,              // [[z, x, y], ...] in zoom order, the centre box last
  record: null,            // {key, done, total, full, at}
  ctl: null,               // the AbortController of the batch in flight
  waited: 0,               // paints spent waiting for /boundaries to load
  fetched: 0,              // this page's own count, for a rig to read

  // ---- when it may run ------------------------------------------------------
  // seen, online, and the map view up (#mapData is /map's sign) — the same
  // predicate /live uses, plus the wire. Any of them false ends the run
  // between batches; the record keeps its place and the next chance resumes.

  onMap() {
    return !!document.getElementById('mapData')
      && typeof feature_Map !== 'undefined' && !!feature_Map.map;
  },

  may() {
    if (typeof document === 'undefined' || document.visibilityState !== 'visible') return false;
    if (typeof navigator !== 'undefined' && navigator.onLine === false) return false;
    return this.onMap();
  },

  // where the squares go: the service worker's cache, by its own name. With no
  // Cache API there is nowhere to stock and nothing runs.
  CACHE: 'miso',
  canStore() {
    return typeof caches !== 'undefined' && !!caches.open;
  },

  // the platform's word on the wire. iOS has no navigator.connection, so this
  // is false there and the cap is the protection; where it exists, a metered
  // or cellular link stocks the low zooms only and leaves 15 and 16 for wifi.
  metered() {
    const c = typeof navigator !== 'undefined' && navigator.connection;
    if (!c) return false;
    return !!(c.saveData || c.type === 'cellular');
  },

  // ---- the ground tag ---------------------------------------------------------
  // the urls must be the ones the map itself asks for, or the cache answers
  // nothing: /fresh-tiles stamps `?g=N` on the tile layer, and the same tag
  // goes on every url here. Without /fresh-tiles the map asks bare and so
  // does this.
  tag() {
    if (typeof feature_FreshTiles === 'undefined') return '';
    return String(feature_FreshTiles.TAG || '');
  },

  url(t) {
    const q = this.tag();
    return 'tiles/' + t[0] + '/' + t[1] + '/' + t[2] + '.png' + (q ? '?' + q : '');
  },

  // ---- the area ----------------------------------------------------------------
  // the patch /boundaries draws (the constituency feature of its file) is the
  // project's area today; a project card carrying its own boundary is the
  // named next reading, and it lands here. Failing the patch, the box round
  // the pins on the map; failing pins, the district box. null means "not
  // yet": /boundaries is still fetching its file and a later paint will know.

  area() {
    if (typeof feature_Boundaries !== 'undefined') {
      const d = feature_Boundaries.data;
      if (d && Array.isArray(d.features)) {
        for (const f of d.features) {
          const p = f.properties || {};
          if (p.kind !== 'constituency' || !f.geometry) continue;
          const box = this.bboxOf(f.geometry);
          if (box) return { key: 'patch:' + (p.code || p.name || 'constituency'), box: box };
        }
      } else if (feature_Boundaries.failed <= 3 && this.waited < 12) {
        this.waited++;
        return null;
      }
    }
    const ms = (typeof feature_Map !== 'undefined' && feature_Map.markers) || [];
    const at = [];
    for (const m of ms) {
      try { const ll = m.getLatLng(); at.push([ll.lng, ll.lat]); } catch (e) { }
    }
    if (at.length) {
      const box = this.bboxOf({ type: 'MultiPoint', coordinates: at });
      if (box) {
        // padded to a couple of kilometres, so one pin still buys its street
        box.w -= 0.03; box.e += 0.03; box.s -= 0.02; box.n += 0.02;
        // the key is the zoom-12 square range (10 km squares), never a position
        const a = this.tile(box.n, box.w, 12);
        const b = this.tile(box.s, box.e, 12);
        return { key: 'pins:' + a[0] + '-' + b[0] + ',' + a[1] + '-' + b[1], box: box };
      }
    }
    return { key: 'district', box: Object.assign({}, this.BOX) };
  },

  bboxOf(g) {
    let w = Infinity, s = Infinity, e = -Infinity, n = -Infinity;
    const walk = (c) => {
      if (typeof c[0] === 'number') {
        if (c[0] < w) w = c[0]; if (c[0] > e) e = c[0];
        if (c[1] < s) s = c[1]; if (c[1] > n) n = c[1];
        return;
      }
      for (const k of c) walk(k);
    };
    try { walk(g.coordinates || []); } catch (x) { return null; }
    if (!isFinite(w) || !isFinite(n)) return null;
    return { w: w, s: s, e: e, n: n };
  },

  // ---- the squares ----------------------------------------------------------------

  tile(lat, lon, z) {
    const n = Math.pow(2, z);
    const x = Math.floor((lon + 180) / 360 * n);
    const r = lat * Math.PI / 180;
    const y = Math.floor((1 - Math.log(Math.tan(r) + 1 / Math.cos(r)) / Math.PI) / 2 * n);
    return [Math.min(Math.max(x, 0), n - 1), Math.min(Math.max(y, 0), n - 1)];
  },

  squares(box, z, out) {
    const a = this.tile(box.n, box.w, z);
    const b = this.tile(box.s, box.e, z);
    for (let y = a[1]; y <= b[1]; y++) {
      for (let x = a[0]; x <= b[0]; x++) out.push([z, x, y]);
    }
  },

  zoomsFor(area) {
    return this.ZOOMS;
  },

  build(area) {
    const out = [];
    for (const z of this.zoomsFor(area)) this.squares(area.box, z, out);
    const cy = (area.box.s + area.box.n) / 2;
    const cx = (area.box.w + area.box.e) / 2;
    const dlat = this.CENTRE_KM / 111.0;
    const dlon = this.CENTRE_KM / (111.0 * Math.cos(cy * Math.PI / 180));
    this.squares({ w: cx - dlon, s: cy - dlat, e: cx + dlon, n: cy + dlat }, this.CENTRE_ZOOM, out);
    return out.length > this.CAP ? out.slice(0, this.CAP) : out;
  },

  // ---- the record --------------------------------------------------------------------

  load() {
    try {
      const r = JSON.parse(localStorage.getItem(this.KEY) || 'null');
      if (r && typeof r.key === 'string' && typeof r.done === 'number') return r;
    } catch (e) { }
    return null;
  },

  save(r) {
    try { localStorage.setItem(this.KEY, JSON.stringify(r)); } catch (e) { }
  },

  // ---- the run -----------------------------------------------------------------------

  kick() {
    if (this.running) return;
    if (!this.may() || !this.canStore()) return;
    const area = this.area();
    if (!area) {
      if (!this.timer) this.timer = setTimeout(() => { this.timer = null; this.kick(); }, 1000);
      return;
    }
    const key = (this.tag() || 'g=0') + '|' + area.key;
    let rec = this.record || this.load();
    if (!rec || rec.key !== key) {
      this.plan = this.build(area);
      rec = { key: key, done: 0, total: this.plan.length, full: 0, at: 0 };
    } else if (!this.plan) {
      this.plan = this.build(area);
      rec.total = this.plan.length;
      if (rec.done > rec.total) rec.done = rec.total;
    }
    this.record = rec;
    this.run();
  },

  async run() {
    if (this.running) return;
    this.running = true;
    try {
      while (this.may()) {
        const rec = this.record;
        const plan = this.plan;
        let limit = plan.length;
        if (this.metered()) {
          // the low zooms go on any wire; 15 and the centre wait for wifi
          limit = 0;
          while (limit < plan.length && plan[limit][0] < 15) limit++;
        }
        if (rec.done >= limit) {
          if (rec.done >= plan.length && !rec.full) {
            rec.full = Date.now();
            this.save(rec);
            this.tell();
          }
          break;
        }
        const batch = plan.slice(rec.done, Math.min(rec.done + this.LANES, limit));
        this.ctl = typeof AbortController !== 'undefined' ? new AbortController() : null;
        const got = await Promise.all(batch.map((t) => this.one(t)));
        this.ctl = null;
        if (got.indexOf('net') >= 0) break;   // the wire went: keep our place
        rec.done += batch.length;
        rec.missed = (rec.missed || 0) + got.filter((g) => g === 'miss').length;
        rec.at = Date.now();
        this.save(rec);
        await new Promise((r) => setTimeout(r, this.PAUSE_MS));
      }
    } catch (e) {
      /* a run that throws is a run that stopped; the record keeps its place */
    } finally {
      this.running = false;
      this.ctl = null;
    }
  },

  // one square. The service worker's network-first policy stores every ok
  // answer it relays, so through a controlling worker the fetch is the cache
  // write; a page the worker does not yet control (its first load) puts the
  // answer into the same cache by hand. A 404 (a square the proxy could not
  // get) is 'miss': counted done, not retried this run. A failed fetch is
  // 'net': the run stops here.
  async one(t) {
    const url = this.url(t);
    try {
      const opts = {};
      if (this.ctl) opts.signal = this.ctl.signal;
      const r = await fetch(url, opts);
      this.fetched++;
      if (!r.ok) return 'miss';
      const sw = typeof navigator !== 'undefined' && navigator.serviceWorker;
      if (!(sw && sw.controller)) {
        try {
          const c = await caches.open(this.CACHE);
          await c.put(new Request(url), r.clone());
        } catch (e) { }
      }
      return 'ok';
    } catch (e) {
      return 'net';
    }
  },

  // hidden, offline, or off the map: cut the batch in flight
  halt() {
    if (this.ctl) { try { this.ctl.abort(); } catch (e) { } }
  },

  // ---- what to say ------------------------------------------------------------------

  text() {
    const r = this.record || this.load();
    if (!r) return 'stocked: nothing yet';
    const g = r.key.split('|')[0];
    const when = r.full ? new Date(r.full).toLocaleString() : 'never';
    const zooms = this.ZOOMS[0] + '–' + this.CENTRE_ZOOM;
    return 'stocked: ' + r.done + ' of ' + r.total + ' squares, zooms ' + zooms
      + ', ground ' + g + ', last full at ' + when
      + (r.missed ? ', ' + r.missed + ' missed' : '')
      + (this.running ? ' (running)' : '');
  },

  tell() {
    if (typeof feature_Engineer !== 'undefined' && feature_Engineer.refresh) {
      try { feature_Engineer.refresh(); } catch (e) { }
    }
  },
};

{
  // /map's sync seam, taken as /boundaries and /live take it: the property
  // replaced at load, never a timer (notes.md, "the apply-wrapper race").
  // Every paint asks: on the map, seen, online? then run; off it, halt.
  if (typeof feature_Map !== 'undefined') {
    const fm_stSync = feature_Map.sync;
    feature_Map.sync = function () {
      fm_stSync.call(this);
      try {
        if (feature_Stocked.onMap()) feature_Stocked.kick();
        else feature_Stocked.halt();
      } catch (e) {
      }
    };
  }

  // the ways of stopping and resuming: visibility and the wire
  document.addEventListener('visibilitychange', () => {
    if (document.visibilityState === 'visible') feature_Stocked.kick();
    else feature_Stocked.halt();
  });
  window.addEventListener('online', () => feature_Stocked.kick());
  window.addEventListener('offline', () => feature_Stocked.halt());
}
