// a tap on the ground outside an open card closes it: the tool's own
// button is the way back from a card, and this sends the same tap. Only
// the bare ground counts — the card, the toolbar, the picker, the lozenge,
// the panel and every body-level sheet are somebody's.
{
  const fm_backdropOwned = '.card-page, .toolbar, .browse-picker, #build, #panel, #shade, #projSheet, #frameSheet, #placeSheet, #cardToast, #misoMap, .leaflet-container, [data-ev], [contenteditable], input, button';
  document.addEventListener('click', (e) => {
    if (!e.target || !e.target.closest) return;
    if (!document.querySelector('.card-page')) return;           // nothing open
    if (e.target.closest(fm_backdropOwned)) return;               // that tap is somebody's
    if (typeof feature_Loop === 'undefined' || !feature_Loop.state) return;
    let open = '';
    try { open = JSON.parse(feature_Loop.state).open_tool || ''; } catch (err) {}
    if (!open) return;
    feature_Loop.send({ type: 'click', ev: 'tool_' + open });
  });
}
