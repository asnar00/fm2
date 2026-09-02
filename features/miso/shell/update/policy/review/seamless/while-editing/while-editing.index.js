const feature_WhileEditing = {
  // an own card open for writing (/editing's flag for the page on screen),
  // or a block still holding the caret — its words reach the store on the
  // tap away (/keep/manual)
  editing() {
    if (typeof feature_Editing !== 'undefined') {
      const page = feature_Editing.page();
      if (page && feature_Editing.open[feature_Editing.id(page)]) return true;
    }
    const el = document.activeElement;
    if (el && el.getAttribute && el.getAttribute('contenteditable') === 'true'
        && el.getAttribute('data-block') !== null) return true;
    return false;
  },

  // the edit has ended: a build that waited for it goes now (/seamless's own
  // retry, for the save that changes no state)
  retry() {
    if (typeof feature_Seamless === 'undefined' || typeof feature_Review === 'undefined') return;
    if (!feature_Seamless.deferred || feature_Seamless.busy()) return;
    const b = feature_Seamless.deferred;
    feature_Seamless.deferred = 0;
    feature_Review.apply(b);
  },
};
{
  if (typeof feature_Seamless !== 'undefined') {
    const fm_whileEditingBusy = feature_Seamless.busy.bind(feature_Seamless);
    feature_Seamless.busy = function () {
      return fm_whileEditingBusy() || feature_WhileEditing.editing();
    };
  }
  // the save (tick, save pill) ends in /editing's lock; the build follows it
  if (typeof feature_Editing !== 'undefined') {
    const fm_whileEditingLock = feature_Editing.lock.bind(feature_Editing);
    feature_Editing.lock = function () {
      fm_whileEditingLock();
      setTimeout(() => feature_WhileEditing.retry(), 0);
    };
  }
}
