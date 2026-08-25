const feature_FromPicture = {
  // EXIF lives at the front of a JPEG and an APP1 segment cannot exceed 64KB,
  // so the first 256KB is the whole of what is worth reading out of a photo
  // that is several megabytes long.
  MAX: 262144,

  // the place read out of the chosen file, held until the picture itself
  // lands: a framing that is cancelled must leave the card as it was.
  pending: null,

  // ---- the read --------------------------------------------------------
  // decimal degrees from the file's own EXIF GPS tag, or null. The slice and
  // the whole walk sit inside one try: a malformed file is "no tag", never a
  // throw.
  async tag(file) {
    try {
      if (!file || typeof file.slice !== 'function') return null;
      const head = file.slice(0, this.MAX);
      const buf = (typeof head.arrayBuffer === 'function')
        ? await head.arrayBuffer()
        : await new Response(head).arrayBuffer();
      if (!buf || buf.byteLength < 4) return null;
      return this.parse(new DataView(buf));
    } catch (e) {
      return null;
    }
  },

  // the segment walk: SOI, then marker after marker to the first APP1 whose
  // data begins "Exif\0\0". Anything out of step ends the walk with no tag.
  parse(v) {
    const n = v.byteLength;
    if (v.getUint16(0) !== 0xffd8) return null;
    let p = 2;
    while (p + 4 <= n) {
      if (v.getUint8(p) !== 0xff) return null;
      const marker = v.getUint8(p + 1);
      if (marker === 0xff) { p += 1; continue; }           // fill byte
      if (marker === 0x01 || (marker >= 0xd0 && marker <= 0xd8)) {
        p += 2; continue;                                  // no length of its own
      }
      if (marker === 0xd9 || marker === 0xda) return null;  // the pixels begin
      const len = v.getUint16(p + 2);
      if (len < 2 || p + 2 + len > n) return null;
      if (marker === 0xe1 && this.exif(v, p + 4)) return this.gps(v, p + 10, n);
      p += 2 + len;
    }
    return null;
  },

  exif(v, at) {
    if (at + 6 > v.byteLength) return false;
    return v.getUint8(at) === 0x45 && v.getUint8(at + 1) === 0x78
        && v.getUint8(at + 2) === 0x69 && v.getUint8(at + 3) === 0x66
        && v.getUint8(at + 4) === 0 && v.getUint8(at + 5) === 0;
  },

  // the TIFF block: byte order honoured, IFD0 followed to the GPS IFD, and
  // the two coordinates read out of it. Every number is checked before it is
  // believed — a place that is not a place is no tag at all.
  gps(v, tiff, end) {
    if (tiff + 8 > end) return null;
    const bo = v.getUint16(tiff);
    if (bo !== 0x4949 && bo !== 0x4d4d) return null;
    const le = (bo === 0x4949);
    if (v.getUint16(tiff + 2, le) !== 42) return null;
    const at = this.find(v, tiff, v.getUint32(tiff + 4, le), end, le, 0x8825);
    if (at === null) return null;
    const lat = this.deg(v, tiff, at, end, le, 0x0001, 0x0002, 'S');
    const lon = this.deg(v, tiff, at, end, le, 0x0003, 0x0004, 'W');
    if (lat === null || lon === null) return null;
    if (lat < -90 || lat > 90 || lon < -180 || lon > 180) return null;
    // a camera with no fix of its own writes zeros; the Gulf of Guinea loses
    if (lat === 0 && lon === 0) return null;
    return { lat: lat, lon: lon };
  },

  // one IFD, entry by entry — and the one place the bounds are checked, so
  // neither reader below can follow an offset into space.
  each(v, tiff, off, end, le, fn) {
    const at = tiff + off;
    if (off <= 0 || at < tiff || at + 2 > end) return;
    const count = v.getUint16(at, le);
    if (count < 1 || count > 512) return;
    if (at + 2 + count * 12 > end) return;
    for (let i = 0; i < count; i++) {
      const e = at + 2 + i * 12;
      fn(v.getUint16(e, le), v.getUint16(e + 2, le), v.getUint32(e + 4, le),
         e + 8);
    }
  },

  // a tag whose value is one LONG — the GPS IFD's own offset
  find(v, tiff, off, end, le, want) {
    let out = null;
    this.each(v, tiff, off, end, le, (tag, type, count, valAt) => {
      if (tag === want && type === 4 && count === 1)
        out = v.getUint32(valAt, le);
    });
    return out;
  },

  // a reference letter plus three rationals: degrees, minutes, seconds. Two
  // ASCII bytes fit inside the entry; twenty-four bytes of rationals never
  // do, so those are always at an offset.
  deg(v, tiff, off, end, le, refTag, valTag, neg) {
    let ref = '';
    let dms = null;
    this.each(v, tiff, off, end, le, (tag, type, count, valAt) => {
      if (tag === refTag && type === 2 && count >= 1 && count <= 4)
        ref = String.fromCharCode(v.getUint8(valAt));
      if (tag === valTag && type === 5 && count === 3) {
        const a = tiff + v.getUint32(valAt, le);
        if (a < tiff || a + 24 > end) return;
        dms = [this.rat(v, a, le), this.rat(v, a + 8, le),
               this.rat(v, a + 16, le)];
      }
    });
    if (!dms || dms.indexOf(null) >= 0) return null;
    const d = dms[0] + dms[1] / 60 + dms[2] / 3600;
    if (!isFinite(d)) return null;
    return (ref.toUpperCase() === neg) ? -d : d;
  },

  rat(v, at, le) {
    const den = v.getUint32(at + 4, le);
    if (!den) return null;
    return v.getUint32(at, le) / den;
  },

  // ---- the sheet's third line ------------------------------------------
  // which of the two this place came from. /location draws no source at all,
  // so a pill with no attribute is a place it took itself.
  said(pill) {
    const line = document.getElementById('placeSource');
    if (!line) return;
    const src = (pill && pill.getAttribute) ? pill.getAttribute('data-source') : '';
    line.textContent = (src === 'picture') ? 'from the picture' : 'from this phone';
  },
};

