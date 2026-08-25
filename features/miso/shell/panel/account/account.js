const feature_Account = {
  wasOpen: false,
  // the open seam: WHAT the account tool shows is a decision, and this pair is
  // where it is made. The default is the one this node was born with — the
  // system panel sheet — so a composition with nothing else in it behaves
  // exactly as before. A feature that gives 👤 a surface of its own replaces
  // the pair (see /me).
  openTool() {
    if (typeof feature_Panel !== 'undefined') feature_Panel.open();
  },
  closeTool() {
    if (typeof feature_Panel !== 'undefined') feature_Panel.close();
  },
  // the page half: the account tool's open state drives whatever the seam says
  watch() {
    let s = {};
    try { s = JSON.parse(feature_Loop.state || '{}'); } catch (e) {}
    const isOpen = (s.open_tool === 'account');
    if (isOpen && !this.wasOpen) this.openTool();
    if (!isOpen && this.wasOpen) this.closeTool();
    this.wasOpen = isOpen;
  },
  init() {
    const fm_acctApply = feature_Loop.apply;
    const self = this;
    feature_Loop.apply = function (p) {
      fm_acctApply.call(this, p);
      self.watch();
    };
    if (typeof feature_Panel !== 'undefined') {
      // a shade-tap dismissal must also leave the tool, so toolbar state
      // never lies; only fires while the tool is open, so no loop
      const fm_acctClose = feature_Panel.close.bind(feature_Panel);
      feature_Panel.close = function () {
        fm_acctClose();
        let s = {};
        try { s = JSON.parse(feature_Loop.state || '{}'); } catch (e) {}
        if (s.open_tool === 'account') feature_Loop.send({ type: 'click', ev: 'tools_home' });
      };
      // the corner logo button's tap is parked for the future agent interface
      feature_Panel.buttonTap = () => {};
    }
  },
};
const fm_acctInit = setInterval(() => {
  if (typeof feature_Loop !== 'undefined' && feature_Loop.state !== null) {
    clearInterval(fm_acctInit);
    feature_Account.init();
  }
}, 100);
