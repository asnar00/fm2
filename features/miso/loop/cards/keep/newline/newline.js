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
