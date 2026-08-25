// Enter at the end of a paragraph makes a line that the next save must not
// eat: trim spaces, not newlines. A contenteditable reports a fresh empty
// last line as a double newline; one is the break the user made, so keep
// exactly one trailing newline at most.
if (typeof feature_Cards !== 'undefined') {
  feature_Cards.textOf = function (el) {
    let t = (el.innerText || '').replace(/^[ \t]+|[ \t]+$/g, '');
    t = t.replace(/\n{2,}$/, '\n');
    return t;
  };
}

// the caret across a line break (accounts #p79). /keep measured the caret as
// a text offset, and a break is not text — so after Enter the caret on the
// new line measured the same as the end of the line above, and the restore
// put it there. Save and restore now count the same way: a text node counts
// its characters, a <br> counts one, and a block counts one when anything
// precedes it (Chrome puts the FIRST new line in a <div> after bare text).
if (typeof feature_Keep !== 'undefined') {
  const fm_isBlock = (n) => n.nodeType === 1 && (n.tagName === 'DIV' || n.tagName === 'P');
  // chars before the caret, by the rule above, over the clone of what precedes it
  feature_Keep.caretOf = function (el) {
    const sel = document.getSelection && document.getSelection();
    if (!sel || !sel.rangeCount) return null;
    const r = sel.getRangeAt(0);
    if (!el.contains(r.startContainer)) return null;
    const pre = r.cloneRange();
    pre.selectNodeContents(el);
    pre.setEnd(r.startContainer, r.startOffset);
    const frag = pre.cloneContents();
    let count = 0;
    const walk = document.createTreeWalker(frag, NodeFilter.SHOW_TEXT | NodeFilter.SHOW_ELEMENT, null);
    let n;
    while ((n = walk.nextNode())) {
      if (n.nodeType === 3) count += n.nodeValue.length;
      else if (n.tagName === 'BR') count += 1;
      else if (fm_isBlock(n)) { if (count > 0) count += 1; }
    }
    return count;
  };
  feature_Keep.putCaret = function (el, off) {
    const sel = document.getSelection && document.getSelection();
    if (!sel) return;
    const r = document.createRange();
    let count = 0, placed = false;
    const walk = document.createTreeWalker(el, NodeFilter.SHOW_TEXT | NodeFilter.SHOW_ELEMENT, null);
    let n;
    while ((n = walk.nextNode())) {
      if (n.nodeType === 3) {
        const len = n.nodeValue.length;
        if (count + len >= off) { r.setStart(n, Math.max(0, off - count)); placed = true; break; }
        count += len;
      } else if (n.tagName === 'BR') {
        count += 1;
        if (count === off) { r.setStartAfter(n); placed = true; break; }
      } else if (fm_isBlock(n)) {
        if (count > 0) { count += 1; if (count === off) { r.setStart(n, 0); placed = true; break; } }
      }
    }
    if (!placed) { r.selectNodeContents(el); r.collapse(false); } else { r.collapse(true); }
    sel.removeAllRanges(); sel.addRange(r);
  };
}
