// The app stops guessing. /straight-through popped a guide when it thought
// the thing already existed — and next to a request that had gone straight to
// the builder, that read as two answers to one press. So the search does not
// run and no card is drawn; the ask simply sits at `asked` until a person
// looks at it. When the thing does already exist, that person answers, and
// the answer lands here: `answered`, quiet, with their words under the ask.
const feature_NoGuide = {
  // the asks a person has answered rather than built
  answered() {
    try {
      const s = JSON.parse(feature_Loop.state || '{}');
      const list = JSON.parse(s.asks || '[]');
      return Array.isArray(list) ? list.filter((a) => a.status === 'answered') : [];
    } catch (e) { return []; }
  },

  esc(t) { return String(t).replace(/&/g, '&amp;').replace(/</g, '&lt;'); },

  // the feature-list grammar (#p39): status where the number sits, the ask's
  // own words, the answer under them — no header, no ceremony. An answered
  // ask with no note is just a finished row.
  rows(items) {
    return items.map((a) =>
      '<div class="crow">'
      + '<span class="cnum astatus">answered</span>'
      + '<div class="ctext"><b>' + this.esc(a.text || '') + '</b></div>'
      + '</div>'
      + (a.note ? '<div class="ansblock">' + this.esc(a.note) + '</div>' : '')
    ).join('');
  },

  render() {
    const box = $('changes');
    const old = document.getElementById('answered');
    if (!box || !box.classList.contains('chooser-home')) { if (old) old.remove(); return; }
    const items = this.answered().slice().sort((a, b) => (b.t || 0) - (a.t || 0));
    if (!items.length) { if (old) old.remove(); return; }
    const html = this.rows(items);
    if (old) { old.innerHTML = html; return; }
    const sect = document.createElement('div');
    sect.id = 'answered';
    sect.innerHTML = html;
    // an answer is finished business: below the asks still becoming, and at
    // the top of the box when there are none of those
    const anchor = document.getElementById('requests')
      || document.getElementById('didyoumean')
      || document.getElementById('building')
      || document.getElementById('awaiting');
    if (anchor) anchor.after(sect);
    else box.prepend(sect);
  },
};
if (typeof feature_StraightThrough !== 'undefined') {
  // the narrowest seam that stops the whole suggestion: match() is where the
  // table is loaded, the query embedded and the catalog scored, and show() is
  // only ever reached through it. Nothing searches; nothing is drawn. Untick
  // and the guide comes back, /straight-through unedited.
  feature_StraightThrough.match = async function () { return null; };
}
if (typeof feature_Lifecycle !== 'undefined') {
  const fm_noGuideRender = feature_Lifecycle.render.bind(feature_Lifecycle);
  feature_Lifecycle.render = function () {
    fm_noGuideRender();
    feature_NoGuide.render();
  };
}
