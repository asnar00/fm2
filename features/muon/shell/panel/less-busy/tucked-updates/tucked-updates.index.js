const feature_TuckedUpdates = {
  place() {
    const box = $('changes');
    const seg = $('policySeg');
    if (!box || !seg || !box.classList.contains('chooser-home')) return;
    let tucked = document.getElementById('tucked');
    if (!tucked) {
      tucked = document.createElement('div');
      tucked.id = 'tucked';
      const firstRow = box.querySelector(':scope > .crow');
      if (firstRow) box.insertBefore(tucked, firstRow);
      else box.appendChild(tucked);
    }
    tucked.appendChild(seg);
    this.show();
  },
  // the container is OURS to show and hide (#p81 kept honestly)
  show() {
    const box = $('changes');
    const tucked = document.getElementById('tucked');
    if (!box || !tucked) return;
    tucked.style.display = box.classList.contains('folded') ? 'none' : 'block';
  },
};
{
  if (typeof feature_Chooser !== 'undefined') {
    const fm_tuckedMount = feature_Chooser.mount.bind(feature_Chooser);
    feature_Chooser.mount = async function () {
      await fm_tuckedMount();
      feature_TuckedUpdates.place();
    };
  }
  if (typeof feature_FeaturesButton !== 'undefined') {
    const fm_tuckedToggle = feature_FeaturesButton.toggle.bind(feature_FeaturesButton);
    feature_FeaturesButton.toggle = function () {
      fm_tuckedToggle();
      feature_TuckedUpdates.show();
    };
  }
}
