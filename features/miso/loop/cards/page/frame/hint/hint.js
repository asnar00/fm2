// one line above the framing window: the two gestures, and what the square is
const feature_Hint = {
  words: 'pinch to zoom, drag to move',
};
if (typeof feature_Frame !== 'undefined' && feature_Frame.sheet && feature_Frame.win) {
  const fm_hint = document.createElement('div');
  fm_hint.id = 'frameHint';
  fm_hint.textContent = feature_Hint.words;
  feature_Frame.sheet.insertBefore(fm_hint, feature_Frame.win);
}
