const feature_OpenChip = {
  // the nearest tool in a node's lineage: self, ancestors, then descendants
  toolFor(path) {
    const byPath = typeof feature_Chooser !== 'undefined' ? feature_Chooser.byPath : null;
    if (!byPath) return null;
    for (let p = path; p; ) {
      const n = byPath[p];
      if (!n) break;
      if (n.tool) return n.tool;
      p = n.parent;
    }
    const queue = byPath[path] ? [...byPath[path].children] : [];
    while (queue.length) {
      const n = queue.shift();
      if (n.tool) return n.tool;
      queue.push(...n.children);
    }
    return null;
  },

  dress() {
    const box = $('askResults');
    if (!box) return;
    const have = new Set([...box.querySelectorAll('[data-open]')]
      .map((c) => c.getAttribute('data-open')));
    const catalog = new Map(feature_Ask.catalog().map((t) => [t.ev, t.label]));
    const chips = [];
    for (const row of box.querySelectorAll('.crow[data-path]')) {
      const tool = this.toolFor(row.getAttribute('data-path'));
      if (!tool || have.has('tool_' + tool)) continue;
      if (!catalog.has('tool_' + tool)) continue; // stamped, but not composed here
      have.add('tool_' + tool);
      chips.push({ ev: 'tool_' + tool, label: catalog.get('tool_' + tool) || tool });
    }
    if (!chips.length) return;
    let strip = box.querySelector('.askchips');
    if (!strip) {
      strip = document.createElement('div');
      strip.className = 'askchips';
      box.prepend(strip);
    }
    for (const c of chips) {
      const chip = document.createElement('span');
      chip.className = 'askchip';
      chip.setAttribute('data-open', c.ev);
      chip.textContent = 'open ' + c.label;
      strip.appendChild(chip);
    }
  },
};
if (typeof feature_Ask !== 'undefined') {
  const fm_openChipGo = feature_Ask.go.bind(feature_Ask);
  feature_Ask.go = async function () {
    await fm_openChipGo();
    feature_OpenChip.dress();
  };
}
