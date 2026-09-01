const feature_Audience = {
  // the six words, highest first — the same order the Rust half ranks by. A
  // list, not a form (/taste): six pills, one lit.
  GRADES: ['admin', 'candidate', 'team', 'volunteer', 'supporter', 'public'],
  DEFAULT: 'team',
  grade: 'team',
  row: null,

  // the option list, built once and living in /projects' sheet — which is
  // itself furniture outside #app, so a repaint cannot take it away.
  build(box, bar) {
    const row = document.createElement('div');
    row.id = 'projGrade';
    for (const g of this.GRADES) {
      const opt = document.createElement('span');
      opt.className = 'grade-opt';
      opt.setAttribute('data-grade', g);
      opt.textContent = g;
      // no data-ev: /loop's delegated click must not repaint #app out from
      // under an open sheet (the rule projects.js follows for its own rows)
      opt.addEventListener('click', (e) => {
        e.preventDefault();
        this.pick(g);
      });
      row.appendChild(opt);
    }
    box.insertBefore(row, bar);
    this.row = row;
    this.pick(this.DEFAULT);
  },

  pick(g) {
    this.grade = g;
    if (!this.row) return;
    for (const el of this.row.querySelectorAll('.grade-opt'))
      el.classList.toggle('on', el.getAttribute('data-grade') === g);
  },

  // the promote tap. Taken in the CAPTURE phase so /loop's delegated click
  // never sends a bare `click` on it, and sent from here because the event
  // needs a clock: there is none inside `update` (misses.md, the clock in
  // wasm).
  promote() {
    const page = document.querySelector('.card-page[data-card]');
    const id = page ? page.getAttribute('data-card') : '';
    if (!id) return;
    if (typeof feature_Loop === 'undefined' || !feature_Loop.state) return;
    feature_Loop.send({ type: 'PostPromote', data: { id, t: Date.now() } });
  },
};

{
  // /projects' sheet gains the option list, and its two functions gain the
  // grade — typeof-guarded, because a sibling being unticked is the absence
  // this half has to survive.
  if (typeof feature_Projects !== 'undefined') {
    const fm_audBox = document.getElementById('projBox');
    const fm_audBar = document.getElementById('projBar');
    if (fm_audBox && fm_audBar) feature_Audience.build(fm_audBox, fm_audBar);

    // the page-half twin of `projects_role_link`: the sheet says one more
    // thing about the person it has picked, and the send is untouched.
    const fm_audData = feature_Projects.roleData.bind(feature_Projects);
    feature_Projects.roleData = function () {
      const d = fm_audData();
      d.grade = feature_Audience.grade;
      return d;
    };

    // a sheet opened again is a fresh question: the rank goes back to the
    // one everybody already in a project holds.
    const fm_audOpen = feature_Projects.open.bind(feature_Projects);
    feature_Projects.open = function (cardId) {
      feature_Audience.pick(feature_Audience.DEFAULT);
      return fm_audOpen(cardId);
    };
  }

  document.addEventListener('click', (e) => {
    const hit = e.target && e.target.closest
      ? e.target.closest('[data-ev="posts_promote"]') : null;
    if (!hit) return;
    e.stopPropagation();
    e.preventDefault();
    feature_Audience.promote();
  }, true);
}
