const feature_SquarePosts = {
  // the pin markup, with the row's kind written onto it. /map draws every pin
  // the same way and the CSS tells them apart, so the only thing the page half
  // adds is the word: `data-kind="post"` on the outer `.map-pin`.
  mark(html, p) {
    const kind = (p && typeof p.kind === 'string') ? p.kind : '';
    if (!kind) return html;
    // exactly /map's opening tag, and nothing else. /live writes its own pin
    // (`<div class="map-pin map-live"…`), which fails this test and is left
    // alone: a live pin is a person, whatever card the row came from.
    const open = '<div class="map-pin">';
    if (html.indexOf(open) !== 0) return html;
    return '<div class="map-pin" data-kind="' + this.esc(kind) + '">'
      + html.slice(open.length);
  },

  esc(s) {
    return String(s).replace(/&/g, '&amp;').replace(/</g, '&lt;')
      .replace(/>/g, '&gt;').replace(/"/g, '&quot;');
  },
};

{
  // /map's pin-drawing seam, taken by replacing the property at load — the
  // idiom /boundaries and /one-pin use, never a timer-installed wrapper. The
  // markup is /map's; this only writes a word on it, and any surprise leaves
  // the pin as /map drew it.
  if (typeof feature_Map !== 'undefined') {
    const fm_squarePin = feature_Map.pinHtml;
    feature_Map.pinHtml = function (p) {
      const html = fm_squarePin.call(this, p);
      try {
        return feature_SquarePosts.mark(html, p);
      } catch (e) {
        return html;
      }
    };
  }
}
