// the build row — "build 347 · up to date" and the features button — moves
// below the requests list and the update row: what is pending comes first,
// what you are running comes after. Done at load, after /build-row has made
// the row; the update row may be tucked away (/tucked-updates), so the
// anchor is whichever of it and the list is there.
{
  const fm_row = document.getElementById('buildRow');
  const fm_upd = document.getElementById('updateBtn');
  const fm_anchor = (fm_upd && fm_upd.closest('.row')) || document.getElementById('changes');
  if (fm_row && fm_anchor && fm_anchor !== fm_row) fm_anchor.after(fm_row);
}
