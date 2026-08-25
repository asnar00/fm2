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
// decline /account's parking of the corner button at LOAD, before any init
// timer fires: the lozenge's tap is this node's, whatever order the timers
// take (with /world-cache the order changed and the button went dead)
if (typeof feature_Account !== 'undefined') {
  feature_Account.parkTap = function () {};
}
const fm_noobInit = setInterval(() => {
  if (typeof feature_Loop !== 'undefined' && feature_Loop.state !== null) {
    clearInterval(fm_noobInit);
    feature_NoobButton.init();
  }
}, 100);
