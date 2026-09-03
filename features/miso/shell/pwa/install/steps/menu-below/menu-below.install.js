// the share step says where the menu is; the view-more step drops the glyph
// that differs between phones (#p100, the ruling)
const feature_MenuBelow = { active: true };
{
  const ios = document.getElementById('ios');
  const steps = ios ? ios.querySelectorAll('.step') : [];
  if (steps.length >= 2) {
    const key = steps[0].querySelector('.key');
    steps[0].textContent = 'tap ';
    if (key) steps[0].appendChild(key);
    steps[0].appendChild(document.createTextNode(' in the browser menu below'));
    steps[1].innerHTML = 'then <b>view more</b>';
  }
}
