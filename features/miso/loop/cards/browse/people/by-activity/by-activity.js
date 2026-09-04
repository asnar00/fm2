// the third of the three things "active" means: the last time a person's phone
// said where it was. The other two — their last post and their last card edit —
// are in the world already; this one is the server's, reaching the page through
// /live's own poll, so it is handed in as an event under this node's own key.
// /people does exactly this with the invite distances, and for the same reason:
// a fact the server holds is not world state and must not pretend to be.
const feature_ByActivity = {
  said: '',
  pending: null,

  // {cardId: lastSeenMs} from /live's rows. A live person with no card of
  // yours is skipped — there is no row of theirs to order.
  from(rows) {
    const out = {};
    for (const r of (rows || [])) {
      if (!r || !r.id) continue;
      const t = Number(r.t) || 0;
      if (t > 0) out[r.id] = t;
    }
    return out;
  },

  // deferred, and never from inside a paint. /live's `draw` is called from its
  // own fetch, but /live-only repaints the band from inside that same call, so
  // sending straight from here would put an event inside a paint — the fault
  // that took build 690 down. One timer, latest value wins.
  tell(rows) {
    const m = this.from(rows);
    const v = JSON.stringify(m);
    if (v === this.said) return;
    this.said = v;
    if (this.pending) return;
    const self = this;
    this.pending = setTimeout(() => {
      self.pending = null;
      if (typeof feature_Loop === 'undefined' || feature_Loop.state == null) return;
      try { feature_Loop.send({ type: 'PeopleActive', data: m }); } catch (e) { /* the next tick tells it */ }
    }, 0);
  },
};

{
  if (typeof feature_Live !== 'undefined') {
    const fm_baDraw = feature_Live.draw;
    feature_Live.draw = function (rows) {
      const r = fm_baDraw.call(this, rows);
      try { feature_ByActivity.tell(rows); } catch (e) { /* the order stands */ }
      return r;
    };
  }
}
