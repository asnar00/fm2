// the camera the next recording asks for, chosen with the camera button in
// the recording row. The Rust half holds the value; this half is where a
// camera is actually asked for, so this is where the value has to arrive.
const feature_Armed = {
  // the bridged var. `front` is the default, so a device that has never
  // touched the button — and a state that has not arrived yet — reads front,
  // which is the ask ("default selfie").
  camera() {
    let s = {};
    try { s = JSON.parse(feature_Loop.state || '{}'); } catch (e) {}
    return s.camera === 'back' ? 'environment' : 'user';
  },

  // two ways to reach /capture/video's `constraints()`, and which one is
  // right depends on what is composed beside this node.
  //
  // With /flip ticked, its `constraints()` calls `feature_Flip.facing()` at
  // the moment the camera is asked for — so redefining THAT one function is
  // enough, and it is order-proof: /flip's own install may run before or
  // after this file and the answer is the same either way. (Wrapping
  // `constraints` instead would be a race — /flip installs off a 100 ms
  // interval and would replace whatever it found.)
  //
  // With /flip unticked there is no facing to redefine, so the constraint is
  // written straight onto /video. Both are typeof-guarded: a sibling being
  // unticked is the absence this half has to survive, and with /video gone
  // too there is no camera to point and nothing here runs.
  install() {
    if (typeof feature_Flip !== 'undefined') {
      feature_Flip.facing = function () {
        return feature_Armed.camera();
      };
      return true;
    }
    if (typeof feature_Video !== 'undefined') {
      if (feature_Video.fm_armedWrapped) return true;
      feature_Video.fm_armedWrapped = true;
      feature_Video.constraints = function () {
        return { video: { facingMode: feature_Armed.camera() }, audio: true };
      };
      return true;
    }
    return false;
  },
};

{
  // the long-press words for the two levels and the four buttons. Written
  // into /tool-words' own tables from here rather than into its file: the
  // tables are one place on purpose, and a node that adds a button adds its
  // line (as-sub-tools named this as the next node's business and this is
  // it). typeof-guarded — with /tool-words unticked each button's `title` is
  // what the card falls back to.
  //
  // `record` and `level` go in TOOLS, not BUTTONS: `words()` routes anything
  // beginning `tool_` to the tool table and never looks in the other one, and
  // both are levels of the tree rather than acts inside one.
  if (typeof feature_ToolWords !== 'undefined') {
    feature_ToolWords.TOOLS.record = {
      name: 'record',
      intro: 'Set the camera and who the post reaches, then record.',
    };
    feature_ToolWords.TOOLS.level = {
      name: 'publish level',
      intro: 'Who your next posts reach. Same as me, or any rank at or below your own.',
    };
    feature_ToolWords.BUTTONS.armed_flip = {
      name: 'camera',
      intro: 'Front camera, back camera. It stays as you leave it.',
    };
    // /video's own two events already have lines; this only names the row
    // they now stand in, and leaves them alone.
  }

  // /flip and /video both install themselves off a 100 ms interval, so this
  // one waits the same way rather than assuming a load order. It gives up
  // after ten seconds: with neither node composed there is no camera to point
  // and an interval that never ends is a leak, not a retry.
  let fm_armedTries = 0;
  const fm_armedInit = setInterval(() => {
    fm_armedTries = fm_armedTries + 1;
    if (feature_Armed.install() || fm_armedTries > 100) {
      clearInterval(fm_armedInit);
    }
  }, 100);
}
