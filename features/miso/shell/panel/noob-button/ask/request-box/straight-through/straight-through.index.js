// The box used to put its own doubt between the asker and the builder:
// press miso, read three guesses, decide none of them is it, press send.
// Now the words go through at once and the search runs afterwards on the
// filed text — only to say "miso already does that", by popping the
// feature's own guide over the sheet. The ask stays filed either way.
const feature_StraightThrough = {
  // a strong match, not merely a match. /semantic-find lists a result at
  // 0.28; interrupting someone with a guide asks for more. Over 32 asks
  // against the 379-node catalog (16 things miso does, 16 it does not) the
  // real ones scored 0.453–0.827 and the absent ones 0.264–0.587; 0.50
  // pops 12 of the 16 real and 2 of the 16 absent.
  strong: 0.50,
  turn: 0,

  // the words for a hit: a tool's CURRENT line where /tool-words has one
  // (so this card and the long-press card never disagree), else the node's
  // own user paragraph, else its purpose
  guideFor(n) {
    if (typeof feature_ToolWords !== 'undefined' && n.tool) {
      const w = feature_ToolWords.words('tool_' + n.tool);
      if (w && w.intro) return { name: w.name, intro: w.intro };
    }
    return { name: n.name, intro: n.intro || n.purpose || '' };
  },

  card() {
    let c = document.getElementById('askGuide');
    if (!c) {
      c = document.createElement('div');
      c.id = 'askGuide';
      c.innerHTML = '<div class="agtop"><b></b><button class="agx">✕</button></div>'
        + '<div class="agtext"></div>';
      document.body.appendChild(c);
      c.querySelector('.agx').onclick = () => feature_StraightThrough.hide();
    }
    return c;
  },

  // the exported intro is cut at 400 characters, which can land mid-sentence:
  // end on the last full stop rather than in the middle of a word
  tidy(t) {
    const s = String(t).trim();
    if (/[.!?]$/.test(s)) return s;
    let cut = -1;
    for (const end of ['. ', '! ', '? ']) cut = Math.max(cut, s.lastIndexOf(end));
    return cut > 40 ? s.slice(0, cut + 1) : s + '…';
  },

  // the tree's prose carries markdown emphasis (a **bin**, the word *sure?*):
  // draw it instead of printing the asterisks. Text nodes only — the words
  // come from the catalog, and nothing here builds markup from them.
  emphasise(el, text) {
    el.textContent = '';
    const re = /\*\*([^*]+)\*\*|\*([^*]+)\*/g;
    let at = 0, m;
    while ((m = re.exec(text)) !== null) {
      if (m.index > at) el.appendChild(document.createTextNode(text.slice(at, m.index)));
      const tag = document.createElement(m[1] ? 'b' : 'i');
      tag.textContent = m[1] || m[2];
      el.appendChild(tag);
      at = m.index + m[0].length;
    }
    if (at < text.length) el.appendChild(document.createTextNode(text.slice(at)));
  },

  // under the box you typed in, not over it: the sheet stays the context
  place(c) {
    const row = document.getElementById('askRow');
    const top = row ? row.getBoundingClientRect().bottom + 8 : innerHeight * 0.12;
    const at = Math.max(8, Math.min(Math.round(top), Math.round(innerHeight - 120)));
    c.style.top = at + 'px';
    c.style.maxHeight = Math.max(120, Math.round(innerHeight - at - 16)) + 'px';
  },

  // a card holding a name and no words is noise: say nothing instead
  show(n) {
    const g = this.guideFor(n);
    if (!g.intro) return false;
    const c = this.card();
    c.querySelector('b').textContent = g.name;
    this.emphasise(c.querySelector('.agtext'), this.tidy(g.intro));
    this.place(c);
    c.classList.add('on');
    return true;
  },

  hide() {
    const c = document.getElementById('askGuide');
    if (c) c.classList.remove('on');
  },

  // is the sheet the ask was typed into still up?
  sheetUp() {
    const p = document.getElementById('panel');
    return !!(p && getComputedStyle(p).display !== 'none');
  },

  // the single strongest catalog hit for these words, or null. Guarded and
  // caught throughout: a search that fails is a search that says nothing —
  // the ask is already filed by the time this runs.
  async match(words) {
    if (typeof feature_SemanticFind === 'undefined') return null;
    if (typeof feature_Chooser === 'undefined') return null;
    const sf = feature_SemanticFind;
    try {
      await sf.load();
      if (!sf.ready || !sf.paths || !sf.paths.length) return null;
      const scores = await sf.score(sf.embed(words.join(' ')));
      if (!scores) return null;
      let best = -1, at = -1;
      for (let m = 0; m < sf.paths.length; m++)
        if (scores[m] > best) { best = scores[m]; at = m; }
      if (at < 0 || best < this.strong) return null;
      await feature_Chooser.load();
      return feature_Chooser.byPath[sf.paths[at]] || null;
    } catch (e) { return null; }
  },
};
if (typeof feature_Ask !== 'undefined') {
  // /ask's send seam: the whole road, redefined
  feature_Ask.send = async function (text) {
    const st = feature_StraightThrough;
    st.hide();
    // file first — the ask is provenance the moment it is typed, and
    // nothing that follows can keep it from the builder
    this.file(text);
    const input = document.getElementById('askText');
    if (input) input.value = '';
    const box = document.getElementById('askResults');
    if (box) box.innerHTML = '';
    // then look, and only to say "this exists". The table loads on the
    // first ask ever (~8MB), so this can answer long after the filing:
    // a guide must not arrive over a screen the asker has moved on to.
    const turn = ++st.turn;
    const n = await st.match(this.words(text));
    if (n && turn === st.turn && st.sheetUp()) st.show(n);
  };
  // the placeholder: what the box is for, in the asker's own words (the
  // parent built the row already — fragments load in provenance order)
  const fm_stBox = document.getElementById('askText');
  if (fm_stBox) fm_stBox.setAttribute('placeholder', 'request a fix, tweak or feature');
}
if (typeof feature_Panel !== 'undefined') {
  // the guide never outlives the sheet it came up over
  const fm_stClose = feature_Panel.close.bind(feature_Panel);
  feature_Panel.close = function () {
    feature_StraightThrough.hide();
    return fm_stClose();
  };
}
