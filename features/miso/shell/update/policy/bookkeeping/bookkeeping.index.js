// bookkeeping commits (kind "docs") are counted builds but not release
// items: refuse them at /review's release-line seam
const feature_Bookkeeping = {
  init() {
    if (typeof feature_Review !== 'undefined') {
      feature_Review.releaseWorthy = (c) => c && c.kind !== 'docs';
    }
  },
};
feature_Bookkeeping.init();
