{
  if (typeof feature_Reel !== 'undefined') {
    // the leftmost lozenge fully in view, with a few pixels of grace for snapping
    feature_Reel.current = function () {
      const left = this.list.scrollLeft - 4;
      const all = this.list.querySelectorAll('.reel-post');
      for (const el of all) if (el.offsetLeft >= left) return el;
      return all.length ? all[all.length - 1] : null;
    };
    // the pin carries its card's id, so the match is exact — two posts at one
    // place mark one pin, not both
    if (typeof feature_Map !== 'undefined') {
      const fm_pinHtml = feature_Map.pinHtml;
      feature_Map.pinHtml = function (p) {
        const html = fm_pinHtml.call(this, p);
        const open = '<div class="map-pin"';
        if (!p || !p.id || html.indexOf(open) !== 0) return html;
        return open + ' data-id="' + this.esc(p.id) + '"' + html.slice(open.length);
      };
    }
    const fm_pinMark = feature_Reel.mark;
    feature_Reel.mark = function () {
      fm_pinMark.call(this);
      if (typeof feature_Map === 'undefined' || !feature_Map.map) return;
      const cur = this.list.querySelector('.reel-current');
      const ev = cur ? (cur.getAttribute('data-ev') || '') : '';
      const id = ev.indexOf('browse_open:') === 0 ? ev.slice('browse_open:'.length) : '';
      for (const pin of document.querySelectorAll('#misoMap .map-pin[data-id]'))
        pin.classList.toggle('reel-focus', !!id && pin.getAttribute('data-id') === id);
    };
  }
}
