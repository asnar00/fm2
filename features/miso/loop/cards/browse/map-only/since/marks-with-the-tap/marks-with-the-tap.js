// the three midnights, carried by the finger. /since sends them once at load
// and, on ash's phone, that send never landed (asks#1788532331774): its poll
// waits for `typeof feature_Loop !== 'undefined'`, which is true as soon as the
// fragment is parsed and long before the wasm world is up, and it assigns
// `sent` ABOVE the send inside a try that swallows — so one failed send at boot
// is permanent. A tap that carries its own marks cannot be waiting to be told
// the time.
const feature_MarksWithTheTap = {
  sent: '',
  poll: null,

  // /since's own three, recomputed at the moment they are asked for: midnight
  // today; midnight of the most recent Monday (Monday is day 1, so a Sunday
  // goes back six days); midnight of the 1st. Walked with a Date rather than
  // subtracted in milliseconds, so a clock change lands on the wall-clock
  // midnight and not an hour either side.
  marks() {
    const day = new Date();
    day.setHours(0, 0, 0, 0);
    const week = new Date(day.getTime());
    week.setDate(week.getDate() - ((week.getDay() + 6) % 7));
    week.setHours(0, 0, 0, 0);
    const month = new Date(day.getTime());
    month.setDate(1);
    month.setHours(0, 0, 0, 0);
    return day.getTime() + ',' + week.getTime() + ',' + month.getTime();
  },

  // the loop is up when it has STATE, not when its object exists. /restore
  // two nodes away already uses this test; /since used the weaker one.
  up() {
    return typeof feature_Loop !== 'undefined'
      && feature_Loop.state !== null && feature_Loop.state !== undefined;
  },

  // no latch: `sent` is recorded only after a send that returned. A send that
  // threw is tried again on the next poll.
  tell() {
    if (!this.up()) return false;
    const m = this.marks();
    if (m === this.sent) return true;
    try {
      feature_Loop.send({ type: 'SinceMarks', data: { marks: m } });
    } catch (e) {
      return false;
    }
    this.sent = m;
    return true;
  },

  // keep trying until one send gets through — the case a finger cannot cover
  // is an app that launches with a period already chosen and nobody tapping.
  chase() {
    if (this.poll) clearInterval(this.poll);
    if (this.tell()) { this.arm(); return; }
    this.poll = setInterval(() => {
      if (this.tell()) { clearInterval(this.poll); this.poll = null; this.arm(); }
    }, 250);
  },

  // midnight passing while the app sits open on the map: at the next local
  // midnight the marks are a day out, so one is sent and the map repaints on
  // that turn. Re-armed each time it fires, so a phone left out overnight
  // rolls over on its own, night after night.
  midnight: null,
  arm() {
    if (this.midnight) clearTimeout(this.midnight);
    const next = new Date();
    next.setHours(24, 0, 1, 0);
    let wait = next.getTime() - Date.now();
    if (!isFinite(wait) || wait < 1000) wait = 1000;
    if (wait > 86400000) wait = 86400000;
    this.midnight = setTimeout(() => { this.sent = ''; this.chase(); }, wait);
  },
};

{
  const fm_mwtStart = setInterval(() => {
    if (typeof feature_Loop === 'undefined') return;
    clearInterval(fm_mwtStart);

    // the one choke point every road goes through: the loop's delegated
    // [data-ev] listener, /on-release's synthetic click for a press held past
    // 120 ms, /drive, and every node that mints an event of its own. EVERY
    // event gets the marks, not only a pill's tap — ash's case is a phone shut
    // for two days and opened without touching the filter, where marks from
    // two days ago are worse than none, because the map then shows a day that
    // is not today and looks right. The freshest answer the phone has rides on
    // whatever it does next.
    //
    // They go at the top level, beside `type`: `data` belongs to whoever minted
    // the event and its shape is theirs, while nothing reads an unknown
    // top-level key.
    const fm_mwtSend = feature_Loop.send;
    feature_Loop.send = function (ev) {
      if (!ev || typeof ev !== 'object' || ev.marks) return fm_mwtSend.call(this, ev);
      const out = {};
      for (const k in ev) out[k] = ev[k];
      out.marks = feature_MarksWithTheTap.marks();
      return fm_mwtSend.call(this, out);
    };

    feature_MarksWithTheTap.chase();
  }, 50);

  if (typeof document !== 'undefined') {
    // a phone carried into another zone, or woken after midnight
    document.addEventListener('visibilitychange', () => {
      if (document.visibilityState === 'visible') feature_MarksWithTheTap.chase();
    });
  }
}
