const feature_People = {
  asked: false,

  // how near everyone is, computed by the server because the invite tree
  // lives in the guest list and no device has it. It is loop STATE, not a
  // var: the list is the server's, and syncing it as world state would be a
  // lie (/invite's own reason for the same shape).
  async pull() {
    let d = null;
    try {
      const r = await fetch('users/near', { cache: 'no-store' });
      d = await r.json();
    } catch (e) {
      d = null;
    }
    const near = (d && d.ok && d.near) || {};
    if (typeof feature_Loop === 'undefined' || !feature_Loop.state) return;
    feature_Loop.send({ type: 'PeopleNear', data: near });
  },

  // the people surface appearing is what this node reacts to, read from the
  // DOM rather than by wrapping feature_Loop.apply — that idiom races and
  // orphans other fragments' wrappers (notes.md, "the apply-wrapper race").
  look() {
    const on = document.querySelector('.browse-grid, .browse-list, .browse-empty');
    if (!on) {
      this.asked = false;
      return;
    }
    if (!this.asked) {
      this.asked = true;
      this.pull();
    }
  },
};

{
  const fm_peopleWatch = setInterval(() => {
    const app = document.getElementById('app');
    if (!app) return;
    clearInterval(fm_peopleWatch);
    const look = () => feature_People.look();
    new MutationObserver(look).observe(app, { childList: true, subtree: true });
    look();
  }, 100);
}
