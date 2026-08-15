const feature_Steady = {
  last: undefined, // open_tool as of the previous apply; undefined = not yet seen
};
{
  const fm_steadyApply = feature_Loop.apply;
  feature_Loop.apply = function (p) {
    fm_steadyApply.call(this, p);
    let open = null;
    try {
      const s = JSON.parse(feature_Loop.state || '{}');
      open = typeof s.open_tool === 'string' ? s.open_tool : null;
    } catch (e) {}
    // same mode as last render: still the re-mounted buttons before paint
    if (open === feature_Steady.last)
      for (const b of document.querySelectorAll('.toolbar .tool-button'))
        b.style.animation = 'none';
    feature_Steady.last = open;
  };
}
