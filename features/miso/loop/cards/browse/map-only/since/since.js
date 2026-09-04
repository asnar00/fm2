// the day starts, computed where the clock and the zone are. A wasm build has
// neither SystemTime nor a local time zone (/browse's own date arithmetic says
// so), so it cannot know when local midnight was; the page does. The three
// marks travel in as ONE event — the way a finger sends one — rather than as a
// write to a bridged key, because a node newer than /payload moves state with
// events and never by writing a bridged key (misses.md, navigation from the
// wrong side).
const feature_Since = {
  sent: '',
  midnight: null,

  // midnight today; midnight of the most recent Monday (Monday is day 1, so a
  // Sunday goes back six days); midnight of the 1st. Built by walking a Date
  // rather than by subtracting milliseconds, so a clock change inside the week
  // or the month lands on the wall-clock midnight and not an hour either side.
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

  // quiet when nothing moved: a visibility flip on a phone that has not
  // crossed midnight or a time zone costs no event and no repaint.
  tell() {
    if (typeof feature_Loop === 'undefined') return;
    const m = this.marks();
    if (m === this.sent) { this.arm(); return; }
    this.sent = m;
    try { feature_Loop.send({ type: 'SinceMarks', data: { marks: m } }); } catch (e) { /* not up yet */ }
    this.arm();
  },

  // one timer, aimed at the next local midnight plus a second. Re-aimed every
  // time it fires, so a phone left open overnight rolls over on its own.
  arm() {
    if (this.midnight) clearTimeout(this.midnight);
    const next = new Date();
    next.setHours(24, 0, 1, 0);
    let wait = next.getTime() - Date.now();
    if (!isFinite(wait) || wait < 1000) wait = 1000;
    if (wait > 86400000) wait = 86400000;
    this.midnight = setTimeout(() => this.tell(), wait);
  },

  // the words the long-press card says for the four pills. /tool-words keeps
  // the same table for the tools and the picker's buttons; these are this
  // node's own, so they arrive and leave with it.
  WORDS: {
    since_today: { name: 'today', intro: 'Only what happened since midnight.' },
    since_week: { name: 'week', intro: 'This week, from Monday morning.' },
    since_month: { name: 'month', intro: 'This month, from the first.' },
    since_all: { name: 'all', intro: 'Everything, however old.' },
  },
};

{
  const fm_sinceStart = setInterval(() => {
    if (typeof feature_Loop !== 'undefined') {
      clearInterval(fm_sinceStart);
      feature_Since.tell();
    }
  }, 100);
  if (typeof document !== 'undefined') {
    document.addEventListener('visibilitychange', () => {
      if (document.visibilityState === 'visible') feature_Since.tell();
    });
  }
  // the pills are not tool buttons, so /sub-tool-cards does not arm them; arm
  // them the way /tool-words arms the picker's, and answer with this node's
  // own words before anything else in the chain.
  if (typeof feature_LongPress !== 'undefined') {
    const fm_sinceContent = feature_LongPress.contentFor.bind(feature_LongPress);
    feature_LongPress.contentFor = async function (btn) {
      const w = feature_Since.WORDS[btn.getAttribute('data-ev') || ''];
      if (w) return { name: w.name, intro: w.intro };
      return fm_sinceContent(btn);
    };
    document.addEventListener('pointerdown', (e) => {
      const btn = e.target && e.target.closest ? e.target.closest('.since-pill[data-ev]') : null;
      if (!btn) return;
      feature_LongPress.disarm();
      feature_LongPress.fired = false;
      feature_LongPress.armed = btn;
      feature_LongPress.x = e.clientX;
      feature_LongPress.y = e.clientY;
      feature_LongPress.timer = setTimeout(() => feature_LongPress.show(btn), 500);
    });
    // a long press READS; the tap that follows it must not also switch.
    document.addEventListener('click', (e) => {
      if (feature_LongPress.fired && e.target && e.target.closest && e.target.closest('.since-pill[data-ev]')) {
        e.stopPropagation();
        e.preventDefault();
        feature_LongPress.fired = false;
      }
    }, true);
  }
}
