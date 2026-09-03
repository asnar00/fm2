const feature_PicBeside = {
  // a picture block's `data` is `pic/<24 hex>` instead of bytes. The prefix is
  // in one place because three things test for it: the mint that must not
  // convert twice, the resolver that swaps in a local copy, and the error
  // handler that hides one that never arrived.
  PREFIX: 'pic/',
  DB: 'miso-pics',
  STORE: 'pics',
  RETRY: 4000,

  db: null,
  blobs: {},        // id -> Blob, every picture this device holds
  urls: {},         // id -> object URL, made on first need
  queue: [],        // ids waiting to go up
  sending: false,
  tried: {},        // ids the error handler has already given a second chance

  // ---- the one conversion --------------------------------------------------
  // synchronous on purpose: shrink's callers measure what comes back against
  // the list budget and send it in the same turn, so the reference has to
  // exist before this returns. The disk write and the upload are what happen
  // after.
  mint(dataUrl) {
    if (typeof dataUrl !== 'string' || dataUrl.slice(0, 11) !== 'data:image/') {
      return dataUrl;
    }
    let blob = null;
    try {
      blob = this.toBlob(dataUrl);
    } catch (e) {
      return dataUrl;         // undecodable: leave it inline, as it was before
    }
    if (!blob || !blob.size) return dataUrl;
    const id = this.id();
    this.blobs[id] = blob;
    this.put(id, blob);
    this.queue.push(id);
    this.drain();
    return this.PREFIX + id;
  },

  // 96 bits from the platform's own source. Unguessable is the whole of the
  // read rule's second half: an id nobody can guess is an id nobody can ask
  // for without holding the card that names it.
  id() {
    const b = new Uint8Array(12);
    (self.crypto || window.crypto).getRandomValues(b);
    let s = '';
    for (const x of b) s += x.toString(16).padStart(2, '0');
    return s;
  },

  toBlob(dataUrl) {
    const at = dataUrl.indexOf(';base64,');
    if (at < 0) return null;
    const type = dataUrl.slice(5, at);
    const bin = atob(dataUrl.slice(at + 8));
    const out = new Uint8Array(bin.length);
    for (let i = 0; i < bin.length; i++) out[i] = bin.charCodeAt(i);
    return new Blob([out], { type: type });
  },

  // ---- the local copy ------------------------------------------------------
  // every picture this device made, kept so it shows before and without any
  // upload, and so it still shows offline. Nothing is ever evicted: the one
  // record eviction would reach for is the one not yet uploaded, which is the
  // record the store exists to protect.
  open() {
    return new Promise((res, rej) => {
      const rq = indexedDB.open(this.DB, 1);
      rq.onupgradeneeded = () => rq.result.createObjectStore(this.STORE);
      rq.onsuccess = () => res(rq.result);
      rq.onerror = () => rej(rq.error);
    });
  },

  put(id, blob) {
    if (!this.db) return;
    try {
      const tx = this.db.transaction(this.STORE, 'readwrite');
      tx.objectStore(this.STORE).put({ blob: blob, up: false, at: Date.now() }, id);
    } catch (e) { /* a device that cannot write still has this session's copy */ }
  },

  mark(id) {
    if (!this.db || !this.blobs[id]) return;
    try {
      const tx = this.db.transaction(this.STORE, 'readwrite');
      tx.objectStore(this.STORE).put(
        { blob: this.blobs[id], up: true, at: Date.now() }, id);
    } catch (e) {}
  },

  // everything at once at boot: the records are a few kilobytes each and
  // `resolve` has to be able to answer synchronously.
  async load() {
    try {
      this.db = await this.open();
    } catch (e) {
      return;                 // no IndexedDB: this session's copies still serve
    }
    const rows = await new Promise((res) => {
      try {
        const tx = this.db.transaction(this.STORE, 'readonly');
        const st = tx.objectStore(this.STORE);
        const ks = st.getAllKeys();
        const vs = st.getAll();
        tx.oncomplete = () => res([ks.result || [], vs.result || []]);
        tx.onerror = () => res([[], []]);
      } catch (e) { res([[], []]); }
    });
    const keys = rows[0], vals = rows[1];
    for (let i = 0; i < keys.length; i++) {
      const v = vals[i];
      if (!v || !v.blob) continue;
      this.blobs[keys[i]] = v.blob;
      if (!v.up && this.queue.indexOf(keys[i]) < 0) this.queue.push(keys[i]);
    }
  },

  // ---- the picture queue ---------------------------------------------------
  // its own road, not /messaging's outbox: the outbox carries JSON ops through
  // POST /msg and bytes do not fit that road. One at a time, and an id leaves
  // only when the server has said ok.
  async drain() {
    if (this.sending || !this.queue.length) return;
    this.sending = true;
    try {
      while (this.queue.length) {
        const id = this.queue[0];
        const blob = this.blobs[id];
        if (!blob) { this.queue.shift(); continue; }
        let ok = false;
        try {
          const r = await fetch(this.PREFIX + id,
                                { method: 'POST', body: blob, cache: 'no-store' });
          ok = r.ok;
        } catch (e) { ok = false; }
        if (!ok) break;          // offline, or refused: keep it and try later
        this.queue.shift();
        this.mark(id);
      }
    } finally {
      this.sending = false;
    }
  },

  // ---- showing it ----------------------------------------------------------
  // the reference is a URL the server answers, so every consumer in the tree
  // draws it with no change at all. This is the one thing the server cannot
  // do: put the device's OWN copy up, in the frame the picture was taken, with
  // no network and before any upload.
  // the id inside a reference, whatever is hung off the end of it: the retry
  // below re-asks with a query, and that must still name the same picture or
  // the retry is a second first attempt and retries forever.
  idOf(ref) {
    if (typeof ref !== 'string' || ref.slice(0, this.PREFIX.length) !== this.PREFIX) {
      return '';
    }
    return ref.slice(this.PREFIX.length).split('?')[0];
  },

  urlFor(id) {
    if (this.urls[id]) return this.urls[id];
    const blob = this.blobs[id];
    if (!blob) return '';
    this.urls[id] = URL.createObjectURL(blob);
    return this.urls[id];
  },

  resolve(root) {
    if (!root || !root.querySelectorAll) return;
    const sel = 'img[src^="' + this.PREFIX + '"]';
    const imgs = root.querySelectorAll(sel);
    for (const img of imgs) this.swap(img);
    if (root.matches && root.matches(sel)) this.swap(root);
  },

  swap(img) {
    const url = this.urlFor(this.idOf(img.getAttribute('src') || ''));
    if (url) img.setAttribute('src', url);
  },

  // a reference whose bytes have not arrived — a recipient looking at a copy
  // in the seconds between the owner's op landing and the owner's upload
  // finishing. Hidden rather than left as a broken icon, and given ONE more
  // chance: a picture that is genuinely gone must stop asking.
  missed(img) {
    const ref = img.getAttribute('src') || '';
    const id = this.idOf(ref);
    if (!id) return;
    img.classList.add('pic-away');
    // the bytes may have arrived on this device since the fetch failed
    if (this.urlFor(id)) {
      img.classList.remove('pic-away');
      img.setAttribute('src', this.urls[id]);
      return;
    }
    if (this.tried[id]) return;
    this.tried[id] = true;
    setTimeout(() => {
      if (!img.isConnected) return;
      img.classList.remove('pic-away');
      img.setAttribute('src', this.PREFIX + id + '?again=1');
    }, this.RETRY);
  },

  watch() {
    const self = this;
    const obs = new MutationObserver((records) => {
      for (const r of records) {
        if (r.type === 'attributes') { self.resolve(r.target); continue; }
        for (const n of r.addedNodes) {
          if (n.nodeType === 1) self.resolve(n);
        }
      }
    });
    obs.observe(document.documentElement, {
      childList: true, subtree: true,
      attributes: true, attributeFilter: ['src'],
    });
    document.addEventListener('error', (e) => {
      const el = e.target;
      if (el && el.tagName === 'IMG') self.missed(el);
    }, true);
  },

  async start() {
    this.watch();
    await this.load();
    this.resolve(document);        // whatever was painted while we were reading
    this.drain();
    window.addEventListener('online', () => this.drain());
  },
};

