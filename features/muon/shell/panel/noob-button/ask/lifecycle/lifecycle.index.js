const feature_Lifecycle = {
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

  render() {
    const box = $('changes');
    const old = document.getElementById('requests');
    if (!box || !box.classList.contains('chooser-home')) { if (old) old.remove(); return; }
    const asks = this.asks().slice().sort((a, b) => (b.t || 0) - (a.t || 0));
    if (!asks.length) { if (old) old.remove(); return; }
    const rows = asks.map((a) =>
      '<div class="crow lrow">'
      + '<span class="cnum lstatus">' + this.esc(a.status) + '</span>'
      + '<div class="ctext"><b>' + this.esc(a.text || '') + '</b>'
      + (a.proposal ? ' <span class="cpurpose">' + this.esc(a.proposal) + '</span>' : '')
      + '</div></div>').join('');
    const html = '<div class="awhead">requests — yours, becoming</div>' + rows;
    if (old) { old.innerHTML = html; return; }
    const sect = document.createElement('div');
    sect.id = 'requests';
    sect.innerHTML = html;
    const awaiting = document.getElementById('awaiting');
    if (awaiting) awaiting.after(sect);
    else box.prepend(sect);
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
