const feature_Keep = {
  // the moment after typing stops at which the words are kept, and the hold
  // that turns a press on a picture into a question about it
  QUIET: 600,
  HOLD: 500,
  DRIFT: 12,

  timer: null,
  editing: null,   // {card, block} — re-resolved at fire time, never a stale node
  painting: false,
  hold_t: null,
  armed: null,
  fired: false,
  x: 0,
  y: 0,
  pill: null,
  target: null,

  // ---- a repaint keeps the block you are editing ------------------------

  // one card block, by the pair that identifies it in either DOM
  find(card, block) {
    const q = window.CSS && CSS.escape ? CSS.escape(card) : card;
    try {
      return document.querySelector('[data-card="' + q + '"][data-block="' + block + '"]');
    } catch (e) {
      return null;
    }
  },

  // the caret as a character offset from the start of the element. Only text
  // nodes are counted, which is exactly what putCaret walks, so the two agree
  // even when the browser has made <div>s and <br>s inside.
  caretOf(el) {
    const sel = document.getSelection && document.getSelection();
    if (!sel || !sel.rangeCount) return null;
    const r = sel.getRangeAt(0);
    if (!el.contains(r.startContainer)) return null;
    const pre = r.cloneRange();
    pre.selectNodeContents(el);
    pre.setEnd(r.startContainer, r.startOffset);
    return pre.toString().length;
  },

  putCaret(el, off) {
    const sel = document.getSelection && document.getSelection();
    if (!sel) return;
    const r = document.createRange();
    let seen = 0, placed = false;
    const walk = document.createTreeWalker(el, NodeFilter.SHOW_TEXT, null);
    let n;
    while ((n = walk.nextNode())) {
      const len = n.nodeValue.length;
      if (seen + len >= off) {
        r.setStart(n, Math.max(0, off - seen));
        placed = true;
        break;
      }
      seen += len;
    }
    if (!placed) {
      r.selectNodeContents(el);
      r.collapse(false);
    } else {
      r.collapse(true);
    }
    sel.removeAllRanges();
    sel.addRange(r);
  },

  // what is under the repaint: the focused card block, its live content as
  // cloned nodes (never serialized, so nothing is re-parsed on the way back),
  // and where the caret sits in it. null when nothing is being edited.
  hold() {
    const el = document.activeElement;
    if (!el || !el.getAttribute) return null;
    if (el.getAttribute('contenteditable') !== 'true') return null;
    const block = el.getAttribute('data-block');
    const card = el.getAttribute('data-card');
    if (block === null || card === null) return null;
    const nodes = document.createDocumentFragment();
    for (const c of Array.from(el.childNodes)) nodes.appendChild(c.cloneNode(true));
    return { card, block, nodes, text: el.innerText || '', caret: this.caretOf(el) };
  },

  // put the words, the caret and the focus back into the same card and block
  // of the freshly painted DOM. If the repaint took the card away there is
  // nothing to restore into, and that is not an error.
  restore(held) {
    if (!held) return true;
    const el = this.find(held.card, held.block);
    if (!el || el.getAttribute('contenteditable') !== 'true') return false;
    while (el.firstChild) el.removeChild(el.firstChild);
    el.appendChild(held.nodes);
    el.focus();
    this.putCaret(el, held.caret === null ? Infinity : held.caret);
    return true;
  },

  // the block is not on the screen any more — the tool was closed, the card
  // opened away — and the focusout that /cards saves on was swallowed as part
  // of the repaint. The words still belong to the card, so send them, once,
  // after the paint has finished rather than re-entering it.
  rescue(held) {
    setTimeout(() => {
      if (typeof feature_Loop === 'undefined' || !feature_Loop.state) return;
      feature_Loop.send({ type: 'CardEdit', data: {
        id: held.card, i: Number(held.block),
        text: (held.text || '').trim(), t: Date.now() } });
    }, 0);
  },

  // ---- save as you type -------------------------------------------------

  send(el) {
    if (!el) return;
    if (typeof feature_Loop === 'undefined' || !feature_Loop.state) return;
    feature_Loop.send({ type: 'CardEdit', data: {
      id: el.getAttribute('data-card'),
      i: Number(el.getAttribute('data-block')),
      text: (el.innerText || '').trim(),
      t: Date.now() } });
  },

  // reset, never queue: two keystrokes inside the quiet window are one send
  // carrying the later text. The block is remembered by card and block rather
  // than by node, because a repaint in between replaces the node.
  soon(el) {
    if (this.timer) clearTimeout(this.timer);
    this.editing = { card: el.getAttribute('data-card'),
                     block: el.getAttribute('data-block') };
    this.timer = setTimeout(() => {
      this.timer = null;
      const at = this.editing;
      this.editing = null;
      if (at) this.send(this.find(at.card, at.block));
    }, this.QUIET);
  },

  // tap-away is /cards' own immediate save; drop ours so the same text is
  // never sent twice.
  cancel() {
    if (this.timer) clearTimeout(this.timer);
    this.timer = null;
    this.editing = null;
  },

  // ---- long-press the picture -------------------------------------------

  show(pic) {
    this.fired = true;
    this.target = { id: pic.getAttribute('data-card'),
                    i: Number(pic.getAttribute('data-block')) };
    const p = this.pill;
    if (!p) return;
    p.style.display = 'block';
    // centred ON the picture, not above it: the picture is a large square with
    // room to spare, and a pill over its own subject cannot collide with the
    // title sitting directly above (which is what putting it above did).
    const r = pic.getBoundingClientRect();
    const w = p.offsetWidth, h = p.offsetHeight;
    let left = r.left + r.width / 2 - w / 2;
    let top = r.top + r.height / 2 - h / 2;
    p.style.left = Math.max(8, Math.min(left, innerWidth - w - 8)) + 'px';
    p.style.top = Math.max(8, Math.min(top, innerHeight - h - 8)) + 'px';
    // one frame at display:block before the fade, so the transition runs
    requestAnimationFrame(() => p.classList.add('show'));
  },

  hide() {
    const p = this.pill;
    this.target = null;
    if (!p) return;
    p.classList.remove('show');
    p.style.display = 'none';
  },

  remove() {
    const t = this.target;
    this.hide();
    if (!t || typeof feature_Loop === 'undefined' || !feature_Loop.state) return;
    feature_Loop.send({ type: 'CardPic',
      data: { id: t.id, i: t.i, data: '', t: Date.now() } });
  },

  disarm() {
    if (this.hold_t) clearTimeout(this.hold_t);
    this.hold_t = null;
    this.armed = null;
  },
};

