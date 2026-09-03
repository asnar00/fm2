// the speaker in a sound-only post's pin face, in place of the initial
const feature_Sound = {
  svg: '<svg class="icon-svg pin-sound" viewBox="0 0 24 24" aria-hidden="true">'
    + '<path d="M4 9.5v5h3.5L12 18.5v-13L7.5 9.5z" fill="currentColor"/>'
    + '<path d="M15.5 9a4 4 0 0 1 0 6M18 6.5a7.5 7.5 0 0 1 0 11" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"/></svg>',
};
if (typeof feature_Map !== 'undefined') {
  const fm_soundPin = feature_Map.pinHtml;
  feature_Map.pinHtml = function (p) {
    const html = fm_soundPin.call(this, p);
    if (!p || !p.sound || p.face) return html;
    const at = html.indexOf('<span>');
    const end = html.indexOf('</span>', at);
    if (at < 0 || end < 0) return html;
    return html.slice(0, at) + feature_Sound.svg + html.slice(end + 7);
  };
}
