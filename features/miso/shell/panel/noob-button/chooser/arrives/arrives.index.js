// the panel opens on the tap; what fills it arrives after. /panel's open
// awaited the chooser's mount, and the mount awaited features/tree.json —
// so one stuck fetch was a nøøb button that "doesn't press" (#p95). It
// sticks for real: under a freshly installed service worker with new
// content, a `cache: 'no-store'` re-fetch of a URL the worker has just
// stored never returns (the gate saw it five times on 2026-08-26; the same
// URL with default cache mode answers in 15 ms). Three rules, then:
const feature_Arrives = { late: false, budget: 2500 };
{
  // 1. a tree.json fetch never carries no-store — the worker is net-first
  //    with /deadline, so freshness is its job — and never waits past the
  //    budget: a late list is an empty answer, and is marked late
  const fm_arrivesFetch = window.fetch;
  window.fetch = function (url, opts) {
    const u = typeof url === 'string' ? url : (url && url.url) || '';
    if (!/features\/tree\.json(\?|$)/.test(u)) return fm_arrivesFetch.apply(this, arguments);
    const plain = Object.assign({}, opts || {}); delete plain.cache;
    return Promise.race([
      fm_arrivesFetch.call(this, u, plain),
      new Promise((_, rej) => setTimeout(() => { feature_Arrives.late = true; rej(new Error('tree.json: late')); }, feature_Arrives.budget)),
    ]);
  };

  if (typeof feature_Chooser !== 'undefined') {
    // 2. a late list is not the list: say so, and let the next open try again
    const fm_arrivesMount = feature_Chooser.mount.bind(feature_Chooser);
    feature_Chooser.mount = async function () {
      feature_Arrives.late = false;
      await fm_arrivesMount();
      if (feature_Arrives.late) {
        this.flat = null; this.byPath = null;
        const box = $('changes');
        if (box) box.innerHTML = '<div class="crow">the feature list is still arriving — open again in a moment</div>';
      }
    };
  }

  if (typeof feature_Panel !== 'undefined') {
    // 3. the sheet is on screen before anything is awaited — and an open
    //    that was closed while it was still filling closes itself again when
    //    it completes, because /panel's own open shows the sheet last (the
    //    gate's warm pass found the shade back over the page, 2026-08-26)
    let fm_arrivesSeq = 0;
    const fm_arrivesClose = feature_Panel.close.bind(feature_Panel);
    feature_Panel.close = function () { fm_arrivesSeq++; fm_arrivesClose(); };
    const fm_arrivesOpen = feature_Panel.open.bind(feature_Panel);
    feature_Panel.open = async function () {
      const mine = ++fm_arrivesSeq;
      $('shade').style.display = 'block';
      $('panel').style.display = 'block';
      await fm_arrivesOpen();
      if (fm_arrivesSeq !== mine) fm_arrivesClose();
    };
  }
}
