{
  if (typeof feature_LongPress !== 'undefined') {
    const fm_furtherShow = feature_LongPress.show;
    feature_LongPress.show = async function (btn) {
      await fm_furtherShow.call(this, btn);
      const c = document.getElementById('toolCard');
      if (!c || c.style.display === 'none') return;
      const top = parseFloat(c.style.top) || 0;
      c.style.top = Math.max(8, top - 12) + 'px';
    };
  }
}
