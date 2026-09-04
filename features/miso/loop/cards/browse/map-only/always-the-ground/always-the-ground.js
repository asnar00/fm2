// the map is the ground everywhere inside a browse tool — its set, its card
// pages, and every level and page nested under it: the recording row, the
// level list, the region page, the invite page. The launcher keeps its dots.
//
// /on-every-tool already remembers which tool the map was last drawn for and
// keeps it behind an open CARD. This widens the same memory to everything else
// inside that tool rather than keeping a second one.
const feature_AlwaysTheGround = {
  // ‹ is drawn only while a tool is open, so its presence is "not the
  // launcher" — the one boundary in ash's ruling, and the only one.
  insideATool() {
    return !!document.querySelector('.toolbar [data-ev="tools_home"]');
  },

  selected() {
    const b = document.querySelector('.toolbar .tool-button.sel[data-ev^="tool_"]');
    return b ? (b.getAttribute('data-ev') || '') : '';
  },

  // /one-level's test, asked on the page half: a tool the registry names is a
  // level of its own, one it does not name is nested under whatever opened it.
  // `tools_catalog` is bridged, so the registry is on the state already.
  registry() {
    try {
      const s = JSON.parse(feature_Loop.state || '{}');
      return JSON.parse(s.tools_catalog || '[]').map((t) => 'tool_' + t.id);
    } catch (e) {
      return [];
    }
  },

  // the memory is /on-every-tool's, extended and not copied. It never clears
  // its own, and does not need to: its test is that the selected tool IS the
  // one the map was drawn for, which a stale value cannot satisfy. A wider
  // test can be fooled by one, so leaving to the launcher clears it here —
  // which can never change /on-every-tool's answer, because it is only ever
  // cleared on a screen where that answer was already no.
  drawnFor() {
    return (typeof feature_OnEveryTool !== 'undefined' && feature_OnEveryTool.was) || '';
  },
  forget() {
    if (typeof feature_OnEveryTool !== 'undefined') feature_OnEveryTool.was = '';
  },
  note(ev) {
    if (typeof feature_OnEveryTool !== 'undefined') feature_OnEveryTool.was = ev;
  },

  // the filter slot is drawn by exactly the surfaces that browse a set — it is
  // /map-only's own seam, filled by /since — so its presence says "this tool
  // draws a map" without naming posts, people or projects, and says it on the
  // first frame of a tool whose set has not been drawn yet. That is the deep
  // link: a relaunch straight into a remembered card, where nothing has been
  // noted because #mapData never appeared.
  browsing() {
    return !!document.querySelector('.since-pills');
  },

  // the reason a level would refuse what it inherited. Ash's rule is that a
  // sub-tool keeps its parent's ground "unless it has a reason to" change it,
  // so the exception is an /extension point/ rather than a list: nothing has a
  // reason today, and a node that grows one redefines this and says why in its
  // own spec. Without it the rule would only be true of the levels that
  // existed when it was written.
  ownGround() {
    return false;
  },

  ground() {
    // the set is on the page: /map shows the map itself
    if (document.getElementById('mapData')) return false;
    // a level that has stated a reason to draw its own ground
    if (this.ownGround()) return false;
    // the launcher: the dots, and the memory goes with them
    if (!this.insideATool()) { this.forget(); return false; }
    const now = this.selected();
    let was = this.drawnFor();
    if (!was && this.browsing() && now) { this.note(now); was = now; }
    if (!was) return false;
    if (!now) return true;                              // a level with no tool of its own
    if (now === was) return true;                       // the tool itself, or a page of it
    if (this.registry().indexOf(now) < 0) return true;  // a tool nested under it
    this.forget();                                      // another top-level tool
    return false;
  },
};

{
  if (typeof feature_Map !== 'undefined') {
    // last of the three wrappers on sync, so /map has hidden the host, then
    // /opens-over-map has had its say about a card, and what is left is every
    // other surface inside the tool.
    const fm_atgSync = feature_Map.sync;
    feature_Map.sync = function () {
      fm_atgSync.call(this);
      try {
        // a card page is the parent's: it also arms the tap that puts the card
        // away, which no other surface here wants.
        if (document.body.classList.contains('fm-map-behind')) return;
        if (!feature_AlwaysTheGround.ground()) return;
        if (!this.map) this.mount();   // the deep link: no map made yet
        this.show();
      } catch (e) { /* the ground is as /map left it */ }
    };
  }
}
