const feature_BeingBuilt = {
  building() {
    try {
      const s = JSON.parse(feature_Loop.state || '{}');
      const list = JSON.parse(s.asks || '[]');
      return Array.isArray(list)
        ? list.filter((a) => a.status === 'building')
        : [];
    } catch (e) { return []; }
  },

  render() {
    const box = $('changes');
    const old = document.getElementById('building');
    if (!box || !box.classList.contains('chooser-home')) { if (old) old.remove(); return; }
    const items = this.building().slice().sort((a, b) => (b.t || 0) - (a.t || 0));
    if (!items.length) { if (old) old.remove(); return; }
    const esc = feature_Lifecycle.esc.bind(feature_Lifecycle);
    const rows = items.map((a) =>
      '<div class="crow brow">'
      + '<span class="cnum bstatus">building</span>'
      + '<div class="ctext"><b>' + esc(a.text || '') + '</b>'
      + (a.proposal ? ' <span class="cpurpose">' + esc(a.proposal) + '</span>' : '')
      + '</div></div>').join('');
    const html = '<div class="awhead">being built</div>' + rows;
    if (old) { old.innerHTML = html; return; }
    const sect = document.createElement('div');
    sect.id = 'building';
    sect.innerHTML = html;
    const awaiting = document.getElementById('awaiting');
    if (awaiting) awaiting.after(sect);
    else box.prepend(sect);
  },
};
if (typeof feature_Lifecycle !== 'undefined') {
  const fm_beingBuiltRender = feature_Lifecycle.render.bind(feature_Lifecycle);
  feature_Lifecycle.render = function () {
    fm_beingBuiltRender();
    feature_BeingBuilt.render();
  };
}
