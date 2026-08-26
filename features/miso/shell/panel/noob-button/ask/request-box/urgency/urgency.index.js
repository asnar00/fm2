// two ways to send an ask to the builder: urgent, or whenever. The asker
// knows; triage should not have to guess. /ask's file() gains an urgency
// and its results footer gets two buttons in place of one.
if (typeof feature_Ask !== 'undefined') {
  feature_Ask.file = function (text, urgency) {
    feature_Loop.send({ type: 'Ask', data: { t: Date.now(), text, urgency: urgency || 'whenever' } });
  };
  const fm_urgGo = feature_Ask.go.bind(feature_Ask);
  feature_Ask.go = async function () {
    await fm_urgGo();
    const old = document.getElementById('askSend');
    if (!old) return;
    const box = document.getElementById('askResults');
    const input = document.getElementById('askText');
    const text = (input && input.value || '').trim();
    const wrap = old.parentElement;
    wrap.innerHTML = '<span class="askwhy">not it? send to the builder</span>'
      + '<button id="askUrgent">urgent</button><button id="askWhenever">whenever</button>';
    const file = (why) => {
      feature_Ask.file(text, why);
      box.innerHTML = '<div class="asknote">filed' + (why === 'urgent' ? ', urgent' : '') + ' — the builder will see it</div>';
      if (input) input.value = '';
    };
    document.getElementById('askUrgent').onclick = () => file('urgent');
    document.getElementById('askWhenever').onclick = () => file('whenever');
  };
}
