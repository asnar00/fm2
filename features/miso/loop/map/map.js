// the hardware half: the map tool being open is the intent, a position
// watch is the effect, and readings come back as events — so the blackbox
// records the whole story and replay needs no satellites.
const feature_Map = {
  watchId: null,
  open: false,

  start() {
    if (this.watchId !== null) return;
    if (!navigator.geolocation) {
      feature_Loop.send({ type: 'LocateFailed',
        data: { err: 'this device has no location sensor' } });
      return;
    }
    this.watchId = navigator.geolocation.watchPosition(
      (p) => feature_Loop.send({ type: 'Located', data: {
        lat: p.coords.latitude, lon: p.coords.longitude,
        acc: p.coords.accuracy, t: p.timestamp } }),
      (e) => feature_Loop.send({ type: 'LocateFailed', data: {
        // a refusal and a broken sensor want different things from the
        // reader, so they are never reported as the same thing
        err: e && e.code === 1 ? 'location permission refused'
                               : 'location unavailable right now' } }),
      { enableHighAccuracy: true, maximumAge: 10000, timeout: 20000 });
  },

  stop() {
    if (this.watchId === null) return;
    try { navigator.geolocation.clearWatch(this.watchId); } catch (e) {}
    this.watchId = null;
  },

  // sensors follow state edges; re-enactment must touch no hardware
  watch() {
    if (typeof feature_Replay !== 'undefined' && feature_Replay.active) return;
    let s = {};
    try { s = JSON.parse(feature_Loop.state || '{}'); } catch (e) { return; }
    const open = s.open_tool === 'map';
    if (open === this.open) return;
    this.open = open;
    if (open) this.start(); else this.stop();
  },

  init() {
    const fm_mapApply = feature_Loop.apply;
    const self = this;
    feature_Loop.apply = function (p) {
      fm_mapApply.call(this, p);
      self.watch();
    };
    this.watch();
  },
};
const fm_mapInit = setInterval(() => {
  if (typeof feature_Loop !== 'undefined' && feature_Loop.state !== null) {
    clearInterval(fm_mapInit);
    feature_Map.init();
  }
}, 100);
