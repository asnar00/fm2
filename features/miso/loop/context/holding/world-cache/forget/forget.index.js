// signing out forgets the device's copy of the world: the cache is one
// person's, and the next person to sign in on this device must not open
// on it. Takes the panel's logout handler at load and wipes before it runs.
{
  const fm_forgetBtn = document.getElementById('logoutBtn');
  if (fm_forgetBtn && typeof feature_WorldCache !== 'undefined') {
    const fm_forgetLogout = fm_forgetBtn.onclick;
    fm_forgetBtn.onclick = async function (e) {
      try { await feature_WorldCache.wipe(); } catch (err) {}
      if (fm_forgetLogout) return fm_forgetLogout.call(this, e);
    };
  }
}
