const feature_Readout = {
  active: /(^|[?&])readout=/.test(location.search),
  timer: null,
  capture(node) {
    if (!node || node.nodeType !== 1) return null;
    if (node.tagName === 'SCRIPT' || node.tagName === 'STYLE') return null;
    const out = { tag: node.tagName.toLowerCase() };
    if (node.id) out.id = node.id;
    if (typeof node.className === 'string' && node.className.trim())
      out.cls = node.className.trim();
    const ev = node.getAttribute && node.getAttribute('data-ev');
    if (ev) out.ev = ev;
    const cs = getComputedStyle(node);
    if (cs.display === 'none' || cs.visibility === 'hidden') out.hidden = true;
    if (node.tagName === 'INPUT') out.value = node.value;
    const kids = [];
    for (const c of node.children) {
      const k = this.capture(c);
      if (k) kids.push(k);
    }
    if (kids.length) out.kids = kids;
    else {
      const t = (node.textContent || '').trim();
      if (t) out.text = t.slice(0, 200);
    }
    return out;
  },
  post() {
    fetch('/diag/readout', { method: 'POST', body: JSON.stringify({
      t: new Date().toISOString(),
      url: location.pathname + location.search,
      body: this.capture(document.body) }) }).catch(() => {});
  },
  schedule() {
    clearTimeout(this.timer);
    this.timer = setTimeout(() => this.post(), 250);
  },
};
if (feature_Readout.active) {
  new MutationObserver(() => feature_Readout.schedule()).observe(
    document.documentElement,
    { subtree: true, childList: true, characterData: true, attributes: true });
  addEventListener('load', () => feature_Readout.schedule());
  feature_Readout.schedule();
}
