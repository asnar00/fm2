const feature_DidYouMean = {
  questions() {
    try {
      const s = JSON.parse(feature_Loop.state || '{}');
      const list = JSON.parse(s.asks || '[]');
      return Array.isArray(list)
        ? list.filter((a) => a.status === 'question')
        : [];
    } catch (e) { return []; }
  },

  esc(t) {
    return String(t).replace(/&/g, '&amp;').replace(/</g, '&lt;');
  },

  // the feature-list grammar, one quiet row per question: the ask's own
  // words on top, the question under them, the readings as chips (#p39 —
  // no headers, no ceremony), the builder's hedge last when there is one
  rows(items) {
    return items.map((a) => {
      const q = a.question || {};
      const opts = Array.isArray(q.options) ? q.options : [];
      const chips = opts.map((o) =>
        '<span class="dymchip' + (o.key === q.likely ? ' dymlikely' : '')
        + '" data-ans="' + (a.t || 0) + '" data-choice="'
        + this.esc(o.key || '') + '">' + this.esc(o.label || o.key || '')
        + '</span>').join('');
      return '<div class="crow">'
        + '<span class="cnum dstatus">?</span>'
        + '<div class="ctext"><b>' + this.esc(a.text || '') + '</b></div>'
        + '</div>'
        + '<div class="dymblock">'
        + '<div class="dymq">' + this.esc(q.text || '') + '</div>'
        + (chips ? '<div class="dymchips">' + chips + '</div>' : '')
        + (a.note ? '<div class="dymnote">' + this.esc(a.note) + '</div>' : '')
        + '</div>';
    }).join('');
  },

  wire(sect) {
    if (sect.dataset.dymWired) return;
    sect.dataset.dymWired = '1';
    sect.addEventListener('click', (e) => {
      const chip = e.target.closest('.dymchip[data-ans]');
      if (!chip) return;
      const t = parseInt(chip.getAttribute('data-ans'), 10);
      const choice = chip.getAttribute('data-choice') || '';
      if (typeof feature_Loop === 'undefined') return;
      feature_Loop.send({ type: 'AskAnswer', data: { t, choice } });
    });
  },

  render() {
    const box = $('changes');
    const old = document.getElementById('didyoumean');
    if (!box || !box.classList.contains('chooser-home')) { if (old) old.remove(); return; }
    const items = this.questions().slice().sort((a, b) => (b.t || 0) - (a.t || 0));
    if (!items.length) { if (old) old.remove(); return; }
    const html = this.rows(items);
    if (old) { old.innerHTML = html; this.wire(old); return; }
    const sect = document.createElement('div');
    sect.id = 'didyoumean';
    sect.innerHTML = html;
    // the siblings' anchor dance: below the being-built block, above the
    // plain requests — and at the top of the box if neither is there
    const anchor = document.getElementById('building') || document.getElementById('awaiting');
    if (anchor) anchor.after(sect);
    else box.prepend(sect);
    this.wire(sect);
  },
};
if (typeof feature_Lifecycle !== 'undefined') {
  const fm_didYouMeanRender = feature_Lifecycle.render.bind(feature_Lifecycle);
  feature_Lifecycle.render = function () {
    fm_didYouMeanRender();
    feature_DidYouMean.render();
  };
}
