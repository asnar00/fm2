const feature_Install = {
  // phone browsers are sent to install the PWA first — no login, no app.
  // (?browser=1 is an undocumented dev bypass, session-scoped.)
  redirect() {
    if (location.search.includes('browser=1')) sessionStorage.misoBrowser = '1';
    const havePwa = typeof feature_Standalone !== 'undefined';
    if (havePwa && feature_Standalone.phone() && !feature_Standalone.standalone()
        && !sessionStorage.misoBrowser) {
      location.replace('install.html');
      return true;
    }
    return false;
  },
};
