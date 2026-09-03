// a recording's post is placed where the recording was made. /as-posts mints
// the card on RecSaved and writes the recording's id on it as `rec`; the
// device is asked once and the card is placed with /location's own event —
// the same block, the same source, the same pill — stamped with the
// recording's time, not the fix's.
{
  if (typeof feature_Loop !== 'undefined' && !feature_Loop.fm_whereTaken) {
    feature_Loop.fm_whereTaken = true;
    const fm_wtSend = feature_Loop.send;
    feature_Loop.send = function (event) {
      fm_wtSend.call(this, event);
      if (!event || event.type !== 'RecSaved' || !event.data || !event.data.id) return;
      const rec = event.data.id;
      const when = typeof event.data.t === 'number' ? event.data.t : Date.now();
      try {
        const geo = typeof navigator !== 'undefined' && navigator.geolocation;
        if (!geo || typeof geo.getCurrentPosition !== 'function') return;
        geo.getCurrentPosition((p) => {
          const c = (p && p.coords) || {};
          if (typeof c.latitude !== 'number' || typeof c.longitude !== 'number') return;
          if (!feature_Loop.state) return;
          let cards = [];
          try { cards = JSON.parse(JSON.parse(feature_Loop.state).cards || '[]'); } catch (e) { cards = []; }
          const card = cards.find((k) => k && k.rec === rec);
          if (!card || !card.id) return;
          feature_Loop.send({ type: 'CardPlace', data: {
            id: card.id, lat: c.latitude, lon: c.longitude,
            acc: typeof c.accuracy === 'number' ? c.accuracy : 0, t: when } });
        }, () => {}, { enableHighAccuracy: false, timeout: 10000, maximumAge: 60000 });
      } catch (e) {
        /* an API that throws is an API that is not there */
      }
    };
  }
}
