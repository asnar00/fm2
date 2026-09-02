const feature_StandIn = {
  // how many levels up a missing square may borrow from. Three turns "I
  // looked at the district zoomed out" into "the district is drawn at street
  // zoom"; beyond that a stand-in is sixteen squares wide and reads as a
  // colour, not a map.
  REACH: 3,
  Layer: null,
  count: 0,     // stand-in requests made, for the rig's eyes

  // the L.TileLayer subclass, made once. Everything Leaflet does with a tile
  // stays as it was; only the error path changes, and only while a parent is
  // within reach.
  layer() {
    if (this.Layer) return this.Layer;
    if (typeof L === 'undefined') return null;
    const self = this;
    this.Layer = L.TileLayer.extend({
      // a tile is a frame Leaflet positions, with the picture inside it.
      // Leaflet's own tile IS the img, and a scaled img that carries the
      // tile's transform is drawn coarse and cut at its edges (Chrome, DPR 2
      // and 3, measured 2026-09-02); the same picture scaled inside a frame
      // that carries the transform is smooth and seamless. The frame answers
      // `src` and `complete` for the img so Leaflet's abort and prune paths,
      // written for an img, keep working.
      createTile(coords, done) {
        const tile = document.createElement('div');
        const img = document.createElement('img');
        const size = this.getTileSize();
        tile.style.overflow = 'hidden';
        img.style.position = 'absolute';
        img.style.left = '0';
        img.style.top = '0';
        img.style.width = size.x + 'px';
        img.style.height = size.y + 'px';
        img.alt = '';
        if (this.options.crossOrigin || this.options.crossOrigin === '') {
          img.crossOrigin = this.options.crossOrigin === true ? '' : this.options.crossOrigin;
        }
        if (typeof this.options.referrerPolicy === 'string') {
          img.referrerPolicy = this.options.referrerPolicy;
        }
        L.DomEvent.on(img, 'load', L.Util.bind(this._tileOnLoad, this, done, tile));
        L.DomEvent.on(img, 'error', L.Util.bind(this._tileOnError, this, done, tile));
        Object.defineProperty(tile, 'complete', { get: () => img.complete });
        Object.defineProperty(tile, 'src', {
          get: () => img.src,
          set: (v) => { img.src = v; },
        });
        tile._standIn = { coords: coords, up: 0, img: img };
        tile.appendChild(img);
        img.src = this.getTileUrl(coords);
        return tile;
      },

      // Leaflet's own getTileUrl takes the zoom from the map, not the
      // coordinates; a parent's url must come from ITS coordinates and from
      // this layer's live template — the string /fresh-tiles stamped the
      // ground tag onto — so the stand-in carries the tag too.
      getTileUrl(coords) {
        if (!coords.standIn) return L.TileLayer.prototype.getTileUrl.call(this, coords);
        const data = {
          r: L.Browser.retina ? '@2x' : '',
          s: this._getSubdomain(coords),
          x: coords.x,
          y: coords.y,
          z: coords.z,
        };
        return L.Util.template(this._url, L.Util.extend(data, this.options));
      },

      _tileOnError(done, tile, e) {
        const s = tile._standIn;
        // a tile Leaflet has already pruned or aborted is blanked to its empty
        // image and taken off the map before the error can land: hand it
        // back, and Leaflet's own path finds no tile under that key. Past
        // the reach, the same — the square stays the ground, tileerror fires.
        if (!s || s.up >= self.REACH || !tile.parentNode
            || s.img.getAttribute('src') === L.Util.emptyImageUrl) {
          return L.TileLayer.prototype._tileOnError.call(this, done, tile, e);
        }
        s.up += 1;
        const n = 1 << s.up;
        const parent = L.point(Math.floor(s.coords.x / n), Math.floor(s.coords.y / n));
        parent.z = s.coords.z - s.up;
        parent.standIn = true;
        self.dress(s.img, this.getTileSize(), s.coords, s.up);
        self.count += 1;
        s.img.src = this.getTileUrl(parent);
      },

      // Leaflet blanks a pruned tile's src before removing it, so a request
      // in flight is dropped; the frame's setter forwards that to the img.
      _removeTile(key) {
        const t = this._tiles[key];
        if (t && t.el._standIn) t.el._standIn.img.src = L.Util.emptyImageUrl;
        return L.TileLayer.prototype._removeTile.call(this, key);
      },
    });
    return this.Layer;
  },

  // the parent square, drawn 2^up tiles wide inside the frame and shifted so
  // the missing square's quadrant lands in it; the frame's overflow crops
  // the rest, so a stand-in never covers a neighbour that did load.
  dress(img, size, coords, up) {
    const n = 1 << up;
    img.style.width = (size.x * n) + 'px';
    img.style.height = (size.y * n) + 'px';
    img.style.left = (-(coords.x % n) * size.x) + 'px';
    img.style.top = (-(coords.y % n) * size.y) + 'px';
  },
};

{
  // /map's mount, taken by property replacement at load — /quiet-credits'
  // idiom on the same function, not a timer (notes.md, "the apply-wrapper
  // race"). For the one call that makes the tile layer, L.tileLayer hands
  // back this node's subclass; the factory is put back whatever happens.
  if (typeof feature_Map !== 'undefined') {
    const fm_siMount = feature_Map.mount;
    feature_Map.mount = function () {
      const Layer = feature_StandIn.layer();
      if (!Layer) return fm_siMount.call(this);
      const fm_siFactory = L.tileLayer;
      L.tileLayer = function (url, options) { return new Layer(url, options); };
      try {
        return fm_siMount.call(this);
      } finally {
        L.tileLayer = fm_siFactory;
      }
    };
  }
}
