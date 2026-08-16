// borrowing a DOM node is a loan: /chooser rebuilds #changes on every
// mount, which deletes the picker /tucked-updates moved inside it. Park it
// on the panel (hidden) across the rebuild, so the parent's own place()
// still finds it by id and re-homes it exactly as it always did. (#p19)
const feature_StaysPut = {
  // out of the doomed container, still in the document — a detached node
  // is invisible to getElementById, which is what the first attempt got
  // wrong: the parent then couldn't find it either
  rescue() {
    const seg = document.getElementById('policySeg');
    const panel = document.getElementById('panel');
    if (!seg || !panel) return;
    const parent = seg.parentElement;
    if (!parent || parent.id !== 'tucked') return;
    seg.dataset.parked = '1';
    seg.style.display = 'none';
    panel.appendChild(seg);
  },

  // the picker's own address in the panel, above the log-out row
  home(seg) {
    const panel = document.getElementById('panel');
    const logout = document.getElementById('logoutBtn');
    const row = logout && logout.closest ? logout.closest('.row') : null;
    if (!panel || !row) return;
    panel.insertBefore(seg, row);
  },

  // whatever the parent managed, the picker ends up somewhere real and visible
  settle() {
    const seg = document.getElementById('policySeg');
    if (!seg || !seg.dataset.parked) return;
    const tucked = document.getElementById('tucked');
    if (tucked) {
      if (seg.parentElement !== tucked) tucked.appendChild(seg);
    } else {
      this.home(seg);   // no chooser hosting it: back to its own layout
    }
    delete seg.dataset.parked;
    seg.style.display = '';
  },
};
{
  if (typeof feature_Chooser !== 'undefined' && typeof feature_TuckedUpdates !== 'undefined') {
    const fm_staysMount = feature_Chooser.mount.bind(feature_Chooser);
    feature_Chooser.mount = async function () {
      feature_StaysPut.rescue();
      await fm_staysMount();
    };
    const fm_staysPlace = feature_TuckedUpdates.place.bind(feature_TuckedUpdates);
    feature_TuckedUpdates.place = function () {
      fm_staysPlace();
      feature_StaysPut.settle();
      feature_TuckedUpdates.show();
    };
  }
}
