const feature_Chooser = {
  open: false, tree: null,

  ticks() {
    try {
      const s = JSON.parse(feature_Loop.state || '{}');
      const t = JSON.parse(s.feature_ticks || '{}');
      return (t && typeof t === 'object') ? t : {};
    } catch (e) { return {}; }
  },

  async show() {
    this.open = true;
    $('chooserView').style.display = 'flex';
    this.reader(false);
    if (!this.tree) {
      this.tree = await fetch('features/tree.json', { cache: 'no-store' })
        .then((r) => r.ok ? r.json() : []).catch(() => []);
    }
    $('chooserList').innerHTML = this.tree.length
      ? this.rows(this.tree, 0)
      : '<div class="crow">no tree exported yet — deploy publishes it</div>';
    this.reflect();
  },

  rows(nodes, depth) {
    return nodes.map((n) => {
      const kids = n.children && n.children.length;
      const arrow = kids
        ? '<span class="carrow" data-cpath="' + n.path + '">›</span>'
        : '<span class="carrow leaf"></span>';
      return '<div class="crow" data-path="' + n.path + '" style="--depth:' + depth + '">'
        + arrow
        + '<div class="ctick" data-ev="ftick_' + n.path + '"></div>'
        + '<div class="ctext" data-read="' + n.path + '"><b>' + n.name + '</b>'
        + (n.purpose ? ' <span class="cpurpose">' + n.purpose.replace(/&/g, '&amp;').replace(/</g, '&lt;') + '</span>' : '')
        + '</div></div>'
        + (kids ? '<div class="ckids" data-kids="' + n.path + '" style="display:none">'
                  + this.rows(n.children, depth + 1) + '</div>' : '');
    }).join('');
  },

  // effective state: a row is shaded when any ancestor (or itself) is unticked
  reflect() {
    if (!this.open) return;
    const t = this.ticks();
    for (const row of document.querySelectorAll('#chooserList .crow')) {
      const path = row.getAttribute('data-path');
      const parts = path.split('/');
      let selfOn = t[path] !== false;
      let effOn = true;
      for (let i = 1; i <= parts.length; i++) {
        if (t[parts.slice(0, i).join('/')] === false) { effOn = false; break; }
      }
      row.querySelector('.ctick').classList.toggle('on', selfOn);
      row.classList.toggle('shaded', !effOn);
    }
  },

  reader(path) {
    $('chooserRead').style.display = path ? 'flex' : 'none';
    if (path) $('chooserFrame').src = 'features/' + path + '/';
    else $('chooserFrame').src = 'about:blank';
  },

  hide() {
    this.open = false;
    $('chooserView').style.display = 'none';
  },
};
{
  const fm_view = document.createElement('div');
  fm_view.id = 'chooserView';
  fm_view.innerHTML = '<div class="chead"><span>features</span>'
    + '<button id="chooserClose">✕</button></div>'
    + '<div id="chooserList"></div>'
    + '<div id="chooserRead"><div class="chead">'
    + '<button id="chooserBack">‹ tree</button></div>'
    + '<iframe id="chooserFrame"></iframe></div>';
  document.body.appendChild(fm_view);
  $('chooserClose').onclick = () => feature_Chooser.hide();
  $('chooserBack').onclick = () => feature_Chooser.reader(false);

  // expand/collapse and read taps (ticks ride data-ev to the loop as usual)
  fm_view.addEventListener('click', (e) => {
    const a = e.target.closest('[data-cpath]');
    if (a) {
      const kids = fm_view.querySelector('[data-kids="' + a.getAttribute('data-cpath') + '"]');
      const openNow = kids.style.display !== 'none';
      kids.style.display = openNow ? 'none' : 'block';
      a.classList.toggle('open', !openNow);
      return;
    }
    const r = e.target.closest('[data-read]');
    if (r) feature_Chooser.reader(r.getAttribute('data-read'));
  });

  const fm_row = document.createElement('div');
  fm_row.className = 'row';
  fm_row.innerHTML = '<button id="chooserBtn">features</button>';
  $('panel').insertBefore(fm_row, $('logoutBtn').closest('.row'));
  $('chooserBtn').onclick = () => { feature_Panel.close(); feature_Chooser.show(); };

  const fm_chooserApply = feature_Loop.apply;
  feature_Loop.apply = function (p) {
    fm_chooserApply.call(this, p);
    feature_Chooser.reflect();
  };
}
