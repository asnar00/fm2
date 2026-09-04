// the map stays behind an open card on every tool that draws one — posts,
// people, projects — and it no longer asks a picker that is not there.
const feature_OnEveryTool = {
  was: '',        // the tool the map was last drawn for

  tool() {
    const b = document.querySelector('.toolbar .tool-button.sel[data-ev^="tool_"]');
    return b ? (b.getAttribute('data-ev') || '') : '';
  },

  // remembered from the screen rather than named in a list: whichever tool had
  // a map a moment ago is the tool whose card should keep it. A tool that
  // never draws one is never remembered, so its card page never claims the
  // map, and a tool added later needs nothing from this node.
  note() {
    if (document.getElementById('mapData')) this.was = this.tool();
  },

  // the parent's question, asked without the picker: Rust draws the page
  // instead of the set, so no #mapData with a card on screen is what "a card
  // is open" looks like — and the surface underneath was a map if the map was
  // last drawn for the tool that is still selected.
  behind() {
    if (document.getElementById('mapData')) return false;
    if (!document.querySelector('.card-page')) return false;
    const now = this.tool();
    return !!now && now === this.was;
  },
};

{
  if (typeof feature_OpensOverMap !== 'undefined' && typeof feature_Map !== 'undefined') {
    feature_OpensOverMap.behind = function () { return feature_OnEveryTool.behind(); };

    // after /opens-over-map's own wrapper, so the tool is noted on the syncs
    // where the set IS on the page — which are the syncs before a card opens.
    const fm_everySync = feature_Map.sync;
    feature_Map.sync = function () {
      fm_everySync.call(this);
      try { feature_OnEveryTool.note(); } catch (e) { /* the last tool stands */ }
    };
    try { feature_OnEveryTool.note(); } catch (e) { /* nothing drawn yet */ }
  }
}
