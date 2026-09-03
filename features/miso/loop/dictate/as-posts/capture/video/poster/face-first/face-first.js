// the face stays on the player until the clip plays. /poster's tap removes the
// frame and /capture/video mounts a bare <video>, which iOS paints black until
// playback; the picture the poster showed becomes the element's own poster.
{
  if (typeof feature_Poster !== 'undefined' && typeof feature_Video !== 'undefined') {
    feature_Poster.faces = feature_Poster.faces || {};
    const fm_faceOpen = feature_Poster.open.bind(feature_Poster);
    feature_Poster.open = function (h) {
      const id = h.getAttribute('data-vid');
      const img = h.querySelector('.poster-frame img');
      if (id && img && img.src) feature_Poster.faces[id] = img.src;
      return fm_faceOpen(h);
    };
    const fm_facePut = feature_Video.put.bind(feature_Video);
    feature_Video.put = function (holder, id) {
      fm_facePut(holder, id);
      const el = holder.querySelector('video');
      const face = feature_Poster.faces[id];
      if (el && face && !el.poster) el.poster = face;
    };
  }
}
