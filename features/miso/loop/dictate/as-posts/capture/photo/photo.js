const feature_Photo = {
  input: null,

  // the picture goes in through /cards' own doors — shrink, the budget, the
  // two ops — so the only new thing here is that they run in one act instead
  // of four. Every step guarded: with /cards or /posts absent, nothing
  // happens rather than something half-happening.
  async make(file) {
    if (!file) return;
    if (typeof feature_Loop === 'undefined' || !feature_Loop.state) return;
    if (typeof feature_Cards === 'undefined') return;
    let data = null;
    try {
      data = await feature_Cards.shrink(file);
    } catch (e) {
      feature_Cards.say('that file is not a picture');
      return;
    }
    if (!data) {
      feature_Cards.say('that picture is too big to keep');
      return;
    }
    // the budget, before the post exists: a post made and then refused its
    // picture is worse than no post. '' names no card, so nothing is
    // discounted — which is right, there is nothing to replace.
    if (feature_Cards.held('', 1) + data.length > feature_Cards.LIST_CAP) {
      feature_Cards.say('no room for that picture — your cards are full');
      return;
    }
    const owner = (typeof feature_Posts !== 'undefined' && feature_Posts.name)
      ? await feature_Posts.name() : '';
    const t = Date.now();
    // a card you make opens ready to write — /editing's own rule, which it
    // arms from a click on the buttons that make one. The camera is one of
    // those buttons, and it is this half that makes the card, so this half
    // says so. Without it a photo post would open locked and the words would
    // be a pencil-tap away, which is not "photo+type".
    if (typeof feature_Editing !== 'undefined') feature_Editing.openNext = true;
    // /new mints <owner>.<t>, and an empty owner becomes "you" there — so the
    // id is known here and the picture's op can name the card in the same
    // turn. send() is synchronous through the wasm, so these apply in order.
    feature_Loop.send({ type: 'CardNew',
      data: { owner, type: 'post', title: '', t } });
    feature_Loop.send({ type: 'CardPic',
      data: { id: (owner || 'you') + '.' + t, i: 1, data, t: Date.now() } });
    if (typeof feature_Posts !== 'undefined' && feature_Posts.caret) {
      feature_Posts.caret();
      setTimeout(() => feature_Posts.settle(), 400);
    }
  },
};

{
  // furniture made at load and living OUTSIDE #app, so a repaint of the
  // loop's html cannot take it away (/cards' pattern). No `capture`
  // attribute: the phone's own menu — take one, or pick one — is the
  // platform default and is what was asked for.
  const fm_photoIn = document.createElement('input');
  fm_photoIn.type = 'file';
  fm_photoIn.accept = 'image/*';
  fm_photoIn.id = 'capturePhotoInput';
  fm_photoIn.style.display = 'none';
  fm_photoIn.addEventListener('change', () => {
    const f = fm_photoIn.files && fm_photoIn.files[0];
    feature_Photo.make(f);
  });
  document.body.appendChild(fm_photoIn);
  feature_Photo.input = fm_photoIn;

  // taken in the CAPTURE phase so /loop's delegated click never sends it on:
  // making the post is this half's act, and a click that did both would open
  // the chooser and waste a round trip (/posts' own reasoning for the plus).
  document.addEventListener('click', (e) => {
    if (!e.target || !e.target.closest) return;
    if (!e.target.closest('[data-ev="capture_photo"]')) return;
    e.stopPropagation();
    e.preventDefault();
    feature_Photo.input.value = '';
    feature_Photo.input.click();
  }, true);
}
