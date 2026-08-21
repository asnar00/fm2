// diary-class commits are counted builds but not release items: they are
// narrowed out at /review's release-line seam. The convention is the commit
// subject's declared prefix — the same one tools/export_features.py reads
// when it decides which release last touched a feature node.
const feature_Bookkeeping = {
  prefixes: ['notes:', 'handover:', 'idea:', 'ideas:', 'format:'],

  // how many entries the last narrowing dropped (the header count reads it)
  dropped: 0,

  // a commit declares itself diary-class in its subject; anything we cannot
  // read as text is NOT diary — an unreadable entry stays visible
  diary(c) {
    const t = c && typeof c.text === 'string' ? c.text.trimStart().toLowerCase() : '';
    return this.prefixes.some((p) => t.startsWith(p));
  },

  // an update never lists nothing: when the whole gap is diary and no feature
  // row stands in for it, one honest line replaces the entries we removed
  summary(server) {
    return [{ build: server, text: 'housekeeping — notes and records only' }];
  },
};
{
  if (typeof feature_Review !== 'undefined') {
    const fm_bkReleases = feature_Review.releases.bind(feature_Review);
    feature_Review.releases = function (changes, running, server, covered) {
      const all = fm_bkReleases(changes, running, server, covered);
      const real = all.filter((c) => !feature_Bookkeeping.diary(c));
      feature_Bookkeeping.dropped = all.length - real.length;
      if (all.length && !real.length && !covered.size)
        return feature_Bookkeeping.summary(server);
      return real;
    };

    // the header counts what it lists: the build gap minus what we dropped
    const fm_bkCount = feature_Review.count.bind(feature_Review);
    feature_Review.count = function (running, server) {
      return Math.max(0, fm_bkCount(running, server) - feature_Bookkeeping.dropped);
    };
  }
}
