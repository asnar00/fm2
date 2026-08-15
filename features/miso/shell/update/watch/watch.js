const feature_Watch = {
  // mid-session detection: a newer build lights the panel handle
  async check() {
    if (typeof feature_Update === 'undefined') return null;
    const v = await feature_Update.fetchVersion();
    if (!v) return null;
    feature_Update.server = v;
    if (feature_Update.newer()) {
      const handle = $('build');
      if (handle) handle.classList.add('update');
    }
    return v;
  },
};
document.addEventListener('visibilitychange', () => {
  if (document.visibilityState === 'visible') feature_Watch.check();
});
window.addEventListener('online', () => feature_Watch.check());
setInterval(() => {
  if (document.visibilityState === 'visible') feature_Watch.check();
}, 60000);