{
  // the two roads a picture is made on, taken by redefinition — /frame takes
  // /cards' `chose` exactly this way, and the linker's fragment gate wraps
  // these assignments so unticking the node hands both back at runtime.
  //
  // Converting HERE and not at the send is what keeps the three list-budget
  // gates (in /cards, /photo and /poster) true: each measures what comes back
  // from these, which is now a 28-character reference.
  if (typeof feature_Cards !== 'undefined' && feature_Cards.shrink) {
    const fm_pbShrink = feature_Cards.shrink;
    feature_Cards.shrink = async function (file) {
      const data = await fm_pbShrink.call(this, file);
      return data ? feature_PicBeside.mint(data) : data;
    };
  }

  if (typeof feature_Poster !== 'undefined' && feature_Poster.draw) {
    const fm_pbDraw = feature_Poster.draw;
    feature_Poster.draw = function (v) {
      const data = fm_pbDraw.call(this, v);
      return data ? feature_PicBeside.mint(data) : data;
    };
  }

  // the net beneath both: a road written after this node that reaches an event
  // carrying inline bytes has them converted here instead. /post-time and
  // /from-picture wrap send too and read only `type` and `data.id`, so the
  // rewrite is invisible to them whichever order the wraps end up in.
  if (typeof feature_Loop !== 'undefined') {
    const fm_pbSend = feature_Loop.send;
    feature_Loop.send = function (event) {
      if (event && event.data && typeof event.data.data === 'string'
          && event.data.data.slice(0, 11) === 'data:image/') {
        event.data.data = feature_PicBeside.mint(event.data.data);
      }
      return fm_pbSend.call(this, event);
    };
  }

  feature_PicBeside.start();
}
