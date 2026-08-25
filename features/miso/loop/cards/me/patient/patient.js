// the ensure waits for a REAL join. /veil marks fm-joined on its timeout
// too, and an ensure sent then reads an empty world and makes a card that
// last-write would send over the real one — the loss of build 292. A card
// can be made later; a card cannot be un-lost. So: only feature_Veil.joined
// counts, the wait is a minute, and if the join never comes the ensure
// simply does not happen this time.
if (typeof feature_Me !== 'undefined') {
  feature_Me.ready = function () {
    if (typeof feature_Veil === 'undefined') return true;   // no veil, no join to wait for
    return !!feature_Veil.joined;
  };
  feature_Me.ensure = async function () {
    for (let i = 0; i < 600 && !this.ready(); i++)
      await new Promise((r) => setTimeout(r, 100));
    if (!this.ready()) return;   // not joined: no card this time, nothing lost
    const owner = await this.name();
    feature_Loop.send({ type: 'CardEnsure',
      data: { owner, type: 'profile', t: Date.now() } });
  };
}
