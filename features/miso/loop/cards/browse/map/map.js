const feature_Map = {
  // the Leaflet instance and its host, both made once and kept for the life
  // of the page. The host lives OUTSIDE #app so the loop's wholesale repaint
  // cannot take the map away — /keep's #cardRemove idiom, for the same reason
  // at a larger size: re-mounting a map on every event would refetch tiles,
  // lose the pan and flicker.
  host: null,
  map: null,
  layer: null,
  markers: [],
  sig: '',      // the pins we last drew, so a repaint that changes nothing does nothing
  fitted: false,
  located: false,
  ATTRIB: '© OpenStreetMap contributors',

  // ---- what the page is showing ----------------------------------------
  // #mapData is Rust's whole contribution to the screen: an empty element
  // carrying the located cards. Present means "the map view is up".

  sync() {
    const data = document.getElementById('mapData');
    if (!data) { this.hide(); return; }
    this.show();
    if (!this.mount()) return;
    let pins = [];
    try { pins = JSON.parse(data.getAttribute('data-pins') || '[]'); } catch (e) { pins = []; }
    this.draw(pins);
    // the host is display:none until now, so Leaflet measured nothing
    this.map.invalidateSize();
  },

  show() {
    if (!this.host || this.host.style.display === 'block') return;
    this.host.style.display = 'block';
  },

  hide() {
    if (!this.host || this.host.style.display === 'none') return;
    this.host.style.display = 'none';
  },

  // ---- the map, made once ----------------------------------------------

  mount() {
    if (this.map) return true;
    if (typeof L === 'undefined' || !this.host) return false;   // vendored file missing: grey, not broken
    this.map = L.map(this.host, {
      zoomControl: false,
      attributionControl: true,
      // the pins carry the tap; a double tap is a zoom, and a stray double
      // tap on a pin should not also open the card
      tapTolerance: 20,
    });
    this.map.setView([51.2719, 0.1904], 3);
    this.map.attributionControl.setPrefix('');
    this.layer = L.tileLayer('tiles/{z}/{x}/{y}.png', {
      maxZoom: 19,
      attribution: this.ATTRIB,
      // one zoom's worth of what is on screen and no more: OSM's policy
      // forbids bulk prefetch and our proxy pays for every miss
      keepBuffer: 1,
      updateWhenIdle: true,
    }).addTo(this.map);
    this.credit();
    return true;
  },

  // the credit belongs to whatever source the server is proxying, so it is
  // asked for rather than assumed. One request, at mount; if it fails the
  // OpenStreetMap line stands, which is true of every source we would use.
  credit() {
    fetch('tiles/attribution')
      .then((r) => (r.ok ? r.text() : ''))
      .then((t) => {
        const line = (t || '').trim();
        if (!line || !this.layer) return;
        this.ATTRIB = line;
        this.map.attributionControl.removeAttribution(
          '© OpenStreetMap contributors');
        this.map.attributionControl.addAttribution(line);
      })
      .catch(() => {});
  },

  // ---- the pins ----------------------------------------------------------

  draw(pins) {
    const sig = JSON.stringify(pins.map((p) => [p.id, p.lat, p.lon, p.title, (p.face || '').length]));
    if (sig === this.sig) return;
    this.sig = sig;
    for (const m of this.markers) m.remove();
    this.markers = [];
    const at = [];
    for (const p of pins) {
      if (typeof p.lat !== 'number' || typeof p.lon !== 'number') continue;
      const m = L.marker([p.lat, p.lon], {
        icon: L.divIcon({
          className: 'map-pin-icon',
          html: this.pinHtml(p),
          iconSize: [40, 50],
          iconAnchor: [20, 50],
        }),
        title: p.title || '',
        keyboard: false,
      }).addTo(this.map);
      // the pin's own handler rather than the loop's delegated [data-ev]
      // listener: Leaflet stops the DOM event on its own markers, so the tap
      // is sent by hand — to the SAME event /browse already answers.
      m.on('click', () => {
        if (typeof feature_Loop !== 'undefined') {
          feature_Loop.send({ type: 'click', ev: 'browse_open:' + p.id });
        }
      });
      this.markers.push(m);
      at.push([p.lat, p.lon]);
    }
    if (!at.length) { this.locate(); return; }
    // fit once, and again only when the set of pins itself changes — a refit
    // on every repaint would fight the hand that just dragged the map
    this.map.fitBounds(L.latLngBounds(at).pad(0.25), {
      animate: false,
      maxZoom: 16,
    });
    this.fitted = true;
  },

  pinHtml(p) {
    const inner = p.face
      ? '<img src="' + this.esc(p.face) + '" alt="">'
      : '<span>' + this.esc(p.initial || '') + '</span>';
    return '<div class="map-pin"><div class="map-pin-face">' + inner
      + '</div><div class="map-pin-stem"></div></div>';
  },

  esc(s) {
    return String(s).replace(/&/g, '&amp;').replace(/</g, '&lt;')
      .replace(/>/g, '&gt;').replace(/"/g, '&quot;');
  },

  // nothing has a place yet: centre on this device if it will say, once, and
  // otherwise leave the world where it is. No error either way — a map with
  // nothing on it is a map.
  locate() {
    if (this.fitted || this.located || !navigator.geolocation) return;
    this.located = true;
    navigator.geolocation.getCurrentPosition(
      (pos) => {
        if (this.fitted) return;
        this.map.setView([pos.coords.latitude, pos.coords.longitude], 14);
      },
      () => {},
      { timeout: 6000, maximumAge: 300000 });
  },
};

{
  // the host, made at load and living outside #app so a repaint cannot take
  // it away. Hidden until a render says the map view is up.
  const fm_mapHost = document.createElement('div');
  fm_mapHost.id = 'misoMap';
  fm_mapHost.style.display = 'none';
  document.body.appendChild(fm_mapHost);
  feature_Map.host = fm_mapHost;

  // /loop's paint seam, taken by replacing the property at load — /keep's
  // idiom, and NOT a timer-installed wrapper (notes.md, "the apply-wrapper
  // race"). The map is synced after the html is on the screen, because that
  // is when #mapData exists to be found.
  if (typeof feature_Loop !== 'undefined') {
    const fm_mapPaint = feature_Loop.paint;
    feature_Loop.paint = function (html) {
      fm_mapPaint.call(this, html);
      try {
        feature_Map.sync();
      } catch (e) {
      }
    };
  }

  // a rotated phone, or a keyboard closing: Leaflet must be told the box
  // changed size, and only while it is on screen.
  window.addEventListener('resize', () => {
    if (feature_Map.map && feature_Map.host.style.display === 'block') {
      feature_Map.map.invalidateSize();
    }
  });
}
