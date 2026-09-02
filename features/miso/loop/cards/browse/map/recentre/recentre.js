const feature_Recentre = {
  ZOOM: 16,
  // /live's own options, unchanged: this is the same read of the same sensor,
  // and its `maximumAge` is why a phone that has just published a heartbeat
  // answers this tap instantly instead of waking the radio again.
  OPTS: { enableHighAccuracy: false, timeout: 8000, maximumAge: 5000 },
  asking: false,
  aimed: false,   // the person has said where the map should look

  // the tap. Answered here and nowhere else: nothing in the world changes,
  // so there is no turn to take — the loop never sees this event and /undo
  // never files a step for it. The `data-ev` is still the row's own idiom
  // (it is how /sub-tool-cards' long press finds the control, and how the
  // tree export stamps the button against this node).
  go() {
    const map = (typeof feature_Map !== 'undefined') && feature_Map.map;
    if (!map) return;                      // no leaflet, no map: nothing to move
    if (this.asking) return;               // a second tap while a fix is in flight
    const geo = typeof navigator !== 'undefined' && navigator.geolocation;
    if (!geo || typeof geo.getCurrentPosition !== 'function') { this.lost(); return; }
    this.asking = true;
    try {
      geo.getCurrentPosition(
        (p) => {
          this.asking = false;
          const c = (p && p.coords) || {};
          if (typeof c.latitude !== 'number' || typeof c.longitude !== 'number') {
            this.lost();
            return;
          }
          this.to(c.latitude, c.longitude);
        },
        () => { this.asking = false; this.lost(); },
        this.OPTS);
    } catch (e) {
      this.asking = false;
      this.lost();
    }
  },

  // the move, made only if the map is still the thing on the screen: a fix can
  // take seconds, and a person who has left the map view in the meantime should
  // not come back to a view that moved under them.
  to(lat, lon) {
    if (!document.getElementById('mapData')) return;
    if (typeof feature_Map === 'undefined' || !feature_Map.map) return;
    // the map has now been aimed by hand. /map's own one-time fit and /live's
    // both read this flag before fitting bounds, so setting it is what stops a
    // live pin arriving a second later from throwing the view away again.
    feature_Map.fitted = true;
    this.aimed = true;
    feature_Map.map.setView([lat, lon], this.ZOOM, { animate: true });
  },

  // the app's own voice, one line at the foot, gone in three seconds. Nothing
  // moves and nothing throws; with /cards unticked there is no toast and the
  // failure is silent, which is what the map's own locate() has always done.
  lost() {
    if (typeof feature_Cards !== 'undefined' && feature_Cards.say) {
      feature_Cards.say("can't find you");
    }
  },
};

{
  // /map's `draw` refits the whole map whenever the SET of pins changes — not
  // only on the first draw, and `fitted` does not guard it. A card edited, a
  // copy arriving from someone you hold, a post made: any of them throws the
  // view away, and the rig caught it doing exactly that within a second of the
  // profile picture landing. Once someone has said "put me in the middle" that
  // is an answer, so the pins are redrawn and the aim is put back — /map's own
  // property, replaced at load and calling the captured original (/live's and
  // /boundaries' idiom for the same object), never a timer.
  //
  // The ruling this encodes: an aim by hand outlives every automatic fit for
  // the life of the page. Before this node nothing could aim it by hand.
  if (typeof feature_Map !== 'undefined' && typeof feature_Map.draw === 'function') {
    const fm_recentreDraw = feature_Map.draw;
    feature_Map.draw = function (pins) {
      if (!feature_Recentre.aimed || !this.map) {
        return fm_recentreDraw.call(this, pins);
      }
      const c = this.map.getCenter();
      const z = this.map.getZoom();
      const out = fm_recentreDraw.call(this, pins);
      try {
        this.map.setView([c.lat, c.lng], z, { animate: false });
      } catch (e) {
      }
      return out;
    };
  }

  // capture, on document, ahead of /loop's own delegated [data-ev] listener —
  // which is a bubble-phase listener on the same node, so stopping propagation
  // here means it never runs and the tap costs no turn. /sub-tool-cards'
  // swallow is the same idiom in the same place.
  document.addEventListener('click', (e) => {
    if (!e.target || !e.target.closest) return;
    if (!e.target.closest('[data-ev="map_recentre"]')) return;
    e.stopPropagation();
    // a long press READ this button; it must not also fire it.
    // /sub-tool-cards' swallow calls preventDefault on exactly that click and
    // cannot stop a second listener on the same node, so the mark it leaves is
    // what is read here.
    if (e.defaultPrevented) return;
    e.preventDefault();
    try {
      feature_Recentre.go();
    } catch (err) {
    }
  }, true);
}
