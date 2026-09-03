{
  if (typeof feature_Viewer !== 'undefined') {
    const fm_glyphMake = feature_Viewer.make;
    feature_Viewer.make = function () {
      fm_glyphMake.call(this);
      const b = this.sheet && this.sheet.querySelector('.repview-share');
      if (!b || b.querySelector('svg')) return;
      b.setAttribute('aria-label', 'share the PDF');
      b.setAttribute('title', 'share the PDF');
      b.classList.add('repview-glyph');
      b.innerHTML = '<svg viewBox="0 0 24 24" width="22" height="22" fill="none" stroke="currentColor" '
        + 'stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">'
        + '<path d="M12 15V3"/><path d="M8 7l4-4 4 4"/>'
        + '<path d="M5 11v8a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2v-8"/></svg>';
    };
  }
}
