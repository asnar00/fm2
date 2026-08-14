const feature_Queue = {
  open: false,

  ticks() {
    try {
      const s = JSON.parse(feature_Loop.state || '{}');
      const t = JSON.parse(s.update_ticks || '{}');
      return (t && typeof t === 'object') ? t : {};
    } catch (e) { return {}; }
  },

  ticked(build) {
    const t = this.ticks();
    return t[String(build)] !== false; // absent = default: ticked
  },

  async show() {
    this.open = true;
    const box = $('queueView');
    box.style.display = 'flex';
    const changes = await fetch('changes.json', { cache: 'no-store' })
      .then((r) => r.ok ? r.json() : []).catch(() => []);
    const rows = changes.map((c) => {
      const feature = c.kind === 'feature'; // unknown kind counts as fix, like /policy
      const tickable = feature
        ? '<div class="qtick" data-ev="qtick_' + c.build + '"></div>'
        : '<div class="qtick static"></div>';
      return '<div class="qrow" data-build="' + c.build + '">' + tickable
        + '<div class="qkind ' + (feature ? 'feature' : 'fix') + '">'
        + (feature ? 'feature' : 'fix') + '</div>'
        + '<div class="qtext"><b>' + c.build + '</b> '
        + String(c.text).replace(/&/g, '&amp;').replace(/</g, '&lt;') + '</div></div>';
    });
    $('queueList').innerHTML = rows.join('') || '<div class="qrow">no recorded changes</div>';
    this.reflect();
  },

  hide() {
    this.open = false;
    $('queueView').style.display = 'none';
  },

  // tick states come from loop state, so a toggle on another device (or the
  // authoritative echo of ours) moves the boxes here
  reflect() {
    if (!this.open) return;
    for (const el of document.querySelectorAll('#queueList .qtick[data-ev]')) {
      const build = el.getAttribute('data-ev').slice(6);
      el.classList.toggle('on', this.ticked(build));
    }
  },
};
{
  const fm_view = document.createElement('div');
  fm_view.id = 'queueView';
  fm_view.innerHTML = '<div class="qhead"><span>every update</span>'
    + '<button id="queueClose">✕</button></div>'
    + '<div id="queueList"></div>';
  document.body.appendChild(fm_view);
  $('queueClose').onclick = () => feature_Queue.hide();

  // the panel's changes teaser is the way in
  const fm_changes = $('changes');
  if (fm_changes) {
    fm_changes.classList.add('expandable');
    fm_changes.onclick = () => feature_Queue.show();
  }

  const fm_queueApply = feature_Loop.apply;
  feature_Loop.apply = function (p) {
    fm_queueApply.call(this, p);
    feature_Queue.reflect();
  };
}
