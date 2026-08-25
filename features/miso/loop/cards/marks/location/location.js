const feature_Location = {
  // one position per card per page load. A repaint is not a new card, and
  // the observer below fires on every one of them.
  asked: {},
  sheet: null,
  where: null,
  meta: null,

  // "just now" / "3 min ago" / "2 hours ago" — plain words, no clock
  since(t) {
    const ms = Date.now() - t;
    if (!t || !isFinite(ms) || ms < 0) return 'just now';
    const m = Math.floor(ms / 60000);
    if (m < 1) return 'just now';
    if (m < 60) return m + ' min ago';
    const h = Math.floor(m / 60);
    if (h < 24) return h + (h === 1 ? ' hour ago' : ' hours ago');
    const d = Math.floor(h / 24);
    return d + (d === 1 ? ' day ago' : ' days ago');
  },

  // the single geolocation call. Denied, unavailable, timed out or absent:
  // nothing is stored and nothing is said — the pill stays dim, and a tap
  // (`again`) is the one thing that gets a second prompt.
  ask(id, again) {
    if (!id) return;
    if (this.asked[id] && !again) return;
    this.asked[id] = true;
    if (typeof navigator === 'undefined') return;
    const geo = navigator.geolocation;
    if (!geo || typeof geo.getCurrentPosition !== 'function') return;
    try {
      geo.getCurrentPosition((p) => {
        const c = (p && p.coords) || {};
        if (typeof c.latitude !== 'number' || typeof c.longitude !== 'number') return;
        if (typeof feature_Loop === 'undefined' || !feature_Loop.state) return;
        feature_Loop.send({ type: 'CardPlace', data: {
          id,
          lat: c.latitude,
          lon: c.longitude,
          acc: typeof c.accuracy === 'number' ? c.accuracy : 0,
          t: Date.now() } });
      }, () => {}, { enableHighAccuracy: false, timeout: 10000, maximumAge: 300000 });
    } catch (e) {
      /* an API that throws is an API that is not there */
    }
  },

  // the card page appearing is what this node reacts to, read from the DOM
  // rather than by wrapping feature_Loop.apply — that idiom races and orphans
  // other fragments' wrappers (notes.md, "the apply-wrapper race").
  look() {
    const pill = document.querySelector('.card-page .card-place');
    if (!pill) return;
    if (!pill.classList.contains('dim')) return;
    this.ask(pill.getAttribute('data-card'), false);
  },

  // the placeholder view: everything the block knows, straight off the pill's
  // data attributes, so the sheet never reads the store a second time
  show(pill) {
    if (!this.sheet || !pill) return;
    const lat = Number(pill.getAttribute('data-lat'));
    const lon = Number(pill.getAttribute('data-lon'));
    if (!isFinite(lat) || !isFinite(lon)) return;
    const acc = Math.round(Number(pill.getAttribute('data-acc')) || 0);
    const t = Number(pill.getAttribute('data-t')) || 0;
    this.where.textContent = lat.toFixed(4) + ', ' + lon.toFixed(4);
    this.meta.textContent = (acc > 0 ? '±' + acc + 'm · ' : '') + this.since(t);
    this.sheet.classList.add('show');
  },

  hide() {
    if (this.sheet) this.sheet.classList.remove('show');
  },
};

{
  // furniture made at load and living OUTSIDE #app, so a repaint of the
  // loop's html while the sheet is open cannot take it away — the #cardToast
  // and #frameSheet precedent.
  const fm_placeSheet = document.createElement('div');
  fm_placeSheet.id = 'placeSheet';

  const fm_placeBox = document.createElement('div');
  fm_placeBox.id = 'placeBox';
  const fm_placeWhere = document.createElement('div');
  fm_placeWhere.id = 'placeWhere';
  const fm_placeMeta = document.createElement('div');
  fm_placeMeta.id = 'placeMeta';
  const fm_placeClose = document.createElement('button');
  fm_placeClose.id = 'placeClose';
  fm_placeClose.type = 'button';
  fm_placeClose.textContent = 'close';
  fm_placeBox.appendChild(fm_placeWhere);
  fm_placeBox.appendChild(fm_placeMeta);
  fm_placeBox.appendChild(fm_placeClose);
  fm_placeSheet.appendChild(fm_placeBox);
  document.body.appendChild(fm_placeSheet);

  feature_Location.sheet = fm_placeSheet;
  feature_Location.where = fm_placeWhere;
  feature_Location.meta = fm_placeMeta;

  fm_placeClose.addEventListener('click', (e) => {
    e.preventDefault();
    feature_Location.hide();
  });
  // the ground closes it too; the box itself does not
  fm_placeSheet.addEventListener('click', (e) => {
    if (e.target === fm_placeSheet) feature_Location.hide();
  });

  // the pill carries no data-ev, so the loop's own delegated click never
  // fires for it. A dim pill asks again; a lit one opens the place.
  document.addEventListener('click', (e) => {
    if (!e.target || !e.target.closest) return;
    const pill = e.target.closest('.card-place');
    if (!pill) return;
    if (pill.classList.contains('dim')) {
      feature_Location.ask(pill.getAttribute('data-card'), true);
      return;
    }
    feature_Location.show(pill);
  });

  const fm_placeWatch = setInterval(() => {
    const app = document.getElementById('app');
    if (!app) return;
    clearInterval(fm_placeWatch);
    const look = () => feature_Location.look();
    new MutationObserver(look).observe(app, { childList: true, subtree: true });
    look();
  }, 100);
}
