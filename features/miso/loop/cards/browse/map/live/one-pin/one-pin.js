const feature_OnePin = {
  // the card ids /live last drew a pin for: while an id is here, the placed
  // marker for that card stands aside
  live: {},

  // /map's markers carry no card; the rows they were drawn from do. /map's
  // draw pushes one marker per row with a numeric position, in row order, so
  // the array and the kept rows align by index. A count that disagrees is a
  // shape this node does not know: tag nothing, and every pin stands as /map
  // drew it — two pins is the known failure, a wrongly hidden person is not.
  tag() {
    if (typeof feature_Map === 'undefined') return;
    const data = document.getElementById('mapData');
    if (!data) return;
    let pins = [];
    try { pins = JSON.parse(data.getAttribute('data-pins') || '[]'); } catch (e) { pins = []; }
    const kept = pins.filter((p) => p && typeof p.lat === 'number' && typeof p.lon === 'number');
    const ms = feature_Map.markers || [];
    if (kept.length !== ms.length) return;
    for (let i = 0; i < ms.length; i++) ms[i].fm_card = String(kept[i].id || '');
  },

  // what /live just drew: the ids with a live pin standing
  apply(rows) {
    const live = {};
    for (const p of rows || []) {
      if (!p || !p.id) continue;
      if (typeof p.lat !== 'number' || typeof p.lon !== 'number') continue;
      live[String(p.id)] = true;
    }
    this.live = live;
    this.settle();
  },

  // a placed marker steps aside while its person's live pin stands, and steps
  // back when it goes. remove()/addTo() rather than opacity: the marker keeps
  // its place in /map's array and its click handler (Leaflet holds listeners
  // on the layer), and an invisible pin never catches a tap.
  settle() {
    if (typeof feature_Map === 'undefined' || !feature_Map.map) return;
    for (const m of feature_Map.markers || []) {
      const id = m.fm_card;
      if (!id) continue;
      const stand = !!this.live[id];
      if (stand && !m.fm_aside) {
        m.remove();
        m.fm_aside = true;
      } else if (!stand && m.fm_aside) {
        m.addTo(feature_Map.map);
        m.fm_aside = false;
      }
    }
  },
};

{
  // both seams taken by property replacement at load (/boundaries' idiom,
  // never a timer). Without /map or /live there is nothing to settle.
  if (typeof feature_Map !== 'undefined') {
    const fm_onePinDraw = feature_Map.draw;
    feature_Map.draw = function (pins) {
      fm_onePinDraw.call(this, pins);
      try {
        feature_OnePin.tag();
        feature_OnePin.settle();
      } catch (e) {
      }
    };
  }
  if (typeof feature_Live !== 'undefined') {
    const fm_onePinLive = feature_Live.draw;
    feature_Live.draw = function (rows) {
      fm_onePinLive.call(this, rows);
      try { feature_OnePin.apply(rows); } catch (e) {}
    };
    const fm_onePinClear = feature_Live.clear;
    feature_Live.clear = function () {
      fm_onePinClear.call(this);
      try { feature_OnePin.apply([]); } catch (e) {}
    };
  }
}