{
  // the seam is /cards' `chose`, taken by redefinition and kept in a closure —
  // /frame takes the same one, and this link sits OUTSIDE it because
  // provenance puts this node later. So the file read here is the ORIGINAL the
  // user picked, before /frame's canvas throws the EXIF away. It reads, then
  // hands the file on untouched; nothing here knows anything about /frame.
  if (typeof feature_Cards !== 'undefined') {
    const fm_picChose = feature_Cards.chose;
    feature_Cards.chose = async function (file) {
      const t = this.target;
      const id = (t && t.id) ? t.id : '';
      feature_FromPicture.pending = null;
      if (file && id) {
        const at = await feature_FromPicture.tag(file);
        if (at) feature_FromPicture.pending = { id: id, lat: at.lat, lon: at.lon };
      }
      return fm_picChose.call(this, file);
    };
  }

  // the place travels on the back of the picture: /cards sends CardPic at the
  // end of both paths (framed and unframed), so a cancelled framing or a photo
  // refused for being too big never reaches here and the card is untouched.
  // A load-time redefinition of a named function — NOT a timer wrapping
  // apply, which is the race in notes.md.
  if (typeof feature_Loop !== 'undefined') {
    const fm_picSend = feature_Loop.send;
    feature_Loop.send = function (event) {
      const out = fm_picSend.call(this, event);
      const p = feature_FromPicture.pending;
      if (p && event && event.type === 'CardPic'
          && event.data && event.data.id === p.id) {
        feature_FromPicture.pending = null;
        fm_picSend.call(this, { type: 'CardPlace', data: {
          id: p.id, lat: p.lat, lon: p.lon, acc: 0,
          t: Date.now(), source: 'picture' } });
      }
      return out;
    };
  }

  // /location's sheet, told to say where its place came from. Its `show` is
  // taken the same way, so location.js is not edited.
  if (typeof feature_Location !== 'undefined') {
    const fm_picShow = feature_Location.show;
    feature_Location.show = function (pill) {
      const out = fm_picShow.call(this, pill);
      feature_FromPicture.said(pill);
      return out;
    };
  }

  // the line itself, made at load into /location's box — under the accuracy,
  // above the close, the quietest thing in the sheet
  {
    const fm_picBox = document.getElementById('placeBox');
    if (fm_picBox) {
      const fm_picLine = document.createElement('div');
      fm_picLine.id = 'placeSource';
      const fm_picClose = document.getElementById('placeClose');
      if (fm_picClose) fm_picBox.insertBefore(fm_picLine, fm_picClose);
      else fm_picBox.appendChild(fm_picLine);
    }
  }
}
