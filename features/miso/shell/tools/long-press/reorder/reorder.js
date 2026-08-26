const feature_Reorder = {
  held: null, buttons: [], centers: [], origIdx: -1, newIdx: -1,
  x: 0, y: 0, dragging: false, dragged: false, pending: null, pointer: 0,

  // the launcher row is the only row that reorders: open_tool empty, and the
  // toolbar showing plain tool buttons (no ‹, no controls).
  launcher() {
    if (typeof feature_Loop === 'undefined' || !feature_Loop.state) return false;
    try {
      return !JSON.parse(feature_Loop.state).open_tool;
    } catch (e) {
      return false;
    }
  },

  toolOf(el) {
    return el && el.closest ? el.closest('.toolbar .tool-button[data-ev^="tool_"]') : null;
  },

  idOf(btn) {
    return (btn.getAttribute('data-ev') || '').replace(/^tool_/, '');
  },

  // measure once, at the moment the drag begins: the row does not re-render
  // while a finger is down, so these positions stay true for the whole drag.
  begin(btn) {
    const bar = btn.closest('.toolbar');
    if (!bar) return false;
    this.buttons = Array.from(bar.querySelectorAll('.tool-button[data-ev^="tool_"]'));
    if (this.buttons.length < 2) return false;
    this.origIdx = this.buttons.indexOf(btn);
    if (this.origIdx < 0) return false;
    this.centers = this.buttons.map((b) => {
      const r = b.getBoundingClientRect();
      return r.left + r.width / 2;
    });
    this.held = btn;
    this.newIdx = this.origIdx;
    this.dragging = true;
    if (typeof feature_LongPress !== 'undefined') feature_LongPress.hide();
    btn.classList.add('dragging');
    for (const b of this.buttons) if (b !== btn) b.classList.add('shifting');
    return true;
  },

  // the held button follows the finger; the others part to open a slot
  move(dx) {
    const lo = this.centers[0] - this.centers[this.origIdx];
    const hi = this.centers[this.centers.length - 1] - this.centers[this.origIdx];
    const d = Math.max(lo, Math.min(hi, dx));
    this.held.style.transform = 'translateX(' + d + 'px) scale(1.08)';
    const at = this.centers[this.origIdx] + d;
    let best = 0;
    for (let i = 1; i < this.centers.length; i++)
      if (Math.abs(this.centers[i] - at) < Math.abs(this.centers[best] - at)) best = i;
    if (best !== this.newIdx) {
      this.newIdx = best;
      this.slot();
    }
  },

  slot() {
    for (let i = 0; i < this.buttons.length; i++) {
      if (i === this.origIdx) continue;
      let shift = 0;
      if (this.newIdx > this.origIdx && i > this.origIdx && i <= this.newIdx)
        shift = this.centers[i - 1] - this.centers[i];
      if (this.newIdx < this.origIdx && i >= this.newIdx && i < this.origIdx)
        shift = this.centers[i + 1] - this.centers[i];
      this.buttons[i].style.transform = shift ? 'translateX(' + shift + 'px)' : '';
    }
  },

  clear() {
    for (const b of this.buttons) {
      b.style.transform = '';
      b.classList.remove('dragging', 'shifting');
    }
    this.buttons = []; this.centers = []; this.held = null;
    this.dragging = false; this.origIdx = -1; this.newIdx = -1;
  },

  // on release: the row as arranged, sent whole. The repaint that follows
  // renders it from the var, so the transforms are cleared first.
  drop() {
    const ids = this.buttons.map((b) => this.idOf(b));
    const moved = this.newIdx !== this.origIdx;
    if (moved) {
      const id = ids.splice(this.origIdx, 1)[0];
      ids.splice(this.newIdx, 0, id);
      this.dragged = true;
    }
    this.clear();
    if (moved && typeof feature_Loop !== 'undefined')
      feature_Loop.send({ type: 'ToolOrder', data: { order: ids } });
  },
};
{
  document.addEventListener('pointerdown', (e) => {
    feature_Reorder.dragged = false;
    if (feature_Reorder.dragging) feature_Reorder.clear();
    const btn = feature_Reorder.toolOf(e.target);
    if (!btn || !feature_Reorder.launcher()) return;
    feature_Reorder.pending = btn;
    feature_Reorder.x = e.clientX;
    feature_Reorder.y = e.clientY;
    feature_Reorder.pointer = e.pointerId;
  });
  document.addEventListener('pointermove', (e) => {
    if (feature_Reorder.dragging) {
      feature_Reorder.move(e.clientX - feature_Reorder.x);
      e.preventDefault();
      return;
    }
    if (!feature_Reorder.pending) return;
    const dx = e.clientX - feature_Reorder.x, dy = e.clientY - feature_Reorder.y;
    if (Math.hypot(dx, dy) <= 12) return;
    // past the drift threshold: along the row, and only once the hold has
    // shown its card, this becomes a drag. Anything else is the parent's
    // disarm, untouched — a scroll is still not a question.
    const held = typeof feature_LongPress !== 'undefined' && feature_LongPress.fired;
    if (!held || Math.abs(dx) <= Math.abs(dy) || !feature_Reorder.begin(feature_Reorder.pending)) {
      feature_Reorder.pending = null;
      return;
    }
    try { feature_Reorder.held.setPointerCapture(feature_Reorder.pointer); } catch (err) {}
    feature_Reorder.move(dx);
    e.preventDefault();
  });
  for (const ev of ['pointerup', 'pointercancel'])
    document.addEventListener(ev, () => {
      feature_Reorder.pending = null;
      if (feature_Reorder.dragging) feature_Reorder.drop();
    });
  // a drag rearranges; it must not also open. The parent swallows the click
  // when it lands on a tool button; this covers a release that ends anywhere
  // else, and clears the parent's flag so the next tap is an ordinary tap.
  document.addEventListener('click', (e) => {
    if (!feature_Reorder.dragged) return;
    feature_Reorder.dragged = false;
    if (typeof feature_LongPress !== 'undefined') feature_LongPress.fired = false;
    e.stopPropagation();
    e.preventDefault();
  }, true);
}
