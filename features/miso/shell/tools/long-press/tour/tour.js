// the first run's tour: /long-press's card, driven. Each step is a target
// (a selector for the real control), a line, and a test on the loop state
// or the DOM that says the move was made. Steps advance on apply — where a
// real tap's consequence lands — so nothing here fakes a tap and any road
// to the same state counts. A step whose control is not on the screen is
// passed over. Offered once per user: tour_seen travels; localStorage
// mirrors it so a relaunch before the op ships does not offer it twice.
const feature_Tour = {
  steps: [
    { at: '[data-ev="tools_home"]', say: "that's your card. tap ‹ to go on",
      skipIf: (s) => s.open_tool !== 'account', done: (s) => s.open_tool === '' },
    { at: '[data-ev="tool_posts"]', say: 'posts are what you make in the field. tap the bubble',
      done: (s) => s.open_tool === 'posts' },
    { at: '[data-ev="tool_posts"]', say: '+ makes a post from where you stand. tap the bubble again to come back',
      done: (s) => s.open_tool === '' },
    { at: '[data-ev="tool_account"]', say: 'people: everyone whose card you hold. tap the person',
      done: (s) => s.open_tool === 'account' },
    { at: '[data-ev="browse_map"]', say: 'the map puts them where they are. tap the map',
      done: () => !!document.querySelector('.browse-view.browse-on[data-ev="browse_map"]') },
    { at: '[data-ev="tool_account"]', say: 'tap the person again to come back',
      done: (s) => s.open_tool === '' },
    { at: '[data-ev="tool_projects"]', say: 'projects: a campaign, and who is in it. tap the flag',
      done: (s) => s.open_tool === 'projects' },
    { at: '[data-ev="tool_projects"]', say: "new makes one. tap the flag again, and it's yours",
      done: (s) => s.open_tool === '' },
  ],
  at: -1,        // -1 not started; -2 ended; else the current step
  card: null,
  here: null,    // the element wearing the ring

  state() {
    try { return JSON.parse(feature_Loop.state || '{}'); } catch (e) { return {}; }
  },

  seen(s) {
    if (s.tour_seen === true) return true;
    try { if (localStorage.misoTourSeen === '1') return true; } catch (e) {}
    return false;
  },

  // after the gate, never before; on a joined world; once
  may(s) {
    if (!s._joined) return false;
    if (this.seen(s)) return false;
    if (typeof feature_ProfileFirst !== 'undefined' && feature_ProfileFirst.gated()) return false;
    return true;
  },

  target(step) {
    return step ? document.querySelector(step.at) : null;
  },

  // the public door: a later "show me again" starts here
  start() {
    this.at = 0;
    this.check();
  },

  end() {
    this.at = -2;
    try { localStorage.misoTourSeen = '1'; } catch (e) {}
    this.ring(null);
    if (this.card) this.card.classList.remove('show');
    if (typeof feature_Loop !== 'undefined' && feature_Loop.state)
      feature_Loop.send({ type: 'TourSeen' });
  },

  check() {
    if (typeof feature_Loop === 'undefined' || feature_Loop.state === null) return;
    const s = this.state();
    if (this.at === -2) return;
    if (this.at === -1) {
      if (!this.may(s)) return;
      this.at = 0;
    }
    // pass over steps whose control is absent or that do not apply; advance
    // on the current step's test; end when the list runs out
    for (let guard = 0; guard < this.steps.length + 1; guard++) {
      const step = this.steps[this.at];
      if (!step) { this.end(); return; }
      if ((step.skipIf && step.skipIf(s)) || !this.target(step)) { this.at++; continue; }
      if (step.done(s)) { this.at++; continue; }
      break;
    }
    if (this.at >= this.steps.length) { this.end(); return; }
    this.place();
  },

  ring(el) {
    if (this.here && this.here !== el) this.here.classList.remove('tour-here');
    this.here = el;
    if (el) el.classList.add('tour-here');
  },

  place() {
    const step = this.steps[this.at];
    const el = this.target(step);
    if (!el) return;
    const c = this.card || this.make();
    c.querySelector('.tour-say').textContent = step.say;
    c.querySelector('.tour-skip').style.display = this.at > 0 ? '' : 'none';
    c.classList.add('show');
    this.ring(el);
    const r = el.getBoundingClientRect();
    const cw = c.offsetWidth;
    const ch = c.offsetHeight;
    let left = r.left + r.width / 2 - cw / 2;
    left = Math.max(8, Math.min(left, innerWidth - cw - 8));
    const above = r.top - ch - 12;
    const below = above < 8;
    c.classList.toggle('below', below);
    c.style.left = left + 'px';
    c.style.top = (below ? r.bottom + 12 : above) + 'px';
    c.style.setProperty('--tour-px', (r.left + r.width / 2 - left) + 'px');
  },

  make() {
    const c = document.createElement('div');
    c.id = 'tourCard';
    c.innerHTML = '<div class="tour-say"></div><div class="tour-skip">skip</div>';
    c.querySelector('.tour-skip').addEventListener('pointerdown', (e) => {
      e.preventDefault(); e.stopPropagation();
      feature_Tour.end();
    });
    document.body.appendChild(c);
    this.card = c;
    return c;
  },
};
{
  const fm_tourInit = setInterval(() => {
    if (typeof feature_Loop === 'undefined' || feature_Loop.state === null) return;
    clearInterval(fm_tourInit);
    const fm_tourApply = feature_Loop.apply;
    feature_Loop.apply = function (p) {
      fm_tourApply.call(this, p);
      feature_Tour.check();
    };
    // repaints move the target; the ring and the card follow
    const app = document.getElementById('app');
    if (app) new MutationObserver(() => { if (feature_Tour.at >= 0) feature_Tour.check(); })
      .observe(app, { childList: true, subtree: true });
    addEventListener('resize', () => { if (feature_Tour.at >= 0) feature_Tour.place(); });
    // the toolbar's buttons slide in on a mode change (/tools' bar-slide,
    // /steady): the target's rectangle is mid-flight when the paint lands,
    // so the card is placed again when the motion has ended
    for (const ev of ['animationend', 'transitionend'])
      document.addEventListener(ev, () => { if (feature_Tour.at >= 0) feature_Tour.place(); });
    feature_Tour.check();
  }, 100);
}
