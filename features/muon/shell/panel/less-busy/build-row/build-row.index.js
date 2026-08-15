{
  const fm_line = $('buildLine');
  if (fm_line) {
    const fm_row = document.createElement('div');
    fm_row.id = 'buildRow';
    fm_line.before(fm_row);
    fm_row.appendChild(fm_line);
    const fm_btn = $('featuresBtn');
    if (fm_btn) {
      const fm_oldRow = fm_btn.closest('.row');
      fm_row.appendChild(fm_btn);
      if (fm_oldRow && fm_oldRow.id === 'featuresRow') fm_oldRow.remove();
    }
  }
}
