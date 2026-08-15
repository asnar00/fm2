const feature_FreshCatalog = {};
{
  if (typeof feature_Delta !== 'undefined' && typeof feature_Chooser !== 'undefined') {
    const fm_freshCatalogQuiet = feature_Delta.quiet.bind(feature_Delta);
    feature_Delta.quiet = async function (build) {
      // forget before anything downstream re-renders: the next reader
      // fetches the build the device is actually running
      feature_Chooser.flat = null;
      feature_Chooser.byPath = null;
      await fm_freshCatalogQuiet(build);
    };
  }
}
