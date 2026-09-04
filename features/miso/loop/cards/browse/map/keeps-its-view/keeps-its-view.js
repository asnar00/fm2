// the map opens where you left it. /map makes Leaflet at the world — `setView`
// at zoom 3 — and relies on the first `draw` to fit the pins, so a map made on
// a screen with no set showing has nothing to fit and stays at the world. That
// is what ash saw after the update to 685: centre 51.2719,0.1904 at zoom 15
// before, zoom 0 after (measured on the rig, #p119).
//
// The first cut of this node then broke boot (#p119a, build 690: "syncing…"
// stuck, then a crash). Leaflet fires `moveend` SYNCHRONOUSLY for a programmatic
// `setView` and for `fitBounds`, and both of those happen inside /map's `sync`,
// which runs inside `paint`, which runs inside `apply` — so sending from that
// handler re-entered the loop from inside its own paint. Measured on the rig:
// three re-entrant sends at boot, one nested to depth 2, and the veil lifts
// only in the line AFTER the inner apply returns (/veil's wrapper), so anything
// that throws down there leaves "syncing…" up for good.
//
// Two things follow, and neither is a guard. A move the app made is not a move
// the user made, so it is not recorded at all — which is also the correct rule:
// the remembered view is where the HAND left the map. And a view worth
// recording is sent after the paint, never during it, which is /keep's own
// idiom for the same hazard ("after the paint has finished rather than
// re-entering it").
const feature_KeepsItsView = {
  FLOOR: 3,             // /map's own placeholder zoom: below this is the globe
  restoreOnce: false,   // a view was applied at mount; the next fit must not undo it
  quiet: 0,             // depth of moves this node or /map is making itself
  pending: null,        // the one deferred send

  // "<lat>,<lon>,<zoom>" off the bridged var. One turn stale at worst, which
  // for "where was I looking" is the same answer.
  remembered() {
    try {
      const s = JSON.parse(feature_Loop.state || '{}');
      const v = String(s.map_view || '').split(',');
      if (v.length !== 3) return null;
      const lat = parseFloat(v[0]), lon = parseFloat(v[1]), z = parseInt(v[2], 10);
      if (!isFinite(lat) || !isFinite(lon) || !isFinite(z)) return null;
      // a zoom at or below /map's own placeholder is the whole globe, and no
      // hand on this app has ever chosen it — it is what the broken build
      // recorded from invalidateSize's own moveend, and every phone that ran
      // 690 has one stored. Treated as no memory, so the fit takes over, which
      // is the right answer for a map that has never been placed. This heals
      // the field rather than only stopping the next one.
      if (z <= this.FLOOR) return null;
      return { lat: lat, lon: lon, z: z };
    } catch (e) {
      return null;
    }
  },

  // a move the app is making: the restore at mount, the fit inside draw. The
  // handlers stay bound — Leaflet has no "who moved it" — so the answer is a
  // depth counter around the moves this tree makes on purpose.
  hush(fn) {
    this.quiet++;
    try { return fn(); } finally { this.quiet--; }
  },

  where(map) {
    if (!map) return '';
    try {
      const c = map.getCenter();
      return c.lat.toFixed(5) + ',' + c.lng.toFixed(5) + ',' + map.getZoom();
    } catch (e) {
      return '';   // a Leaflet with no view set yet has no centre to give
    }
  },

  // AFTER the paint, never inside it. One timer, latest value wins, so a drag
  // that fires a dozen moveends costs one event.
  say(map) {
    if (this.quiet > 0) return;
    if (this.pending) return;
    const self = this;
    this.pending = setTimeout(() => {
      self.pending = null;
      const v = self.where(map);
      if (!v) return;
      const had = self.remembered();
      if (had && v === had.lat.toFixed(5) + ',' + had.lng.toFixed(5) + ',' + had.z) return;
      if (typeof feature_Loop === 'undefined' || feature_Loop.state == null) return;
      try { feature_Loop.send({ type: 'MapView', data: { v: v } }); } catch (e) { /* the next move says it */ }
    }, 0);
  },

  // every road a HAND moves the map by ends in moveend or zoomend — a drag, a
  // pinch, /recentre's setView, /floating's pan — so one pair of handlers
  // records them all and /recentre needs nothing from this node.
  watch(map) {
    if (!map || map.fm_kvWatched) return;
    map.fm_kvWatched = true;
    const self = this;
    map.on('moveend', () => self.say(map));
    map.on('zoomend', () => self.say(map));
  },
};

{
  if (typeof feature_Map !== 'undefined') {
    // EVERY move the app makes is inside /map's sync — mount's setView, draw's
    // fitBounds, and invalidateSize's resize, which fires a moveend of its own
    // and was recording the world at zoom 0 on the rig. Every move a HAND
    // makes is outside it: a drag, a pinch, and /recentre, which moves the map
    // from its own click listener. So the line between "the app moved it" and
    // "the user moved it" is exactly this call, and hushing it is the whole
    // rule rather than a list of the moves known today.
    const fm_kvSync = feature_Map.sync;
    feature_Map.sync = function () {
      const self = this;
      return feature_KeepsItsView.hush(() => fm_kvSync.call(self));
    };

    // a map MADE at any time opens at the remembered view — never the world.
    //
    // "made" is the word that matters. /map's `mount` self-guards (`if
    // (this.map) return true`) and /map's `sync` calls it on every sync, so
    // this wrapper is handed a no-op far more often than a new Leaflet. The
    // first cut applied the remembered view on every one of those calls, which
    // snapped the map back out from under the hand a second after every drag —
    // and each snap fired the `moveend` that re-entered the loop. That pair is
    // what stuck the veil and grew the page until Safari killed it (#p119a).
    // So the restore happens on the transition and nowhere else.
    const fm_kvMount = feature_Map.mount;
    feature_Map.mount = function () {
      const fresh = !this.map;
      const made = fm_kvMount.call(this);
      if (!made || !this.map || !fresh) return made;
      try {
        feature_KeepsItsView.watch(this.map);
        const v = feature_KeepsItsView.remembered();
        if (v) {
          const map = this.map;
          feature_KeepsItsView.hush(() => map.setView([v.lat, v.lon], v.z, { animate: false }));
          // /map's own flag: with a view of the user's there is nothing for
          // locate() to ask the device about.
          this.fitted = true;
          feature_KeepsItsView.restoreOnce = true;
        }
      } catch (e) { /* the world, as /map left it */ }
      return made;
    };

    // the first draw after a mount fits the pins, which would take the
    // remembered view away again. The pins are still drawn — only the fit is
    // undone, and only that once: after it, /map's own rule (refit only when
    // the set of pins changes) is back, untouched. The whole call is hushed
    // either way, because a fit is the app moving the map and not the hand.
    const fm_kvDraw = feature_Map.draw;
    feature_Map.draw = function (pins) {
      const self = this;
      if (!feature_KeepsItsView.restoreOnce || !this.map) {
        return feature_KeepsItsView.hush(() => fm_kvDraw.call(self, pins));
      }
      const at = feature_KeepsItsView.where(this.map);
      const r = feature_KeepsItsView.hush(() => fm_kvDraw.call(self, pins));
      feature_KeepsItsView.restoreOnce = false;
      const p = at.split(',');
      if (p.length === 3) {
        feature_KeepsItsView.hush(() => {
          try {
            self.map.setView([parseFloat(p[0]), parseFloat(p[1])], parseInt(p[2], 10),
                             { animate: false });
          } catch (e) { /* as drawn */ }
        });
      }
      return r;
    };
  }
}
