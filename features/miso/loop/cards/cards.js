const feature_Cards = {
  // a picture block is inlined in the card, and the whole cards list travels
  // as ONE op through /msg, whose body cap is 16384 bytes — over that the
  // message is truncated, rejected as untyped, and retried forever, jamming
  // the outbox. So the budget is the wire's, not the disk's: 8KB for one
  // picture's data URL, and 14KB for the whole list once it is added. Over
  // either, the picture is refused out loud rather than quietly stored.
  CAP: 8192,
  LIST_CAP: 14000,
  input: null,
  toast: null,
  target: null,

  say(words) {
    if (!this.toast) return;
    this.toast.textContent = words;
    this.toast.classList.add('show');
    clearTimeout(this.toastTimer);
    this.toastTimer = setTimeout(() => this.toast.classList.remove('show'), 3200);
  },

  // how many characters this world's cards take on the wire, NOT counting
  // the block about to be overwritten — replacing a picture must not be
  // charged for the one it replaces, or a second picture is never allowed.
  // The bridged `s.cards` lags one turn behind the store, which is fine for
  // a budget: it is never stale by more than one edit.
  held(id, at) {
    try {
      const raw = String(JSON.parse(feature_Loop.state || '{}').cards || '[]');
      const list = JSON.parse(raw);
      let old = 0;
      for (const c of list) {
        if (c && c.id === id && c.blocks && c.blocks[at])
          old = String(c.blocks[at].data || '').length;
      }
      return raw.length - old;
    } catch (e) {
      return 0;
    }
  },

  // longest edge 256px, JPEG, quality stepping down until it fits. null means
  // "this cannot be brought under the cap" — the caller says so to the user.
  async shrink(file) {
    const url = URL.createObjectURL(file);
    try {
      const img = await new Promise((res, rej) => {
        const im = new Image();
        im.onload = () => res(im);
        im.onerror = () => rej(new Error('not an image'));
        im.src = url;
      });
      const long = Math.max(img.width, img.height) || 1;
      const scale = Math.min(1, 256 / long);
      const cv = document.createElement('canvas');
      cv.width = Math.max(1, Math.round(img.width * scale));
      cv.height = Math.max(1, Math.round(img.height * scale));
      cv.getContext('2d').drawImage(img, 0, 0, cv.width, cv.height);
      for (const q of [0.8, 0.65, 0.5, 0.4, 0.3, 0.2]) {
        const d = cv.toDataURL('image/jpeg', q);
        if (d.length <= this.CAP) return d;
      }
      return null;
    } finally {
      URL.revokeObjectURL(url);
    }
  },

  async chose(file) {
    const t = this.target;
    if (!file || !t) return;
    let data = null;
    try {
      data = await this.shrink(file);
    } catch (e) {
      this.say('that file is not a picture');
      return;
    }
    if (!data) {
      this.say('that picture is too big to keep');
      return;
    }
    if (this.held(t.id, t.i) + data.length > this.LIST_CAP) {
      this.say('no room for that picture — your cards are full');
      return;
    }
    feature_Loop.send({ type: 'CardPic',
      data: { id: t.id, i: t.i, data, t: Date.now() } });
  },
};

{
  // furniture this node owns, made at load and living OUTSIDE #app so a
  // repaint of the loop's html cannot take it away
  const fm_cardsIn = document.createElement('input');
  fm_cardsIn.type = 'file';
  fm_cardsIn.accept = 'image/*';
  fm_cardsIn.id = 'cardPicInput';
  fm_cardsIn.style.display = 'none';
  fm_cardsIn.addEventListener('change', () => {
    const f = fm_cardsIn.files && fm_cardsIn.files[0];
    feature_Cards.chose(f);
  });
  document.body.appendChild(fm_cardsIn);
  feature_Cards.input = fm_cardsIn;

  const fm_cardsToast = document.createElement('div');
  fm_cardsToast.id = 'cardToast';
  document.body.appendChild(fm_cardsToast);
  feature_Cards.toast = fm_cardsToast;

  // tap away to keep: the block's text reaches the store when focus leaves it.
  // focusout (not blur) so one delegated listener survives every repaint.
  document.addEventListener('focusout', (e) => {
    const el = e.target;
    if (!el || !el.getAttribute) return;
    if (el.getAttribute('contenteditable') !== 'true') return;
    const at = el.getAttribute('data-block');
    if (at === null) return;
    if (typeof feature_Loop === 'undefined' || !feature_Loop.state) return;
    feature_Loop.send({ type: 'CardEdit', data: {
      id: el.getAttribute('data-card'),
      i: Number(at),
      text: (el.innerText || '').trim(),
      t: Date.now() } });
  });

  // the picture block opens the file chooser. It carries no data-ev, so the
  // loop's own delegated click never fires for it.
  document.addEventListener('click', (e) => {
    if (!e.target || !e.target.closest) return;
    const pic = e.target.closest('.card-pic');
    if (!pic) return;
    feature_Cards.target = { id: pic.getAttribute('data-card'),
                             i: Number(pic.getAttribute('data-block')) };
    feature_Cards.input.value = '';
    feature_Cards.input.click();
  });

  // the tile renderer's dev mount: the URL says on or off, every load, so a
  // device that asked for it once is not stuck with it forever
  {
    const fm_cardsWant = /(^|[?&])cardtiles=1/.test(location.search);
    const fm_cardsTiles = setInterval(() => {
      if (typeof feature_Loop === 'undefined' || !feature_Loop.state) return;
      clearInterval(fm_cardsTiles);
      feature_Loop.send({ type: 'CardTiles', data: { on: fm_cardsWant } });
    }, 100);
  }
}
