const feature_Source = {
  open() { window.open('features/', '_blank'); },
};
{
  const fm_row = document.createElement('div');
  fm_row.className = 'row';
  fm_row.innerHTML = '<button id="sourceBtn">view source</button>';
  $('panel').insertBefore(fm_row, $('logoutBtn').closest('.row'));
  $('sourceBtn').onclick = () => feature_Source.open();
}