{
  // take /loop's paint seam by replacing the property at load — /me's idiom on
  // feature_Account.openTool. NOT a timer-installed wrapper: those race each
  // other on this build (notes.md, "the apply-wrapper race").
  if (typeof feature_Loop !== 'undefined') {
    const fm_keepPaint = feature_Loop.paint;
    feature_Loop.paint = function (html) {
      const held = feature_Keep.hold();
      feature_Keep.painting = true;
      let back = true;
      try {
        fm_keepPaint.call(this, html);
        back = feature_Keep.restore(held);
      } catch (e) {
      } finally {
        feature_Keep.painting = false;
      }
      if (held && !back) feature_Keep.rescue(held);
    };
  }

  // the remove pill: furniture this node owns, made at load and living OUTSIDE
  // #app so a repaint cannot take it away — #cardToast's idiom.
  const fm_keepPill = document.createElement('div');
  fm_keepPill.id = 'cardRemove';
  fm_keepPill.textContent = 'remove';
  fm_keepPill.style.display = 'none';
  fm_keepPill.addEventListener('click', (e) => {
    e.stopPropagation();
    feature_Keep.remove();
  });
  document.body.appendChild(fm_keepPill);
  feature_Keep.pill = fm_keepPill;

  // a repaint that destroys the focused block is not a tap-away. Some engines
  // fire focusout when a focused element is removed; with the caret restored
  // afterwards that would make /cards save on every repaint, and its save is
  // itself a repaint — an endless round. Swallowed in the capture phase, so it
  // never reaches /cards' listener whatever order the fragments loaded in.
  document.addEventListener('focusout', (e) => {
    if (feature_Keep.painting) e.stopPropagation();
  }, true);

  // save as you type: a moment after the last keystroke, not only on tap-away
  document.addEventListener('input', (e) => {
    const el = e.target;
    if (!el || !el.getAttribute) return;
    if (el.getAttribute('contenteditable') !== 'true') return;
    if (el.getAttribute('data-block') === null) return;
    feature_Keep.soon(el);
  });
  document.addEventListener('focusout', (e) => {
    const el = e.target;
    if (!el || !el.getAttribute) return;
    if (el.getAttribute('data-block') === null) return;
    feature_Keep.cancel();
  });

  // a title is one line: Enter finishes it, and the blur is the save
  document.addEventListener('keydown', (e) => {
    if (e.key !== 'Enter' || e.shiftKey || e.isComposing) return;
    const el = e.target;
    if (!el || !el.classList || !el.classList.contains('card-title')) return;
    if (el.getAttribute('contenteditable') !== 'true') return;
    e.preventDefault();
    el.blur();
  });

  // hold a filled picture and it offers to remove itself; a plain tap is
  // /cards' chooser, untouched
  document.addEventListener('pointerdown', (e) => {
    if (!e.target || !e.target.closest) return;
    if (e.target.closest('#cardRemove')) return;
    feature_Keep.hide();
    const pic = e.target.closest('.card-pic');
    if (!pic || pic.classList.contains('empty')) return;
    feature_Keep.disarm();
    feature_Keep.fired = false;
    feature_Keep.armed = pic;
    feature_Keep.x = e.clientX;
    feature_Keep.y = e.clientY;
    feature_Keep.hold_t = setTimeout(() => feature_Keep.show(pic), feature_Keep.HOLD);
  });
  document.addEventListener('pointermove', (e) => {
    if (!feature_Keep.armed) return;
    if (Math.hypot(e.clientX - feature_Keep.x, e.clientY - feature_Keep.y) > feature_Keep.DRIFT)
      feature_Keep.disarm(); // a scroll is not a question
  });
  for (const fm_keepEnd of ['pointerup', 'pointercancel'])
    document.addEventListener(fm_keepEnd, () => feature_Keep.disarm());

  // a hold asks; it must not also open the photo chooser. Capture phase, so
  // this runs before /cards' own bubble-phase click listener whatever order
  // the fragments loaded in.
  document.addEventListener('click', (e) => {
    if (!feature_Keep.fired) return;
    if (!e.target || !e.target.closest || !e.target.closest('.card-pic')) return;
    e.stopPropagation();
    e.preventDefault();
    feature_Keep.fired = false;
  }, true);
}
