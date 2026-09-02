const feature_Live = {
  // ---- the phone that says where it is ------------------------------------
  // one predicate decides whether this device publishes at all. A "share for
  // the next hour" is a second clause here, and nothing else changes.
  BEAT_MS: 10000,
  POLL_MS: 5000,
  timer: null,        // the heartbeat, alive only while may() holds
  poll: null,         // the map's ask, alive only while the map view is up
  markers: {},        // name -> L.marker, the live pins on the map
  fitted: false,

  may() {
    if (typeof document === 'undefined') return false;
    return document.visibilityState === 'visible';
  },

  // one fix, one post — and may() is asked again when the fix lands, so a
  // phone that went dark while the position was being taken sends nothing.
  beat() {
    if (!this.may()) { this.leave(); return; }
    const geo = typeof navigator !== 'undefined' && navigator.geolocation;
    if (!geo || typeof geo.getCurrentPosition !== 'function') return;
    try {
      geo.getCurrentPosition((p) => {
        const c = (p && p.coords) || {};
        if (typeof c.latitude !== 'number' || typeof c.longitude !== 'number') return;
        if (!this.may()) return;
        fetch('live/here', {
          method: 'POST',
          headers: { 'content-type': 'application/json' },
          body: JSON.stringify({ lat: c.latitude, lon: c.longitude }),
        }).catch(() => {});
      }, () => {}, { enableHighAccuracy: false, timeout: 8000, maximumAge: 5000 });
    } catch (e) {
      /* an API that throws is an API that is not there */
    }
  },

  arrive() {
    if (!this.may()) return;
    if (this.timer) return;
    this.timer = setInterval(() => this.beat(), this.BEAT_MS);
    this.beat();
    if (this.onMap()) this.watch();
  },

  // gone, at once: a beacon leaves even as the page is torn down
  leave() {
    if (this.timer) { clearInterval(this.timer); this.timer = null; }
    try {
      if (navigator.sendBeacon) {
        navigator.sendBeacon('live/gone', new Blob(['{}'], { type: 'application/json' }));
      } else {
        fetch('live/gone', { method: 'POST', keepalive: true, body: '{}' }).catch(() => {});
      }
    } catch (e) {
    }
    this.unwatch();
  },

  // ---- the map that asks ----------------------------------------------------
  // /map's sync runs after every paint; #mapData present means the map view
  // is up. Polling lives only while it is, and only while the page is seen.

  onMap() {
    return !!document.getElementById('mapData')
      && typeof feature_Map !== 'undefined' && !!feature_Map.map;
  },

  sync() {
    if (this.onMap() && this.may()) this.watch();
    else this.unwatch();
  },

  watch() {
    if (this.poll) return;
    this.poll = setInterval(() => this.pull(), this.POLL_MS);
    this.pull();
  },

  unwatch() {
    if (this.poll) { clearInterval(this.poll); this.poll = null; }
    this.clear();
  },

  async pull() {
    if (!this.onMap() || !this.may()) { this.unwatch(); return; }
    let d = null;
    try {
      const r = await fetch('live/near', { cache: 'no-store' });
      d = await r.json();
    } catch (e) {
      d = null;
    }
    if (!d || !d.ok || !Array.isArray(d.live)) return;
    if (!this.onMap()) return;
    this.draw(d.live);
  },

  // one marker per live person, moved rather than remade so a pin slides;
  // whoever left the app is taken off. Above the placed pins.
  draw(rows) {
    if (typeof L === 'undefined' || typeof feature_Map === 'undefined' || !feature_Map.map) return;
    const seen = {};
    const at = [];
    for (const p of rows) {
      if (typeof p.lat !== 'number' || typeof p.lon !== 'number') continue;
      const key = p.name || (p.me ? '(me)' : '');
      if (!key) continue;
      seen[key] = true;
      at.push([p.lat, p.lon]);
      const have = this.markers[key];
      if (have) {
        have.setLatLng([p.lat, p.lon]);
        continue;
      }
      const m = L.marker([p.lat, p.lon], {
        icon: L.divIcon({
          className: 'map-pin-icon map-live-icon',
          html: this.pinHtml(p),
          iconSize: [40, 50],
          iconAnchor: [20, 50],
        }),
        title: p.name || '',
        keyboard: false,
        zIndexOffset: 1000,
      }).addTo(feature_Map.map);
      // the tap seam: a later node hangs "message them" here
      m.on('click', () => this.tap(p));
      this.markers[key] = m;
    }
    for (const key of Object.keys(this.markers)) {
      if (seen[key]) continue;
      this.markers[key].remove();
      delete this.markers[key];
    }
    // a map that has fitted nothing yet fits the live pins, once — the same
    // one-time courtesy /map pays its own pins, and never a fight with the hand
    if (at.length && !this.fitted && !feature_Map.fitted) {
      this.fitted = true;
      try {
        feature_Map.map.fitBounds(L.latLngBounds(at).pad(0.25), { animate: false, maxZoom: 16 });
      } catch (e) {
      }
    }
  },

  // a live pin opens the card you hold, as a placed pin does; a person with
  // no card yet (yourself, before the first) opens nothing
  tap(p) {
    if (!p.id || typeof feature_Loop === 'undefined') return;
    // after the click has finished bubbling, not during it: the open
    // repaints the page synchronously, the map view goes and clear() takes
    // this marker with it, and /backdrop's document listener then sees a
    // card page open and a tap whose target is no longer on the page —
    // "the bare ground" — and closes what was just opened (one-pin review,
    // 2026-09-02). A beat later the tap has landed and the page may change.
    setTimeout(() => feature_Loop.send({ type: 'click', ev: 'browse_open:' + p.id }), 0);
  },

  clear() {
    for (const key of Object.keys(this.markers)) this.markers[key].remove();
    this.markers = {};
  },

  // /map's own pin markup with one word added: the face is the same face,
  // the ring is the only difference (live.css)
  pinHtml(p) {
    const esc = (s) => String(s).replace(/&/g, '&amp;').replace(/</g, '&lt;')
      .replace(/>/g, '&gt;').replace(/"/g, '&quot;');
    const inner = p.face
      ? '<img src="' + esc(p.face) + '" alt="">'
      : '<span>' + esc(p.initial || '') + '</span>';
    return '<div class="map-pin map-live"' + (p.me ? ' data-me="1"' : '')
      + ' data-name="' + esc(p.name || '') + '"><div class="map-pin-face">'
      + inner + '</div><div class="map-pin-stem"></div></div>';
  },
};

{
  // the ways of leaving and coming back: visibility, and only visibility.
  // An installed app on iOS answers hasFocus() false for its whole life,
  // never fires focus, and fires blur at odd moments (the Spotlight overlay
  // closing at launch) with no focus to balance it — the iPhone 17 Pro
  // simulator, 2026-09-02, twice. On a phone one app is in front at a time,
  // so visible is focused; hidden is gone.
  window.addEventListener('pagehide', () => feature_Live.leave());
  document.addEventListener('visibilitychange', () => {
    if (document.visibilityState === 'visible') feature_Live.arrive();
    else feature_Live.leave();
  });

  // /map's sync seam, taken as /boundaries takes it: the property replaced
  // at load, never a timer (notes.md, "the apply-wrapper race"). Without /map
  // there is no map and nothing to draw on — and this node is its child.
  if (typeof feature_Map !== 'undefined') {
    const fm_liveSync = feature_Map.sync;
    feature_Map.sync = function () {
      fm_liveSync.call(this);
      try {
        feature_Live.sync();
      } catch (e) {
      }
    };
  }

  // the loop may not be up yet at load; the first heartbeat waits for it so
  // a cookie-less login page never posts
  const fm_liveStart = setInterval(() => {
    if (typeof feature_Loop === 'undefined' || !feature_Loop.state) return;
    clearInterval(fm_liveStart);
    feature_Live.arrive();
  }, 250);
}
