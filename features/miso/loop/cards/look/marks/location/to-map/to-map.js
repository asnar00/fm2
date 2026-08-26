// the pill goes to the map: tapping "map location" leaves the card and
// switches the surface to its map view, where this card is a pin among
// the others — the placeholder sheet is retired. Replaced at load (/me's
// idiom); /location's own click handler calls show() by name.
if (typeof feature_Location !== 'undefined') {
  feature_Location.show = function (pill) {
    if (typeof feature_Loop === 'undefined' || !feature_Loop.state) return;
    let open = '';
    try { open = JSON.parse(feature_Loop.state).open_tool || ''; } catch (e) {}
    // back to the set (the tool's own button is the way back from a card),
    // then the map view of that set
    if (open) feature_Loop.send({ type: 'click', ev: 'tool_' + open });
    setTimeout(() => feature_Loop.send({ type: 'click', ev: 'browse_map' }), 150);
  };
}
