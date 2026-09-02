{
  // the engineer section's tenant (engineer.agent.md): capture fill, replace
  // the property at load, call the captured one first, append one line. The
  // stocking object is declared by this node's other fragment, composed after
  // this one, so it is reached at fill time and never at load.
  if (typeof feature_Engineer !== 'undefined') {
    const fm_stFill = feature_Engineer.fill;
    feature_Engineer.fill = function (box) {
      fm_stFill.call(this, box);
      if (typeof feature_Stocked === 'undefined') return;
      const line = document.createElement('div');
      line.id = 'stocked';
      line.textContent = feature_Stocked.text();
      box.appendChild(line);
    };
  }
}
