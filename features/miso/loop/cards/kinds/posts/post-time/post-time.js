const feature_PostTime = {
  // EXIF lives at the front of a JPEG and an APP1 segment cannot exceed 64KB,
  // so the first 256KB is the whole of what is worth reading — /from-picture's
  // number, for the same reason.
  MAX: 262144,

  // the time read out of the chosen file, held until the picture itself lands:
  // a framing that is cancelled, or a photograph refused for being too big,
  // must leave the card's time exactly as it was.
  pending: null,

  // ---- the read --------------------------------------------------------
  // epoch milliseconds from the photograph's own EXIF date, or null. The
  // slice and the whole walk sit inside one try: a malformed file is "no
  // date", never a throw.
  async taken(file) {
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
  // data begins "Exif\0\0". The same walk /from-picture makes for the GPS
  // tag, written again here rather than borrowed: that node exposes its IFD
  // readers but not the TIFF offset they need, and a post's time must still
  // be read when /location and /from-picture are unticked.
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
      if (marker === 0xe1 && this.exif(v, p + 4)) return this.date(v, p + 10, n);
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

  // the TIFF block: byte order honoured, IFD0 followed to the Exif sub-IFD
  // (pointer tag 0x8769) for DateTimeOriginal (0x9003) — the moment the
  // shutter opened. A file with no sub-IFD falls back to IFD0's own DateTime
  // (0x0132), which a scanner or an editor writes when the camera did not.
  date(v, tiff, end) {
    if (tiff + 8 > end) return null;
    const bo = v.getUint16(tiff);
    if (bo !== 0x4949 && bo !== 0x4d4d) return null;
    const le = (bo === 0x4949);
    if (v.getUint16(tiff + 2, le) !== 42) return null;
    const ifd0 = v.getUint32(tiff + 4, le);
    const sub = this.find(v, tiff, ifd0, end, le, 0x8769);
    if (sub !== null) {
      const t = this.ms(this.ascii(v, tiff, sub, end, le, 0x9003));
      if (t !== null) return t;
    }
    return this.ms(this.ascii(v, tiff, ifd0, end, le, 0x0132));
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

  // a tag whose value is one LONG — the Exif sub-IFD's own offset
  find(v, tiff, off, end, le, want) {
    let out = null;
    this.each(v, tiff, off, end, le, (tag, type, count, valAt) => {
      if (tag === want && type === 4 && count === 1)
        out = v.getUint32(valAt, le);
    });
    return out;
  },

  // an ASCII tag. "YYYY:MM:DD HH:MM:SS\0" is twenty bytes and so never fits
  // the four inside the entry, but the four-byte case is read anyway rather
  // than assumed away — a shorter string in the same tag is still a string.
  ascii(v, tiff, off, end, le, want) {
    let out = '';
    this.each(v, tiff, off, end, le, (tag, type, count, valAt) => {
      if (tag !== want || type !== 2 || count < 2 || count > 64) return;
      const at = (count <= 4) ? valAt : tiff + v.getUint32(valAt, le);
      if (at < tiff || at + count > end) return;
      let s = '';
      for (let i = 0; i < count; i++) {
        const b = v.getUint8(at + i);
        if (b === 0) break;
        s += String.fromCharCode(b);
      }
      out = s;
    });
    return out;
  },

  // EXIF dates carry no zone: the camera wrote the wall clock where the
  // picture was taken. It is read as local time and stored as the epoch
  // millisecond that reading names — which is right where the photograph was
  // taken, and is the parked question everywhere else (time zones are named
  // in the spec, not built). A blank date — EXIF's "    :  :     :  :  " —
  // fails the shape and is no date at all.
  ms(s) {
    if (!s) return null;
    const m = /^(\d{4}):(\d{2}):(\d{2})[ T](\d{2}):(\d{2}):(\d{2})/.exec(s);
    if (!m) return null;
    const y = +m[1];
    if (y < 1900 || y > 3000) return null;
    const t = new Date(y, +m[2] - 1, +m[3], +m[4], +m[5], +m[6]).getTime();
    if (!isFinite(t) || t <= 0) return null;
    return t;
  },

  // is this card one of ours? The bridged `s.cards` lags the store by one
  // turn, which is fine here: a post exists for many turns before its picture
  // is chosen. If the world has not caught up, the open page's own `post`
  // class answers — /posts marks it.
  isPost(id) {
    try {
      const list = JSON.parse(String(JSON.parse(feature_Loop.state || '{}').cards || '[]'));
      for (const c of list) {
        if (c && c.id === id) return c.type === 'post';
      }
    } catch (e) {
      /* fall through to the page */
    }
    return !!document.querySelector('.card-page.post');
  },
};

{
  // /cards' `chose`, taken by redefinition and kept in a closure — /frame and
  // /from-picture take the same one, and this link sits OUTSIDE both because
  // provenance puts this node later. So the file read here is the ORIGINAL the
  // user picked, before /frame's canvas throws the EXIF away. It reads, then
  // hands the file on untouched; nothing here knows anything about either.
  if (typeof feature_Cards !== 'undefined') {
    const fm_ptChose = feature_Cards.chose;
    feature_Cards.chose = async function (file) {
      const t = this.target;
      const id = (t && t.id) ? t.id : '';
      feature_PostTime.pending = null;
      if (file && id && feature_PostTime.isPost(id)) {
        const at = await feature_PostTime.taken(file);
        if (at) feature_PostTime.pending = { id: id, when: at };
      }
      return fm_ptChose.call(this, file);
    };
  }

  // the time travels on the back of the picture: /cards sends CardPic at the
  // end of both paths (framed and unframed), so a cancelled framing or a photo
  // refused for being too big never reaches here and the post keeps the time
  // it had. A load-time redefinition of a named function — NOT a timer
  // wrapping apply, which is the race in notes.md.
  if (typeof feature_Loop !== 'undefined') {
    const fm_ptSend = feature_Loop.send;
    feature_Loop.send = function (event) {
      const out = fm_ptSend.call(this, event);
      const p = feature_PostTime.pending;
      if (p && event && event.type === 'CardPic'
          && event.data && event.data.id === p.id) {
        feature_PostTime.pending = null;
        fm_ptSend.call(this, { type: 'CardWhen', data: {
          id: p.id, when: p.when, source: 'photo', t: Date.now() } });
      }
      return out;
    };
  }
}
