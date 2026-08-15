const feature_Ask = {
  norm(t) { return String(t).toLowerCase().match(/[a-z0-9]+/g) || []; },

  // query words: drop the little words ("do", "my", "me") unless that
  // would leave nothing to match on
  words(text) {
    const all = this.norm(text);
    const meaty = all.filter((w) => w.length > 2);
    return meaty.length ? meaty : all;
  },

  // find, grade one: tools already on the toolbar, by label words
  tools(words) {
    const out = [];
    for (const b of document.querySelectorAll('[data-ev^="tool_"]')) {
      const label = b.getAttribute('title') || '';
      if (this.norm(label).some((w) => words.includes(w)))
        out.push({ ev: b.getAttribute('data-ev'), label });
    }
    return out;
  },

  // find, grade one: features in the tree, by name/purpose/intro overlap
  async features(words) {
    if (typeof feature_Chooser === 'undefined') return [];
    await feature_Chooser.load();
    const scored = [];
    for (const n of feature_Chooser.flat) {
      const hay = this.norm(n.name + ' ' + (n.purpose || '') + ' ' + (n.intro || ''));
      const score = words.filter((w) => hay.includes(w)).length;
      if (score > 0) scored.push({ n, score });
    }
    scored.sort((a, b) => b.score - a.score);
    return scored.slice(0, 3).map((x) => x.n);
  },

  // file: the wish becomes state, travels, persists — provenance born
  file(text) {
    feature_Loop.send({ type: 'Ask', data: { t: Date.now(), text } });
  },

  async go() {
    const input = $('askText');
    const box = $('askResults');
    const text = (input.value || '').trim();
    if (!text || !box) return;
    const words = this.words(text);
    const tools = this.tools(words);
    const feats = await this.features(words);
    let html = '';
    if (tools.length)
      html += '<div class="askchips">' + tools.map((t) =>
        '<span class="askchip" data-open="' + t.ev + '">open ' + t.label + '</span>').join('') + '</div>';
    if (feats.length && typeof feature_Chooser !== 'undefined')
      html += feats.map((n) => {
        feature_Chooser.byPath[n.path] = n;
        return feature_Chooser.row(n);
      }).join('');
    if (html) {
      html += '<div class="askfile"><button id="askSend">not it? send to the builder</button></div>';
      box.innerHTML = html;
      // result rows introduce, they don't configure: the tick stays home
      for (const t of box.querySelectorAll('.ctick')) t.remove();
      $('askSend').onclick = () => {
        feature_Ask.file(text);
        box.innerHTML = '<div class="asknote">filed — the builder will see it</div>';
        input.value = '';
      };
    } else {
      this.file(text);
      box.innerHTML = '<div class="asknote">nothing here does that yet — filed for the builder</div>';
      input.value = '';
    }
  },
};
{
  const fm_panel = $('panel');
  if (fm_panel) {
    const fm_row = document.createElement('div');
    fm_row.id = 'askRow';
    fm_row.innerHTML =
      '<div class="askline">'
      + '<input id="askText" placeholder="ask muon — find a tool, or wish for one">'
      + '<button id="askGo">ask</button></div>'
      + '<div id="askResults"></div>';
    fm_panel.insertBefore(fm_row, $('changes'));
    $('askGo').onclick = () => feature_Ask.go();
    $('askText').addEventListener('keydown', (e) => {
      if (e.key === 'Enter') feature_Ask.go();
    });
    // the results strip speaks the chooser's own tap language (guarded:
    // without the chooser only the open-chips are ever rendered)
    $('askResults').addEventListener('click', (e) => {
      const chip = e.target.closest('[data-open]');
      if (chip) {
        // land in the tool: if it's already the open one, don't toggle it away
        const ev = chip.getAttribute('data-open');
        let open = '';
        try { open = JSON.parse(feature_Loop.state || '{}').open_tool || ''; } catch (err) {}
        if ('tool_' + open !== ev)
          feature_Loop.send({ type: 'click', ev });
        if (typeof feature_Panel !== 'undefined') feature_Panel.close();
        return;
      }
      if (typeof feature_Chooser === 'undefined') return;
      const read = e.target.closest('[data-read]');
      if (read) { feature_Chooser.reader(read.getAttribute('data-read')); return; }
      const row = e.target.closest('.crow[data-path]');
      if (row) feature_Chooser.more(row.getAttribute('data-path'));
    });
  }
}
