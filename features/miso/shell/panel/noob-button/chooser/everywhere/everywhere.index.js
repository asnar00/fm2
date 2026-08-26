// a chooser row carries its tick wherever it is drawn: the "do something"
// box's results are chooser rows, so a found feature can be switched on
// or off right there. The tick's click is already the loop's (ftick_<path>);
// this node keeps the tick on the row and keeps it truthful.
const feature_Everywhere = {
  sync(box) {
    if (!box || typeof feature_Chooser === 'undefined') return;
    const t = feature_Chooser.ticks();
    for (const row of box.querySelectorAll('.crow[data-path]')) {
      const path = row.getAttribute('data-path');
      const parts = path.split('/');
      let effOn = true;
      for (let i = 1; i <= parts.length; i++) {
        if (t[parts.slice(0, i).join('/')] === false) { effOn = false; break; }
      }
      const tick = row.querySelector('.ctick');
      if (tick) tick.classList.toggle('on', t[path] !== false);
      row.classList.toggle('shaded', !effOn);
    }
  },
};
if (typeof feature_Ask !== 'undefined') {
  // the ticks stay, and say the truth as soon as the rows are drawn
  feature_Ask.stripTicks = (box) => feature_Everywhere.sync(box);
}
if (typeof feature_Chooser !== 'undefined' && feature_Chooser.reflect) {
  // and follow every change of the ticks, as the chooser's own list does
  const fm_everyReflect = feature_Chooser.reflect.bind(feature_Chooser);
  feature_Chooser.reflect = function () {
    fm_everyReflect();
    feature_Everywhere.sync(document.getElementById('askResults'));
  };
}
