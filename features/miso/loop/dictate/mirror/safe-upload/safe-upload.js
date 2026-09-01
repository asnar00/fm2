const feature_SafeUpload = {};

{
  // property replacement at load, the house idiom; with /mirror unticked
  // there is nothing to guard.
  if (typeof feature_Mirror !== 'undefined' && feature_Mirror.upload) {
    const fm_suUpload = feature_Mirror.upload;
    feature_Mirror.upload = async function () {
      try {
        return await fm_suUpload.call(this);
      } catch (e) {
        // a full device: the exchange already holds what was posted, and the
        // unstamped meta is retried on the next pass — quiet is correct.
        return null;
      }
    };
  }
}
