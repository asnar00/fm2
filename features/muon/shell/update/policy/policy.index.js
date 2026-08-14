const feature_Policy = {
  // launch runs before join delivers state, so decisions made at launch read
  // the localStorage mirror of the var; the mirror refreshes on every apply.
  current() {
    try {
      const s = JSON.parse(feature_Loop.state || '{}');
      if (s.update_policy) return s.update_policy;
    } catch (e) {}
    return localStorage.muonPolicy || 'auto';
  },
  reflect() {
    const p = this.current();
    for (const b of document.querySelectorAll('#policySeg [data-ev]'))
      b.classList.toggle('sel', b.getAttribute('data-ev') === 'policy_' + p);
  },
  // the policy question: may this update apply without being asked?
  async consentNeeded() {
    const p = this.current();
    if (p === 'auto') return false;
    if (p === 'consent') return true;
    // 'fixes': pending changes must all be fixes, and the list must actually
    // cover the gap — unknown pending builds count as needing consent
    const up = typeof feature_Update !== 'undefined' ? feature_Update : null;
    const running = up ? parseInt(up.running, 10) : NaN;
    if (!running) return true;
    const changes = await fetch('changes.json', { cache: 'no-store' })
      .then((r) => r.ok ? r.json() : []).catch(() => []);
    const pending = changes.filter((c) => c.build > running);
    if (!pending.length) return true;
    if (Math.min(...pending.map((c) => c.build)) > running + 1) return true;
    return pending.some((c) => c.kind === 'feature');
  },
};
{
  const fm_row = document.createElement('div');
  fm_row.className = 'row';
  fm_row.id = 'policySeg';
  fm_row.innerHTML = 'updates: '
    + '<button data-ev="policy_auto">automatic</button>'
    + '<button data-ev="policy_fixes">fixes auto</button>'
    + '<button data-ev="policy_consent">ask me</button>';
  $('panel').insertBefore(fm_row, $('logoutBtn').closest('.row'));

  // mirror + reflect on every state change (join arrival, picker clicks,
  // broadcasts from the user's other devices)
  const fm_policyApply = feature_Loop.apply;
  feature_Loop.apply = function (p) {
    fm_policyApply.call(this, p);
    try {
      const s = JSON.parse(feature_Loop.state || '{}');
      if (s.update_policy) localStorage.muonPolicy = s.update_policy;
    } catch (e) {}
    feature_Policy.reflect();
  };
  feature_Policy.reflect();

  // enforcement: replace update's consent hook, wrap auto's act
  if (typeof feature_Update !== 'undefined')
    feature_Update.consented = async () => !(await feature_Policy.consentNeeded());
  if (typeof feature_Auto !== 'undefined') {
    const fm_policyAct = feature_Auto.act;
    feature_Auto.act = async function () {
      if (await feature_Policy.consentNeeded()) return; // leave the pulse asking
      fm_policyAct.call(this);
    };
  }
}
