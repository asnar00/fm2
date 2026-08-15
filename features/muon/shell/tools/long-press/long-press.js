const feature_LongPress = {
  timer: null, armed: null, fired: false, x: 0, y: 0,

  card() {
    let c = document.getElementById('toolCard');
    if (!c) {
      c = document.createElement('div');
      c.id = 'toolCard';
      c.style.display = 'none';
      document.body.appendChild(c);
    }
    return c;
  },

  async contentFor(btn) {
    const ev = btn.getAttribute('data-ev') || '';
    const id = ev.replace(/^tool_/, '');
    const fallback = btn.getAttribute('title') || id;
    if (typeof feature_Chooser === 'undefined') return { name: fallback, intro: '' };
    try {
      await feature_Chooser.load();
      const n = feature_Chooser.flat.find((x) => x.tool === id);
      if (n) return { name: n.name, intro: n.intro || n.purpose || '' };
    } catch (e) {}
    return { name: fallback, intro: '' };
  },

  async show(btn) {
    this.fired = true;
    const { name, intro } = await this.contentFor(btn);
    const c = this.card();
    c.innerHTML = '<b></b>' + (intro ? '<div class="tcintro"></div>' : '');
    c.querySelector('b').textContent = name;
    if (intro) c.querySelector('.tcintro').textContent = intro;
    c.style.display = 'block';
    const r = btn.getBoundingClientRect();
    const cw = c.offsetWidth;
    let left = r.left + r.width / 2 - cw / 2;
    left = Math.max(8, Math.min(left, innerWidth - cw - 8));
    c.style.left = left + 'px';
    c.style.top = Math.max(8, r.top - c.offsetHeight - 10) + 'px';
  },

  hide() {
    const c = document.getElementById('toolCard');
    if (c) c.style.display = 'none';
  },

  disarm() {
    if (this.timer) clearTimeout(this.timer);
    this.timer = null;
    this.armed = null;
  },
};
{
  document.addEventListener('pointerdown', (e) => {
    const open = document.getElementById('toolCard');
    if (open && open.style.display !== 'none' && !e.target.closest('#toolCard'))
      feature_LongPress.hide();
    const btn = e.target.closest('[data-ev^="tool_"]');
    if (!btn) return;
    feature_LongPress.disarm();
    feature_LongPress.fired = false;
    feature_LongPress.armed = btn;
    feature_LongPress.x = e.clientX;
    feature_LongPress.y = e.clientY;
    feature_LongPress.timer = setTimeout(() => feature_LongPress.show(btn), 500);
  });
  document.addEventListener('pointermove', (e) => {
    if (!feature_LongPress.armed) return;
    if (Math.hypot(e.clientX - feature_LongPress.x, e.clientY - feature_LongPress.y) > 12)
      feature_LongPress.disarm(); // a scroll is not a question
  });
  for (const ev of ['pointerup', 'pointercancel'])
    document.addEventListener(ev, () => feature_LongPress.disarm());
  // a long press reads; it must not also open — swallow that one click
  document.addEventListener('click', (e) => {
    if (feature_LongPress.fired && e.target.closest('[data-ev^="tool_"]')) {
      e.stopPropagation();
      e.preventDefault();
      feature_LongPress.fired = false;
    }
  }, true);
}
