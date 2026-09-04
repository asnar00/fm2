// the work queue as a plain feature list: the most recent eight announcements,
// newest first, whatever stage each is at — building, testing, deploying,
// installed — instead of only the ones still `building`. The first three words
// are stamped by the deploy, which knows when its gate starts and when its ship
// lands; `installed` is decided here, on the phone, because it means "this
// phone is running it" and no server can answer that for somebody else's
// device. No build numbers on the sheet (#p208).
const feature_Recent = {
  // eight: the block sits above the requests and the feature list in one
  // panel, and eight rows is what fits before it starts pushing the list it
  // belongs to off the screen — about a working day of announcements.
  N: 8,

  entries() {
    try {
      const s = JSON.parse(feature_Loop.state || '{}');
      const list = JSON.parse(s.builds || '[]');
      return Array.isArray(list) ? list : [];
    } catch (e) { return []; }
  },

  // the build this phone actually launched from — /update's own answer, with
  // its store as the fallback. 'first-run' parses to nothing, which reads as
  // not-yet-installed, and that is honest: a phone that cannot say what it
  // runs cannot claim to have it.
  mine() {
    let v = null;
    if (typeof feature_Update !== 'undefined' && feature_Update.running) {
      v = feature_Update.running;
    }
    if (!v) { try { v = localStorage.misoVersion; } catch (e) { v = null; } }
    const n = parseInt(v, 10);
    return isNaN(n) ? 0 : n;
  },

  stage(a) {
    const st = String(a.status || 'building');
    if (st === 'shipped') {
      const b = parseInt(a.build, 10);
      return (b && this.mine() >= b) ? 'installed' : 'deploying';
    }
    if (st === 'testing' || st === 'deploying') return st;
    return 'building';
  },

  list() {
    return this.entries()
      .slice()
      .sort((a, b) => (b.t || 0) - (a.t || 0))
      .slice(0, this.N)
      .map((a) => Object.assign({}, a, { status: feature_Recent.stage(a) }));
  },

  // the word decides the pill's colour, and a deploy that stopped leaves its
  // reason under the row it belongs to. Both are done to the drawn rows
  // rather than by rewriting /lifecycle's row builder, which siblings share.
  paint() {
    const sect = document.getElementById('building');
    if (!sect) return;
    for (const el of sect.querySelectorAll('.cnum')) {
      el.classList.remove('stage-installed');
      if ((el.textContent || '').trim() === 'installed') {
        el.classList.add('stage-installed');
      }
    }
    const by = {};
    for (const a of this.list()) by[a.t] = a;
    for (const row of sect.querySelectorAll('.crow[data-req]')) {
      const a = by[parseInt(row.getAttribute('data-req'), 10)];
      const more = row.nextElementSibling;
      if (!a || !a.why || !more || !more.classList.contains('cmore')) continue;
      if (more.querySelector('.stage-why')) continue;
      const note = document.createElement('div');
      note.className = 'stage-why';
      note.textContent = a.why;
      more.appendChild(note);
    }
  },
};

{
  // what /announced contributes to the block: the recent N, stage-worded,
  // rather than the building-only list. Nothing else wraps this function
  // today; if something ever does, the seam to redefine is `list` below it.
  if (typeof feature_Announced !== 'undefined') {
    feature_Announced.building = function () { return feature_Recent.list(); };
  }
  // /being-built draws the block from inside /lifecycle's render, so this
  // wrap — newer, therefore outermost — runs once the rows are in the DOM
  if (typeof feature_Lifecycle !== 'undefined') {
    const fm_recentRender = feature_Lifecycle.render.bind(feature_Lifecycle);
    feature_Lifecycle.render = function () {
      fm_recentRender();
      feature_Recent.paint();
    };
  }
}
