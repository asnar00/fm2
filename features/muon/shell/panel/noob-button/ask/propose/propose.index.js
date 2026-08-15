const feature_Propose = {
  pending: null, // an ask text parked by file(), waiting for the editor

  // the draft is the ask, stated plainly — the birthplace travels as data,
  // so the prose needn't repeat it (#p33: no "gains a new ability" filler)
  draft(text) {
    return text;
  },

  editor(text) {
    const box = $('askResults');
    if (!box || !text) return;
    for (const n of box.querySelectorAll('.asknote')) n.remove();
    let ed = document.getElementById('proposeBox');
    if (!ed) {
      ed = document.createElement('div');
      ed.id = 'proposeBox';
      ed.innerHTML =
        '<div class="pdraft">nothing does that yet — describe it for the builder,'
        + ' edit until it says what you mean:</div>'
        + '<textarea id="proposeText" rows="4"></textarea>'
        + '<div class="prow"><button id="proposeGo">propose</button></div>';
      box.appendChild(ed);
      document.getElementById('proposeGo').onclick = () => feature_Propose.fire();
    }
    ed.dataset.ask = text;
    const ta = document.getElementById('proposeText');
    ta.value = this.draft(text);
    ta.focus();
  },

  // the OK: text + approved paragraph + birthplace, through the durable outbox
  fire() {
    const ed = document.getElementById('proposeBox');
    const text = ed ? ed.dataset.ask : '';
    const ta = document.getElementById('proposeText');
    const proposal = (ta && ta.value || '').trim();
    if (!text || !proposal) return;
    const ctx = typeof feature_Birthplace !== 'undefined'
      ? feature_Birthplace.context() : {};
    feature_Loop.send({ type: 'Ask',
      data: Object.assign({ t: Date.now(), text, proposal }, ctx) });
    const box = $('askResults');
    if (box) box.innerHTML =
      '<div class="asknote">proposed — the builder will see it</div>';
    const input = $('askText');
    if (input) input.value = '';
  },
};
if (typeof feature_Ask !== 'undefined') {
  feature_Ask.file = function (text) {
    feature_Propose.pending = text; // the editor is the way to the builder now
  };
  const fm_proposeGo = feature_Ask.go.bind(feature_Ask);
  feature_Ask.go = async function () {
    const q = (($('askText') || {}).value || '').trim();
    await fm_proposeGo();
    if (feature_Propose.pending) {
      const t = feature_Propose.pending;
      feature_Propose.pending = null;
      feature_Propose.editor(t);
      return;
    }
    const send = document.getElementById('askSend');
    if (send) send.onclick = () => feature_Propose.editor(q);
  };
}
