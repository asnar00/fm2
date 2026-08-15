const feature_Veil = {
  joined: false,
  timer: null,
  reveal() {
    document.body.classList.add('fm-joined');
    const veil = $('veil');
    if (veil) veil.remove();
  },
  inform() {
    if ($('joinStale')) return;
    const b = document.createElement('div');
    b.id = 'joinStale';
    b.textContent = 'showing local state — server not reachable';
    document.body.appendChild(b);
  },
  clearInform() {
    const b = $('joinStale');
    if (b) b.remove();
  },
};
{
  const fm_veil = document.createElement('div');
  fm_veil.id = 'veil';
  fm_veil.textContent = 'syncing…';
  document.body.appendChild(fm_veil);

  const fm_gateApply = feature_Loop.apply;
  feature_Loop.apply = function (p) {
    fm_gateApply.call(this, p);
    let joined = false;
    try {
      joined = !!JSON.parse(feature_Loop.state || '{}')._joined;
    } catch (e) {}
    if (joined && !feature_Veil.joined) {
      feature_Veil.joined = true;
      feature_Veil.clearInform();
      feature_Veil.reveal();
    } else if (!feature_Veil.joined && !feature_Veil.timer) {
      // first apply = paint-readiness: the timeout budget starts here, not
      // at script load, so a slow wasm fetch doesn't eat it
      feature_Veil.timer = setTimeout(() => {
        if (!feature_Veil.joined) {
          feature_Veil.reveal();
          feature_Veil.inform();
        }
      }, 2000);
    }
  };
}
