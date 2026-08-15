const feature_Chooser = {
  flat: null, byPath: null,

  ticks() {
    try {
      const s = JSON.parse(feature_Loop.state || '{}');
      const t = JSON.parse(s.feature_ticks || '{}');
      return (t && typeof t === 'object') ? t : {};
    } catch (e) { return {}; }
  },

  async load() {
    if (this.flat) return;
    const tree = await fetch('features/tree.json', { cache: 'no-store' })
      .then((r) => r.ok ? r.json() : []).catch(() => []);
    this.flat = []; this.byPath = {};
    const walk = (nodes, parent) => {
      for (const n of nodes) {
        n.parent = parent;
        this.flat.push(n); this.byPath[n.path] = n;
        walk(n.children, n.path);
      }
    };
    walk(tree, '');
    // most recent first; the number shown is the BUILD that introduced the
    // feature — the same number the release list spoke (#p77)
    this.flat.sort((a, b) => (a.ts < b.ts ? 1 : a.ts > b.ts ? -1 : a.path < b.path ? 1 : -1));
  },

  esc(t) { return String(t).replace(/&/g, '&amp;').replace(/</g, '&lt;'); },

  // the panel's seam calls this: the feature list lives under the who-line
  // (#p78 — the version teaser replaced; the sheet stretches to fit)
  async mount() {
    const box = $('changes');
    if (!box) return;
    box.classList.add('chooser-home');
    box.onclick = null; // the old full-queue tap (if composed) stands down
    box.classList.remove('expandable');
    await this.load();
    box.innerHTML = this.flat.length
      ? this.flat.map((n) => this.row(n)).join('')
      : '<div class="crow">no tree exported yet — deploy publishes it</div>';
    if (!box.dataset.chooserWired) {
      box.dataset.chooserWired = '1';
      box.addEventListener('click', (e) => {
        if (e.target.closest('.ctick')) return; // the tick is the loop's business
        const up = e.target.closest('[data-up]');
        if (up) { feature_Chooser.goto(up.getAttribute('data-up')); return; }
        const chip = e.target.closest('[data-goto]');
        if (chip) { feature_Chooser.goto(chip.getAttribute('data-goto')); return; }
        const read = e.target.closest('[data-read]');
        if (read) { feature_Chooser.reader(read.getAttribute('data-read')); return; }
        const row = e.target.closest('.crow[data-path]');
        if (row) feature_Chooser.more(row.getAttribute('data-path'));
      });
    }
    this.reflect();
  },

  row(n) {
    return '<div class="crow" data-path="' + n.path + '" id="crow-' + n.path.replace(/\//g, '-') + '">'
      + '<span class="cnum">' + (n.build || '·') + '</span>'
      + '<div class="ctext"><b>' + n.name + '</b>'
      + (n.purpose ? ' <span class="cpurpose">' + this.esc(n.purpose) + '</span>' : '')
      + '</div>'
      + '<div class="ctick" data-ev="ftick_' + n.path + '"></div>'
      + '</div>'
      + '<div class="cmore" data-morebox="' + n.path + '" style="display:none"></div>';
  },

  // tap-the-line expansion: ‹ (up a level) beside the tappable user
  // paragraph, child chips below to drill down
  more(path) {
    const box = document.querySelector('[data-morebox="' + path + '"]');
    if (box.style.display !== 'none') { box.style.display = 'none'; return; }
    const n = this.byPath[path];
    box.innerHTML =
      '<div class="cintrorow">'
      + (n.parent ? '<span class="cup" data-up="' + n.parent + '">‹</span>' : '')
      + '<div class="cintro" data-read="' + n.path + '">'
      + this.esc(n.intro || '(tap to read this feature’s page)') + '</div>'
      + '</div>'
      + (n.children.length
        ? '<div class="cchips">' + n.children.map((c) =>
            '<span class="cchip" data-goto="' + c.path + '">' + c.name + '</span>').join('') + '</div>'
        : '');
    box.style.display = 'block';
  },

  goto(path) {
    const row = document.getElementById('crow-' + path.replace(/\//g, '-'));
    if (!row) return;
    row.scrollIntoView({ block: 'center' });
    row.classList.add('cflash');
    setTimeout(() => row.classList.remove('cflash'), 900);
    const box = document.querySelector('[data-morebox="' + path + '"]');
    if (box && box.style.display === 'none') this.more(path);
  },

  reflect() {
    const list = $('changes');
    if (!list || !list.classList.contains('chooser-home')) return;
    const t = this.ticks();
    for (const row of list.querySelectorAll('.crow')) {
      const path = row.getAttribute('data-path');
      if (!path) continue;
      const parts = path.split('/');
      let effOn = true;
      for (let i = 1; i <= parts.length; i++) {
        if (t[parts.slice(0, i).join('/')] === false) { effOn = false; break; }
      }
      row.querySelector('.ctick').classList.toggle('on', t[path] !== false);
      row.classList.toggle('shaded', !effOn);
    }
  },

  // the full node page, in place over everything; ✕ dismisses
  reader(path) {
    $('chooserRead').style.display = path ? 'flex' : 'none';
    $('chooserFrame').src = path ? 'features/' + path + '/' : 'about:blank';
  },
};
{
  const fm_read = document.createElement('div');
  fm_read.id = 'chooserRead';
  fm_read.innerHTML = '<div class="chead"><span></span>'
    + '<button id="chooserDismiss">✕</button></div>'
    + '<iframe id="chooserFrame"></iframe>';
  document.body.appendChild(fm_read);
  $('chooserDismiss').onclick = () => feature_Chooser.reader(false);

  const fm_chooserApply = feature_Loop.apply;
  feature_Loop.apply = function (p) {
    fm_chooserApply.call(this, p);
    feature_Chooser.reflect();
  };
}
