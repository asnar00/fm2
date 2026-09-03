// the tick tapped with no line written: the box says so, twice, quickly
const feature_MissionFlash = {
  flash() {
    const box = document.querySelector('.card-page .card-text');
    if (!box) return false;
    if ((box.innerText || '').trim()) return false;
    box.classList.remove('mission-flash');
    void box.offsetWidth;
    box.classList.add('mission-flash');
    setTimeout(() => box.classList.remove('mission-flash'), 800);
    // after /toolbar's own blur-and-lock and /profile-first's reopen
    setTimeout(() => { try { box.focus(); } catch (e) {} }, 250);
    return true;
  },
};
document.addEventListener('pointerdown', (e) => {
  const hit = e.target && e.target.closest ? e.target.closest('[data-ctl="card_edit"]') : null;
  if (!hit || hit.getAttribute('data-face') !== 'save') return;
  if (typeof feature_ProfileFirst === 'undefined' || !feature_ProfileFirst.gated()) return;
  feature_MissionFlash.flash();
}, true);
