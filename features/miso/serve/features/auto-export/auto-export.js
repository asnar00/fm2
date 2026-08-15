const feature_AutoExport = {
  // the last stamp the held catalog was read under
  stamp: null,
};
{
  if (typeof feature_Chooser !== 'undefined') {
    const fm_autoExportLoad = feature_Chooser.load.bind(feature_Chooser);
    feature_Chooser.load = async function () {
      let s = null;
      try {
        s = await fetch('features/stamp', { cache: 'no-store' })
          .then((r) => (r.ok ? r.text() : null));
      } catch (e) {}
      // a stamp that moved since the last read: forget, so the original
      // refetches the words the server is actually serving
      if (s && feature_AutoExport.stamp && s !== feature_AutoExport.stamp) {
        feature_Chooser.flat = null;
        feature_Chooser.byPath = null;
      }
      await fm_autoExportLoad();
      if (s) feature_AutoExport.stamp = s;
    };
  }
}
