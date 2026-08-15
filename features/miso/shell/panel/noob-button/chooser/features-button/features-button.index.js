const feature_FeaturesButton = {
  toggle() {
    const box = $('changes');
    if (box) box.classList.toggle('folded');
    const btn = $('featuresBtn');
    if (btn && box)
      btn.textContent = box.classList.contains('folded') ? 'features' : 'features ▾';
  },
};
{
  if (typeof feature_Chooser !== 'undefined') {
    const fm_row = document.createElement('div');
    fm_row.className = 'row';
    fm_row.id = 'featuresRow';
    fm_row.innerHTML = '<button id="featuresBtn">features</button>';
    const fm_anchor = $('policySeg')
      || ($('updateBtn') ? $('updateBtn').closest('.row') : null);
    if (fm_anchor) fm_anchor.after(fm_row);
    else if ($('panel')) $('panel').appendChild(fm_row);
    $('featuresBtn').onclick = () => feature_FeaturesButton.toggle();

    const fm_featMount = feature_Chooser.mount.bind(feature_Chooser);
    feature_Chooser.mount = async function () {
      await fm_featMount();
      const box = $('changes');
      if (box) box.classList.add('folded'); // every fresh mount starts folded
      const btn = $('featuresBtn');
      if (btn) btn.textContent = 'features';
    };
    // a drill-down must land somewhere visible
    const fm_featGoto = feature_Chooser.goto.bind(feature_Chooser);
    feature_Chooser.goto = function (path) {
      const box = $('changes');
      if (box && box.classList.contains('folded')) feature_FeaturesButton.toggle();
      fm_featGoto(path);
    };
  }
}
