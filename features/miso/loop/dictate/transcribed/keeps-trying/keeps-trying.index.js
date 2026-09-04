// what the engineer section says about transcription: one line per clip that
// has not landed, and one line when nothing on the box can transcribe at all.
// /engineer's own idiom — capture fill, replace it, call the captured one
// first, append after it. Nothing of this appears anywhere else on the app.
const feature_KeepsTrying = {
  last: null,
  at: 0,
  asking: false,

  // the sheet's fill is synchronous and the answer is a fetch, so the block is
  // drawn from what we hold and the fetch redraws when it lands. Never more
  // than one in flight, and never one within five seconds of the last.
  load() {
    if (this.asking || Date.now() - this.at < 5000) return;
    this.asking = true;
    fetch('diag/transcribe', { cache: 'no-store' })
      .then((r) => (r.ok ? r.json() : null))
      .then((v) => {
        this.at = Date.now();
        this.asking = false;
        if (!v) return;
        this.last = v;
        if (typeof feature_Engineer !== 'undefined') feature_Engineer.refresh();
      })
      .catch(() => { this.asking = false; this.at = Date.now(); });
  },

  when(ms) {
    const d = Math.round((ms - Date.now()) / 1000);
    if (!ms) return 'now';
    if (d <= 0) return 'now';
    if (d < 90) return 'in ' + d + 's';
    return 'in ' + Math.round(d / 60) + 'm';
  },

  lines() {
    const v = this.last;
    if (!v) return ['transcription: asking…'];
    const out = [];
    if (!v.best) {
      out.push('transcription: NO RUNG REACHABLE — ' + (v.why_not || 'reason unknown'));
    } else {
      out.push('transcription: ' + (v.rung || 'grade ' + v.best) + ' ready');
    }
    for (const j of v.waiting || []) {
      out.push('  waiting  ' + j.id + '  ' + j.who + '  try ' + (j.tries || 0)
        + '  ' + this.when(j.next) + '  — ' + (j.why || ''));
    }
    for (const j of v.parked || []) {
      out.push('  PARKED   ' + j.id + '  ' + j.who + '  after ' + (j.tries || 0)
        + ' tries  — ' + (j.why || ''));
    }
    if (!(v.waiting || []).length && !(v.parked || []).length) {
      out.push('  nothing waiting');
    }
    return out;
  },
};
{
  if (typeof feature_Engineer !== 'undefined') {
    const fm_ktFill = feature_Engineer.fill.bind(feature_Engineer);
    feature_Engineer.fill = function (box) {
      fm_ktFill(box);
      feature_KeepsTrying.load();
      const block = document.createElement('div');
      block.id = 'keepsTrying';
      block.textContent = feature_KeepsTrying.lines().join('\n');
      box.appendChild(block);
    };
  }
}
