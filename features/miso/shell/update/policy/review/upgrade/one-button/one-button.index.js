const feature_OneButton = {
  // the redraw queue: section() calls run one at a time, in arrival order.
  chain: Promise.resolve(),

  sweep() {
    const all = document.querySelectorAll('#awaiting');
    for (let i = 1; i < all.length; i++) all[i].remove();
  },
};

{
  // property replacement at load, the house idiom; with /review unticked
  // there is nothing to serialise.
  if (typeof feature_Review !== 'undefined' && feature_Review.section) {
    const fm_obSection = feature_Review.section.bind(feature_Review);
    feature_Review.section = function () {
      const run = feature_OneButton.chain.then(() => fm_obSection());
      // the chain sweeps after every run, success or failure, and never
      // wedges; the caller still sees a failure as its own
      feature_OneButton.chain = run.then(
        () => feature_OneButton.sweep(),
        () => feature_OneButton.sweep());
      return run;
    };
  }
}
