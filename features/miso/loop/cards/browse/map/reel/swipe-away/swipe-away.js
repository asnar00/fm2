{
  let fm_swDown = null;
  document.addEventListener('pointerdown', (e) => {
    fm_swDown = null;
    if (!e.isPrimary || !document.body.classList.contains('fm-map-behind')) return;
    const page = e.target && e.target.closest ? e.target.closest('.card-page') : null;
    if (!page) return;
    if (e.target.closest('.frame-win')) return;
    const a = document.activeElement;
    if (a && a.getAttribute && a.getAttribute('contenteditable') === 'true') return;
    fm_swDown = { x: e.clientX, y: e.clientY, t: Date.now(), id: e.pointerId, page };
  }, true);
  document.addEventListener('pointerup', (e) => {
    const d = fm_swDown; fm_swDown = null;
    if (!d || e.pointerId !== d.id) return;
    const dx = e.clientX - d.x, dy = e.clientY - d.y;
    if (Date.now() - d.t > 600 || Math.abs(dx) < 60 || Math.abs(dy) >= 40) return;
    if (!document.body.contains(d.page)) return;
    d.page.classList.add(dx < 0 ? 'fm-swipe-left' : 'fm-swipe-right');
    let sent = false;
    const go = () => {
      if (sent) return;
      sent = true;
      let open = '';
      try { open = JSON.parse(feature_Loop.state).open_tool || ''; } catch (err) {}
      if (!open) open = 'posts';
      feature_Loop.send({ type: 'click', ev: 'tool_' + open });
    };
    d.page.addEventListener('animationend', go, { once: true });
    setTimeout(go, 260);
  }, true);
}
