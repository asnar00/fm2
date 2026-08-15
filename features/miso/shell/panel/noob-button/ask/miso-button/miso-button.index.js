const feature_MisoButton = {};
{
  // the parent built the row already (fragments load in provenance order);
  // say the app's own word: type a wish, press miso — make it so
  const fm_askGo = document.getElementById('askGo');
  if (fm_askGo) fm_askGo.textContent = 'miso';
}
