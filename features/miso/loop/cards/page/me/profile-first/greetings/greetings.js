// the tour waits while a welcome page is on screen
const feature_Greetings = {
  up() { return !!document.getElementById('greetSheet'); },
};
if (typeof feature_Tour !== 'undefined' && typeof feature_Tour.may === 'function') {
  const fm_greetMay = feature_Tour.may.bind(feature_Tour);
  feature_Tour.may = function (s) {
    if (feature_Greetings.up()) return false;
    return fm_greetMay(s);
  };
}
