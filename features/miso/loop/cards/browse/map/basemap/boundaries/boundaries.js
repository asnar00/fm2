const feature_Boundaries = {
  // the patch, drawn under the pins. Everything here is data-driven: the file
  // says which lines exist, what they are called and where a name sits, so a
  // later "highlight my ward" or "colour wards by coverage" reads the same
  // features rather than re-deriving them.
  FILE: 'map/boundaries.geojson',
  PANE: 'misoBoundaryLabels',
  NAMES_FROM: 9,        // below this zoom the names are soup, so they go away
  OVERHANG: 2.2,        // how much wider than its ward a name may be

  data: null,
  lines: null,          // the L.geoJSON layer: constituency + wards
  labels: null,         // an L.layerGroup of one divIcon per ward
  named: [],            // {name, marker, bounds} per ward, for the declutter
  loading: false,
  failed: 0,
  credited: false,
  fitted: false,

  // ---- the seam ---------------------------------------------------------
  // /map's sync() runs on every paint and is the moment the map is known to
  // exist and to have been measured. This runs straight after it. Idempotent
  // by design: the layers are made once and then live on the map object,
  // which /map never destroys, so leaving the map view and coming back
  // redraws nothing.

  ensure() {
    if (typeof L === 'undefined') return;
    if (typeof feature_Map === 'undefined' || !feature_Map.map) return;
    if (this.lines) { this.fit(); return; }
    this.load();
  },

  // one fetch, from our own site — never a third party, so the map keeps the
  // offline promise /tiles made. A failure is not fatal and not permanent:
  // the next paint tries again, a few times, and then stops asking.
  load() {
    if (this.loading || this.failed > 3) return;
    this.loading = true;
    fetch(this.FILE)
      .then((r) => (r.ok ? r.json() : null))
      .then((d) => {
        this.loading = false;
        if (!d || !d.features || !d.features.length) { this.failed++; return; }
        this.data = d;
        this.draw();
      })
      .catch(() => { this.loading = false; this.failed++; });
  },

  // ---- the lines --------------------------------------------------------

  draw() {
    const map = feature_Map.map;
    // a pane of our own between the ground and the pins: Leaflet puts paths at
    // 400 and markers at 600, and a ward's name belongs above its line and
    // under every face.
    if (!map.getPane(this.PANE)) {
      const pane = map.createPane(this.PANE);
      pane.style.zIndex = 450;
      pane.style.pointerEvents = 'none';
    }
    const box = {};    // code -> the ward's own bounds, for the declutter
    this.lines = L.geoJSON(this.data, {
      // furniture, not content: the boundary must never eat a drag or a tap
      // meant for a pin or for the map itself
      interactive: false,
      style: (f) => this.styleOf(f),
      onEachFeature: (f, layer) => {
        const p = f.properties || {};
        if (p.kind === 'ward' && p.code) box[p.code] = layer.getBounds();
      },
    }).addTo(map);

    this.labels = L.layerGroup([], { pane: this.PANE });
    this.named = [];
    for (const f of this.data.features) {
      const p = f.properties || {};
      if (p.kind !== 'ward' || !p.label || !p.name) continue;
      const marker = L.marker([p.label[1], p.label[0]], {
        pane: this.PANE,
        interactive: false,
        keyboard: false,
        icon: L.divIcon({
          className: 'ward-label-icon',
          html: '<span class="ward-label">' + this.esc(p.name) + '</span>',
          iconSize: [0, 0],
        }),
      });
      this.labels.addLayer(marker);
      this.named.push({ name: p.name, marker: marker, bounds: box[p.code] });
    }
    this.labels.addTo(map);

    this.place();
    map.on('zoomend', () => this.place());
    map.on('moveend', () => this.place());
    this.credit();
    this.fit();
  },

  // the constituency is the edge of the whole patch and reads a step brighter
  // than the wards inside it. No fill, no colour: /taste 1 and 3.
  styleOf(f) {
    const kind = (f.properties || {}).kind;
    if (kind === 'constituency') {
      return { color: '#8b8b95', weight: 2, opacity: 0.9, fill: false,
               dashArray: '7 5', lineJoin: 'round' };
    }
    return { color: '#5c5c66', weight: 1, opacity: 0.75, fill: false,
             lineJoin: 'round' };
  },

  // ---- which names are shown ---------------------------------------------
  // Twenty-six names over a district that fits one phone screen is a pile, and
  // a pile is not a readable name. So the names are placed rather than merely
  // drawn: biggest ward first, and a name stands down if it is wider than the
  // ward it belongs to or if it would land on a name already placed. Zooming
  // in makes room and the rest arrive; the rule is the same at every zoom, so
  // what you see is always exactly what fits.
  place() {
    const map = feature_Map.map;
    const pane = map.getPane(this.PANE);
    if (!pane) return;
    if (map.getZoom() < this.NAMES_FROM) { pane.style.display = 'none'; return; }
    pane.style.display = '';

    const size = map.getSize();
    const taken = [];
    const seats = [];
    for (const w of this.named) {
      const el = w.marker.getElement();
      const span = el ? el.firstElementChild : null;
      if (!span) continue;
      span.style.visibility = 'hidden';        // hidden, so it is still measurable
      const p = map.latLngToContainerPoint(w.marker.getLatLng());
      const half = span.offsetWidth / 2;
      const tall = span.offsetHeight / 2;
      // how wide the ward itself is on screen right now
      let room = Infinity;
      if (w.bounds) {
        const a = map.latLngToContainerPoint(w.bounds.getNorthWest());
        const b = map.latLngToContainerPoint(w.bounds.getSouthEast());
        room = Math.abs(b.x - a.x);
      }
      seats.push({ span: span, room: room,
                   box: [p.x - half - 3, p.y - tall - 2,
                         p.x + half + 3, p.y + tall + 2] });
    }
    // the biggest ward's name is the one most worth having
    seats.sort((a, b) => b.room - a.room);
    for (const s of seats) {
      const [x0, y0, x1, y1] = s.box;
      if (x1 < 0 || y1 < 0 || x0 > size.x || y0 > size.y) continue;  // off screen
      // a name may overhang its ward — every paper map's do — but not by so
      // much that you cannot tell which shape it belongs to
      if (s.room * this.OVERHANG < x1 - x0) continue;
      let clash = false;
      for (const t of taken) {
        if (x0 < t[2] && x1 > t[0] && y0 < t[3] && y1 > t[1]) { clash = true; break; }
      }
      if (clash) continue;
      taken.push(s.box);
      s.span.style.visibility = 'visible';
    }
  },

  // OGL v3 asks for the credit. The line comes out of the data file, so a
  // different source ships its own words rather than needing this file edited.
  credit() {
    if (this.credited || !feature_Map.map.attributionControl) return;
    const line = (this.data.credit || this.data.attribution || '').trim();
    if (!line) return;
    feature_Map.map.attributionControl.addAttribution(line);
    this.credited = true;
  },

  // nothing you hold has a place yet: the patch itself is somewhere to be, so
  // the map opens on the constituency instead of on the whole world. Once
  // only, and never when there are pins — /map has already fitted those — and
  // the device's own position, if it arrives, still wins.
  fit() {
    if (this.fitted || !this.lines) return;
    if (feature_Map.fitted || (feature_Map.markers || []).length) return;
    this.fitted = true;
    try {
      feature_Map.map.fitBounds(this.lines.getBounds().pad(0.02),
                                { animate: false });
    } catch (e) {
    }
  },

  esc(s) {
    return String(s).replace(/&/g, '&amp;').replace(/</g, '&lt;')
      .replace(/>/g, '&gt;').replace(/"/g, '&quot;');
  },
};

{
  // /map's own sync seam, taken the way /map took /loop's paint: property
  // replacement at load, not a timer (notes.md, "the apply-wrapper race").
  // With /map unticked there is no map to draw on and this does nothing.
  if (typeof feature_Map !== 'undefined') {
    const fm_bSync = feature_Map.sync;
    feature_Map.sync = function () {
      fm_bSync.call(this);
      try {
        feature_Boundaries.ensure();
      } catch (e) {
      }
    };
  }
}
