// the device's copy of the world, kept in IndexedDB so a reload starts from
// yesterday rather than from nothing. `miso-blobs` (the dictaphone's store) is
// the precedent for the furniture; this is its own database because a store is
// owned by the node that made it.
const feature_WorldCache = {
  db: null,
  owner: null,      // who the cache belongs to; null until we know
  last: '',         // what was written down, to skip writing it again
  timer: null,
  armed: false,     // nothing is written until the hydrate decision is made
  painted: null,    // the paint seam, held while the empty world is on screen
  hydrated: false,

  open() {
    return new Promise((res, rej) => {
      const rq = indexedDB.open('miso-world', 1);
      rq.onupgradeneeded = () => rq.result.createObjectStore('world');
      rq.onsuccess = () => res(rq.result);
      rq.onerror = () => rej(rq.error);
    });
  },

  // who this page is for, WITHOUT asking the network: the shell's loader has
  // already asked and left the answer on /panel. Offline that answer is null —
  // and null must not reject the cache, or the offline case, which is the
  // whole point, would be the one case that fails.
  who() {
    if (typeof feature_Panel !== 'undefined' && feature_Panel.lastWho)
      return feature_Panel.lastWho.name || '';
    return null;
  },

  // read the record, and refuse it if it belongs to somebody else: a device
  // that changed hands keeps no trace of the previous person's world.
  async load() {
    try {
      this.db = await this.open();
      const rec = await new Promise((res, rej) => {
        const tx = this.db.transaction('world', 'readonly');
        const rq = tx.objectStore('world').get('world');
        rq.onsuccess = () => res(rq.result || null);
        rq.onerror = () => rej(rq.error);
      });
      const who = this.who();
      if (rec && who !== null && rec.who !== who) {
        this.wipe();
        this.owner = who;
        return null;
      }
      this.owner = who !== null ? who : (rec && rec.who) || null;
      return rec;
    } catch (e) {
      return null;   // no IndexedDB, or a broken one: boot as we always did
    }
  },

  wipe() {
    try {
      const tx = this.db.transaction('world', 'readwrite');
      tx.objectStore('world').delete('world');
    } catch (e) {}
  },

  // one record, whole, keyed by the user: 'the var table as it stands' is the
  // thing that must be consistent, and one write of one value is how it stays
  // that way. Trailing 300ms, because a typed edit is many turns and only the
  // last one is worth the disk.
  note(world) {
    if (!this.db || !this.armed || !world || !world.length) return;
    const text = JSON.stringify(world);
    if (text === this.last) return;
    this.last = text;
    clearTimeout(this.timer);
    this.timer = setTimeout(() => this.write(text), 300);
  },

  write(text) {
    try {
      const tx = this.db.transaction('world', 'readwrite');
      tx.objectStore('world').put(
        { who: this.owner, at: Date.now(), ctx: JSON.parse(text) }, 'world');
    } catch (e) { /* a device that cannot write is a device without a cache */ }
  },

  // the empty world must never be painted. boot() applies the fresh world
  // before anything can hydrate it, so the paint seam is held shut across that
  // one turn and the hydrate's own turn does the first paint.
  hold() {
    if (this.painted) return;
    this.painted = feature_Loop.paint;
    feature_Loop.paint = function () {};
  },
  release() {
    if (!this.painted) return;
    feature_Loop.paint = this.painted;
    this.painted = null;
  },

  // a hydrated world is shown, but it is NOT joined: /veil keeps `fm-joined`
  // for the real join, so /me/patient still declines to make a card against a
  // world the server has not confirmed. All this does is stop the waiting
  // being a blank screen.
  reveal() {
    document.body.classList.add('fm-cached');
  },
};

{
  const fm_wcBoot = feature_Loop.boot;
  feature_Loop.boot = async function () {
    const rec = await feature_WorldCache.load();
    const ctx = rec && rec.ctx && rec.ctx.length ? rec.ctx : null;
    if (ctx) feature_WorldCache.hold();
    try {
      await fm_wcBoot.call(this);
    } finally {
      feature_WorldCache.release();
    }
    if (ctx) {
      feature_Loop.send({ type: 'WorldHydrate', data: { ctx } });
      feature_WorldCache.hydrated = true;
      feature_WorldCache.reveal();
    }
    feature_WorldCache.armed = true;
  };

  // every applied turn carries the world beside the html; this is where it is
  // noticed. The payload is parsed a second time here rather than read off
  // feature_Loop.state, because the world deliberately never enters the state.
  const fm_wcApply = feature_Loop.apply;
  feature_Loop.apply = function (payloadJson) {
    fm_wcApply.call(this, payloadJson);
    try {
      feature_WorldCache.note(JSON.parse(payloadJson).world);
    } catch (e) { /* a payload we cannot read is not worth writing down */ }
  };
}
