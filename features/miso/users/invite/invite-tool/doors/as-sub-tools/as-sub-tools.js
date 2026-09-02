// the two control buttons open /doors' sheet in one of its two faces. The
// sheet, the two roads and everything they send are /doors' unchanged; what
// this node moves is where the choice is made — the control row, not a page
// (ash, 2026-09-02: "the page with the two buttons is doing the job of the
// toolbar").
const feature_AsSubTools = {
  // the row's events, and the face each opens
  FACES: { invite_qr: 'qr', invite_name: 'name' },

  face(ev) {
    return (ev && this.FACES[ev]) || '';
  },
};

{
  // where the person is going. /doors read the selected project off the
  // `.doors` block; that block is no longer drawn, so it reads this node's
  // holder instead — a whole redefinition, not a wrapper, because the old
  // answer is not a fallback but a stale empty object. Untick this node and
  // /doors' own reader and its own block both come back.
  if (typeof feature_Doors !== 'undefined') {
    feature_Doors.project = function () {
      const el = document.querySelector('.invite-into');
      return {
        id: (el && el.getAttribute('data-project')) || '',
        title: (el && el.getAttribute('data-project-title')) || '',
      };
    };
  }

  // CAPTURE, and the tap stops here: /loop's delegated listener would
  // otherwise send `invite_qr` through the Rust chain, where nothing answers
  // it, and /backdrop reads any tap it sees on nobody's ground. Registered
  // last of the capture listeners (this node is the newest), so
  // /sub-tool-cards' long-press suppression has already run — it calls
  // preventDefault on the click that ends a hold, which is the mark read
  // here: a long press reads the button and must not also open the sheet.
  document.addEventListener('click', (e) => {
    if (e.defaultPrevented) return;
    if (!e.target || !e.target.closest) return;
    const btn = e.target.closest('.tool-button.ctrl[data-ev]');
    if (!btn) return;
    const face = feature_AsSubTools.face(btn.getAttribute('data-ev'));
    if (!face) return;
    e.stopPropagation();
    e.preventDefault();
    if (typeof feature_Doors !== 'undefined') feature_Doors.open(face);
  }, true);

  // with /qr unticked there is no code to show, so there is no way in by one:
  // /doors hid its own button the same way, for the same reason
  if (typeof feature_Qr === 'undefined') {
    const fm_subToolsNoQr = document.createElement('style');
    fm_subToolsNoQr.textContent = '.tool-button[data-ev="invite_qr"] { display: none; }';
    document.head.appendChild(fm_subToolsNoQr);
  }
}
