const feature_Restore = {
  // which tool is open, read from the ROW that was just painted rather than
  // from `state.open_tool`. The var is bridged and /payload republishes it
  // part-way down the update chain, while /people, /posts and /projects each
  // write it back at a later link (the tap that means "back to the set" closes
  // the tool and re-opens it) — so on exactly that tap the mirror says "" and
  // this would remember the launcher for a tool that is plainly open, and
  // reopen at the launcher next time. `watch` runs after the paint, so the row
  // on screen is this turn's answer (/reel's rule, housekeeping #p19).
  // No toolbar yet — boot, or the toolbar unticked — falls back to the mirror,
  // which is what this read always was.
  openTool() {
    try {
      const sel = document.querySelector('.toolbar .tool-button.sel[data-ev^="tool_"]');
      if (sel) return sel.getAttribute('data-ev').slice(5);
      if (document.querySelector('.toolbar .tool-button[data-ev^="tool_"]')) return '';
    } catch (e) { /* fall through to the mirror */ }
    let s = {};
    try { s = JSON.parse(feature_Loop.state || '{}'); } catch (e) {}
    return typeof s.open_tool === 'string' ? s.open_tool : null;
  },

  // remember every change to the open tool, launcher included
  watch() {
    const open = this.openTool();
    if (typeof open !== 'string') return;   // toolbar off: nothing to remember
    if (localStorage.miso_open_tool !== open) {
      localStorage.miso_open_tool = open;
    }
  },
  init() {
    const fm_restApply = feature_Loop.apply;
    const self = this;
    feature_Loop.apply = function (p) {
      fm_restApply.call(this, p);
      self.watch();
    };
    // reopen where the user left off — through the normal event path, and
    // only if the remembered tool's button is actually in this composition
    const id = localStorage.miso_open_tool || '';
    let s = {};
    try { s = JSON.parse(feature_Loop.state || '{}'); } catch (e) {}
    if (id && s.open_tool === ''
        && document.querySelector('[data-ev="tool_' + id + '"]')) {
      feature_Loop.send({ type: 'click', ev: 'tool_' + id });
    }
    this.watch();
  },
};
const fm_restInit = setInterval(() => {
  if (typeof feature_Loop !== 'undefined' && feature_Loop.state !== null) {
    clearInterval(fm_restInit);
    feature_Restore.init();
  }
}, 100);
