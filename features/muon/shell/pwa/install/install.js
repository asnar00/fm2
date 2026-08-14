const feature_Install = {
  // phone browsers are sent to install the PWA first — no login, no app.
  // (?browser=1 is an undocumented dev bypass, session-scoped.)
  redirect() {
    if (location.search.includes('browser=1')) sessionStorage.muonBrowser = '1';
    const havePwa = typeof feature_Pwa !== 'undefined';
    if (havePwa && feature_Pwa.phone() && !feature_Pwa.standalone()
        && !sessionStorage.muonBrowser) {
      location.replace('install.html');
      return true;
    }
    return false;
  },
};
