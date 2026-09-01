const feature_Qr = {
  busy: false,

  // the sheet's state is the mint's answer plus `open` — a transient loop-state
  // key, never a /var: a code belongs to the server
  send(d) {
    if (typeof feature_Loop === 'undefined' || !feature_Loop.state) return;
    feature_Loop.send({ type: 'QrSheet', data: d });
  },

  // open, and "new code", are the same act with one flag between them
  async mint(fresh) {
    if (this.busy) return;
    this.busy = true;
    this.send({ open: true });
    let d = null;
    try {
      const r = await fetch('users/invite/qr/mint', {
        method: 'POST', cache: 'no-store',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ fresh: !!fresh }),
      });
      d = await r.json();
    } catch (e) {
      d = null;
    }
    this.busy = false;
    if (!d || !d.ok) d = { error: (d && d.error) || "couldn't make a code" };
    d.open = true;
    this.send(d);
  },

  close() {
    this.send({ open: false });
    // the count on the invite list may have moved while the sheet was up
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
    const el = document.querySelector('.qr-code');
    if (!el) return;
    const token = el.getAttribute('data-qr-token') || '';
    if (!token) return;
    const url = location.origin + '/join?t=' + token;
    if (el.getAttribute('data-qr-drawn') === url) return;
    el.setAttribute('data-qr-drawn', url);
    if (typeof qrcode === 'undefined') {
      el.textContent = "couldn't draw the code";
      return;
    }
    try {
      el.innerHTML = this.svg(url);
    } catch (e) {
      el.textContent = "couldn't draw the code";
    }
  },
};

{
  // CAPTURE, and the sheet swallows its own taps. /backdrop closes the open
  // tool on any tap that lands on nobody's ground, and its list of owners is
  // its own file, not a seam — so the sheet claims its taps here instead:
  // without this, "done" closed the invite page as well and dropped the
  // canvasser back to the dot grid (found on the rig).
  document.addEventListener('click', (e) => {
    if (!e.target || !e.target.closest) return;
    if (e.target.closest('.qr-sheet')) e.stopPropagation();
    const hit = e.target.closest('[data-qr]');
    if (!hit) return;
    const what = hit.getAttribute('data-qr');
    if (what === 'open') feature_Qr.mint(false);
    if (what === 'new') feature_Qr.mint(true);
    if (what === 'done') feature_Qr.close();
  }, true);

  const fm_qrWatch = setInterval(() => {
    const app = document.getElementById('app');
    if (!app) return;
    clearInterval(fm_qrWatch);
    const look = () => feature_Qr.look();
    new MutationObserver(look).observe(app, { childList: true, subtree: true });
    look();
  }, 100);
}
