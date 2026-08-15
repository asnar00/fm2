const feature_NoobButton = {
  init() {
    // the meta-button opens the meta-surface: the seam returns to its default
    if (typeof feature_Panel !== 'undefined') {
      feature_Panel.buttonTap = () => feature_Panel.open();
    }
    // and the 👤 tool stops driving the panel: empty surface, profile later
    if (typeof feature_Account !== 'undefined') {
      feature_Account.watch = () => {};
    }
  },
};
const fm_noobInit = setInterval(() => {
  if (typeof feature_Loop !== 'undefined' && feature_Loop.state !== null) {
    clearInterval(fm_noobInit);
    feature_NoobButton.init();
  }
}, 100);
