const feature_LivePanel = {
  last: null,

  showing() {
    const p = $('panel');
    return !!p && p.style.display === 'block';
  },

  async refresh() {
    if (typeof feature_Review !== 'undefined') await feature_Review.section();
    if (typeof feature_LessBusy !== 'undefined') feature_LessBusy.refresh();
  },
};
{
  if (typeof feature_Update !== 'undefined')
    feature_LivePanel.last = feature_Update.server;

  if (typeof feature_Watch !== 'undefined') {
    const fm_livePanelCheck = feature_Watch.check.bind(feature_Watch);
    feature_Watch.check = async function () {
      const v = await fm_livePanelCheck();
      if (v && v !== feature_LivePanel.last) {
        feature_LivePanel.last = v;
        if (feature_LivePanel.showing()) feature_LivePanel.refresh();
      }
      return v;
    };
  }

  // a quiet apply changes the story without a reload: tell the panel
  if (typeof feature_Delta !== 'undefined') {
    const fm_livePanelQuiet = feature_Delta.quiet.bind(feature_Delta);
    feature_Delta.quiet = async function (build) {
      await fm_livePanelQuiet(build);
      if (feature_LivePanel.showing()) await feature_LivePanel.refresh();
    };
  }
}
