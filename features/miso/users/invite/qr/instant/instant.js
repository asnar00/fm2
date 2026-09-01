const feature_Instant = {
  busy: false,
  // the name box lives out of the DOM: #app is repainted wholesale by the loop,
  // so a half-typed name would vanish under any other event
  draft: '',

  send(d) {
    if (typeof feature_Loop === 'undefined' || !feature_Loop.state) return;
    feature_Loop.send({ type: 'InstantSheet', data: d });
  },

  open() {
    this.draft = '';
    this.send({ open: true });
  },

  // one field, one button, and the account exists before the code is drawn
  async mint() {
    if (this.busy) return;
    const el = document.querySelector('.ins-name');
    const name = ((el && el.value) || this.draft || '').trim();
    if (!name) {
      this.send({ open: true, error: 'type a name first' });
      return;
    }
    this.busy = true;
    let d = null;
    try {
      const r = await fetch('users/invite/instant/mint', {
        method: 'POST', cache: 'no-store',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ name }),
      });
      d = await r.json();
    } catch (e) {
      d = null;
    }
    this.busy = false;
    if (!d || !d.ok) d = { error: (d && d.error) || "couldn't make a code" };
    else this.draft = '';
    d.open = true;
    this.send(d);
  },

  close() {
    this.draft = '';
    this.send({ open: false });
    // the list may have gained a name while the sheet was up
    if (typeof feature_Invite !== 'undefined' && feature_Invite.pull) feature_Invite.pull();
  },

  // the encoder gives a matrix; the SVG is ours, so the code takes its colours
  // and its quiet zone from this file rather than from a library's opinion
  svg(text) {
    const q = qrcode(0, 'M');
    q.addData(text);
    q.make();
    const n = q.getModuleCount();
    const pad = 4;
    let d = '';
    for (let row = 0; row < n; row++) {
      let run = 0;
      for (let col = 0; col <= n; col++) {
        const dark = col < n && q.isDark(row, col);
        if (dark) { run++; continue; }
        if (run) d += 'M' + (col - run) + ' ' + row + 'h' + run + 'v1h-' + run + 'z';
        run = 0;
      }
    }
    return '<svg xmlns="http://www.w3.org/2000/svg" viewBox="' + (-pad) + ' ' + (-pad)
      + ' ' + (n + pad * 2) + ' ' + (n + pad * 2) + '" shape-rendering="crispEdges">'
      + '<path fill="#0b0b0d" d="' + d + '"/></svg>';
  },

  // #app is repainted wholesale by the loop, so the code is redrawn from the
  // token by observation — never by wrapping feature_Loop.apply
  look() {
    const el = document.querySelector('.ins-code');
    if (!el) return;
    const token = el.getAttribute('data-ins-token') || '';
    if (!token) return;
    const url = location.origin + '/go?t=' + token;
    if (el.getAttribute('data-ins-drawn') === url) return;
    el.setAttribute('data-ins-drawn', url);
    // the encoder is /qr's vendored asset — absence is the unticked state
    if (typeof qrcode === 'undefined') {
      el.textContent = "couldn't draw the code";
      return;
    }
    try {
      el.innerHTML = this.svg(url);
    } catch (e) {
      el.textContent = "couldn't draw the code";
    }
    const box = document.querySelector('.ins-name');
    if (box && !box.value && feature_Instant.draft) box.value = feature_Instant.draft;
  },
};

{
  // the field carries no data-ev, so the loop's delegated click never fires for
  // it and typing never repaints the page out from under the caret
  document.addEventListener('input', (e) => {
    const el = e.target;
    if (!el || !el.classList) return;
    if (el.classList.contains('ins-name')) feature_Instant.draft = el.value;
  });

  // CAPTURE, and the sheet swallows its own taps: /backdrop closes the open tool
  // on any tap that lands on nobody's ground, which would drop the canvasser
  // back to the dot grid on "done" (/qr found this on the rig).
  document.addEventListener('click', (e) => {
    if (!e.target || !e.target.closest) return;
    if (e.target.closest('.ins-sheet')) e.stopPropagation();
    const hit = e.target.closest('[data-ins]');
    if (!hit) return;
    const what = hit.getAttribute('data-ins');
    if (what === 'open') feature_Instant.open();
    if (what === 'mint') feature_Instant.mint();
    if (what === 'done') feature_Instant.close();
  }, true);

  // enter in the name box mints, so the canvasser never has to reach for a pill
  document.addEventListener('keydown', (e) => {
    if (!e.target || !e.target.classList) return;
    if (e.key !== 'Enter' || !e.target.classList.contains('ins-name')) return;
    e.preventDefault();
    feature_Instant.mint();
  });

  const fm_insWatch = setInterval(() => {
    const app = document.getElementById('app');
    if (!app) return;
    clearInterval(fm_insWatch);
    const look = () => feature_Instant.look();
    new MutationObserver(look).observe(app, { childList: true, subtree: true });
    look();
  }, 100);
}
