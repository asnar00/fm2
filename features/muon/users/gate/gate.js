const feature_Gate = {
  // shell is public; data is not. the shell decides to show login by asking.
  whoami: () => fetch('auth/whoami', { cache: 'no-store' })
    .then((r) => r.json()).catch(() => null),
  redirectIfLoggedOut(who) {
    if (who && who.ok && !who.authed) {
      location.replace('login.html');
      return true;
    }
    return false;
  },
};
