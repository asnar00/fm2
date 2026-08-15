const feature_Lifecycle = {
  open: new Set(), // expanded rows (by ask timestamp), surviving re-renders

  asks() {
    try {
      const s = JSON.parse(feature_Loop.state || '{}');
      const list = JSON.parse(s.asks || '[]');
      return Array.isArray(list)
        ? list.filter((a) => a.status === 'asked' || a.status === 'proposed')
        : [];
    } catch (e) { return []; }
  },

  esc(t) {
    return String(t).replace(/&/g, '&amp;').replace(/</g, '&lt;');
  },

  // the feature-list grammar (#p39): status where the number sits, the title,
  // and the description one tap away — no headers, no ceremony
  rows(items, pillClass) {
    return items.map((a) =>
      '<div class="crow" data-req="' + (a.t || 0) + '">'
      + '<span class="cnum ' + pillClass + '">' + this.esc(a.status) + '</span>'
      + '<div class="ctext"><b>' + this.esc(a.text || '') + '</b></div>'
      + '</div>'
      + '<div class="cmore" style="display:' + (this.open.has(a.t) ? 'block' : 'none') + '">'
      + '<div class="cintrorow"><div class="cintro">'
      + this.esc(a.proposal || a.text || '') + '</div></div></div>').join('');
  },

  wire(sect) {
    if (sect.dataset.reqWired) return;
    sect.dataset.reqWired = '1';
    sect.addEventListener('click', (e) => {
      const row = e.target.closest('.crow[data-req]');
      if (!row) return;
      const t = parseInt(row.getAttribute('data-req'), 10);
      if (feature_Lifecycle.open.has(t)) feature_Lifecycle.open.delete(t);
      else feature_Lifecycle.open.add(t);
      feature_Lifecycle.render();
    });
  },

  render() {
    const box = $('changes');
    const old = document.getElementById('requests');
    if (!box || !box.classList.contains('chooser-home')) { if (old) old.remove(); return; }
    const asks = this.asks().slice().sort((a, b) => (b.t || 0) - (a.t || 0));
    if (!asks.length) { if (old) old.remove(); return; }
    const html = this.rows(asks, 'lstatus');
    if (old) { old.innerHTML = html; this.wire(old); return; }
    const sect = document.createElement('div');
    sect.id = 'requests';
    sect.innerHTML = html;
    const anchor = document.getElementById('building') || document.getElementById('awaiting');
    if (anchor) anchor.after(sect);
    else box.prepend(sect);
    this.wire(sect);
  },
};
{
  if (typeof feature_Chooser !== 'undefined') {
    const fm_lifecycleMount = feature_Chooser.mount.bind(feature_Chooser);
    feature_Chooser.mount = async function () {
      await fm_lifecycleMount();
      feature_Lifecycle.render();
    };
  }
  const fm_lifecycleApply = feature_Loop.apply;
  feature_Loop.apply = function (p) {
    fm_lifecycleApply.call(this, p);
    const panel = $('panel');
    if (panel && panel.style.display === 'block') feature_Lifecycle.render();
  };
}
