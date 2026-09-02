// the invite page's one sheet, in two faces: `name` (name, phone, rank,
// invite) and `qr` (rank, show). Furniture made at load and living outside
// #app — /projects' add sheet is the precedent — so the loop's wholesale
// repaint of #app cannot take a half-typed invite away.
const feature_Doors = {
  sheet: null,
  box: null,
  name: null,
  phone: null,
  rank: null,
  line: null,
  say: null,
  go: null,
  face: '',
  busy: false,
  RANKS: ['admin', 'candidate', 'team', 'volunteer', 'supporter', 'public'],
  DEFAULT: 'team',

  ranks() {
    if (typeof feature_Audience !== 'undefined' && feature_Audience.GRADES) return feature_Audience.GRADES;
    return this.RANKS;
  },

  // where the person is going: the selected project, read off the page's
  // own block — the render chain put it there, so no fetch and no race
  project() {
    const el = document.querySelector('.doors');
    return {
      id: (el && el.getAttribute('data-project')) || '',
      title: (el && el.getAttribute('data-project-title')) || '',
    };
  },

  open(face) {
    this.face = face;
    this.say.textContent = '';
    this.rank.value = this.DEFAULT;
    const p = this.project();
    this.line.textContent = p.id ? ('into ' + (p.title || 'the selected project')) : 'no project selected';
    this.box.setAttribute('data-face', face);
    this.go.textContent = face === 'qr' ? 'show' : 'invite';
    this.sheet.classList.add('show');
    if (face === 'name') setTimeout(() => { try { this.name.focus(); } catch (e) {} }, 0);
  },

  close() {
    this.sheet.classList.remove('show');
    this.face = '';
  },

  async post(url, body) {
    try {
      const r = await fetch(url, {
        method: 'POST', cache: 'no-store',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(body),
      });
      const d = await r.json().catch(() => null);
      return d || { ok: false, error: "that didn't land" };
    } catch (e) {
      return { ok: false, error: 'no signal — try again in a moment' };
    }
  },

  // the name road: /invite's route, with a rank and a project beside the
  // name and the number. A success folds the sheet and clears it; a refusal
  // shows the server's own sentence and keeps what was typed.
  async invite() {
    if (this.busy) return;
    const name = this.name.value.trim();
    const phone = this.phone.value.trim();
    const p = this.project();
    const body = { name, phone, rank: this.rank.value };
    if (p.id) body.project = p.id;
    this.busy = true;
    const d = await this.post('users/invite', body);
    this.busy = false;
    if (!d.ok) { this.say.textContent = d.error || "that invite didn't land"; return; }
    this.name.value = '';
    this.phone.value = '';
    this.close();
    if (typeof feature_Invite !== 'undefined' && feature_Invite.pull) feature_Invite.pull();
  },

  // the QR road: mint with the rank (and the project) and hand the answer to
  // /qr's sheet exactly as its own open would — the code fills the screen
  async show() {
    if (this.busy || typeof feature_Qr === 'undefined') return;
    const p = this.project();
    const body = { fresh: false, rank: this.rank.value };
    if (p.id) body.project = p.id;
    this.busy = true;
    const d = await this.post('users/invite/qr/mint', body);
    this.busy = false;
    if (!d.ok) { this.say.textContent = d.error || "couldn't make a code"; return; }
    this.close();
    d.open = true;
    feature_Qr.send(d);
  },
};

