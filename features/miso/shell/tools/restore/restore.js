const feature_Restore = {
  // remember every change to the open tool, launcher included
  watch() {
    let s = {};
    try { s = JSON.parse(feature_Loop.state || '{}'); } catch (e) {}
    if (typeof s.open_tool !== 'string') return; // toolbar off: nothing to remember
    if (localStorage.miso_open_tool !== s.open_tool) {
      localStorage.miso_open_tool = s.open_tool;
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