{
  const fm_doorSheet = document.createElement('div');
  fm_doorSheet.id = 'doorSheet';
  const fm_doorBox = document.createElement('div');
  fm_doorBox.id = 'doorBox';

  const fm_doorName = document.createElement('input');
  fm_doorName.className = 'door-name';
  fm_doorName.placeholder = 'name';
  fm_doorName.autocomplete = 'off';
  const fm_doorPhone = document.createElement('input');
  fm_doorPhone.className = 'door-phone';
  fm_doorPhone.placeholder = 'phone';
  fm_doorPhone.inputMode = 'tel';
  fm_doorPhone.autocomplete = 'off';
  const fm_doorRank = document.createElement('select');
  fm_doorRank.className = 'door-rank';
  for (const g of feature_Doors.ranks()) {
    const o = document.createElement('option');
    o.value = g;
    o.textContent = g;
    fm_doorRank.appendChild(o);
  }
  const fm_doorLine = document.createElement('div');
  fm_doorLine.className = 'door-line';
  const fm_doorSay = document.createElement('div');
  fm_doorSay.className = 'door-say';
  const fm_doorBar = document.createElement('div');
  fm_doorBar.className = 'door-bar';
  const fm_doorCancel = document.createElement('button');
  fm_doorCancel.type = 'button';
  fm_doorCancel.className = 'door-cancel';
  fm_doorCancel.textContent = 'cancel';
  const fm_doorGo = document.createElement('button');
  fm_doorGo.type = 'button';
  fm_doorGo.className = 'door-go';
  fm_doorGo.textContent = 'invite';
  fm_doorBar.appendChild(fm_doorCancel);
  fm_doorBar.appendChild(fm_doorGo);

  fm_doorBox.appendChild(fm_doorName);
  fm_doorBox.appendChild(fm_doorPhone);
  fm_doorBox.appendChild(fm_doorRank);
  fm_doorBox.appendChild(fm_doorLine);
  fm_doorBox.appendChild(fm_doorSay);
  fm_doorBox.appendChild(fm_doorBar);
  fm_doorSheet.appendChild(fm_doorBox);
  document.body.appendChild(fm_doorSheet);

  feature_Doors.sheet = fm_doorSheet;
  feature_Doors.box = fm_doorBox;
  feature_Doors.name = fm_doorName;
  feature_Doors.phone = fm_doorPhone;
  feature_Doors.rank = fm_doorRank;
  feature_Doors.line = fm_doorLine;
  feature_Doors.say = fm_doorSay;
  feature_Doors.go = fm_doorGo;

  fm_doorPhone.addEventListener('keydown', (e) => {
    if (e.key === 'Enter') { e.preventDefault(); feature_Doors.invite(); }
  });

  // CAPTURE, and the sheet swallows its own taps: /backdrop closes the open
  // tool on any tap that lands on nobody's ground and its list of owners is
  // its own file, not a seam — /qr's sheet claims its taps the same way. A
  // stop in the capture phase never reaches the buttons' own listeners, so
  // the sheet's three taps are decided here too. A tap on the dark outside
  // the card is the way out, as on every sheet.
  document.addEventListener('click', (e) => {
    if (!e.target || !e.target.closest) return;
    if (e.target.closest('#doorSheet')) {
      e.stopPropagation();
      if (e.target === fm_doorSheet || e.target.closest('.door-cancel')) {
        e.preventDefault();
        feature_Doors.close();
      } else if (e.target.closest('.door-go')) {
        e.preventDefault();
        if (feature_Doors.face === 'qr') feature_Doors.show(); else feature_Doors.invite();
      }
      return;
    }
    const door = e.target.closest('[data-door]');
    if (!door) return;
    e.stopPropagation();
    feature_Doors.open(door.getAttribute('data-door'));
  }, true);

  // with /qr unticked there is no code to show: the button goes
  if (typeof feature_Qr === 'undefined') {
    const fm_doorNoQr = document.createElement('style');
    fm_doorNoQr.textContent = '.door[data-door="qr"] { display: none; }';
    document.head.appendChild(fm_doorNoQr);
  }

  // no pencil on the invite page (#p70): /editing/toolbar draws it for
  // whatever feature_Editing.page() answers, and the invite page is a
  // .card-page too. Answer nothing for it; the toolbar's own apply then
  // removes the button on the next paint, and card pages are untouched.
  if (typeof feature_Editing !== 'undefined' && feature_Editing.page) {
    const fm_doorPageWas = feature_Editing.page.bind(feature_Editing);
    feature_Editing.page = function () {
      const p = fm_doorPageWas();
      return (p && p.classList.contains('invite-page')) ? null : p;
    };
  }
}
