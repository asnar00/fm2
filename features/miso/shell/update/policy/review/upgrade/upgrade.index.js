const feature_Upgrade = {
  added: null,  // node paths the pending builds introduce (null = not known yet)
  chosen: {},   // the user's in-review choices, local until upgrade is pressed

  async load() {
    if (this.added) return;
    const running = typeof feature_Review !== 'undefined' ? feature_Review.running() : 0;
    const changes = await fetch('changes.json', { cache: 'no-store' })
      .then((r) => r.ok ? r.json() : []).catch(() => []);
    this.added = new Set();
    for (const c of changes) {
      if (c.build > running)
        for (const p of (c.added || [])) this.added.add(p);
    }
  },

  // an addition's shown tick: the user's in-review choice, else their stored
  // explicit choice, else the policy default (automatic pre-ticks)
  shown(path) {
    if (path in this.chosen) return this.chosen[path];
    const stored = typeof feature_Chooser !== 'undefined' ? feature_Chooser.ticks() : {};
    if (path in stored) return stored[path] !== false;
    return typeof feature_Policy !== 'undefined'
      ? feature_Policy.current() === 'auto' : true;
  },

  // the review's dressing: badge the additions, take their ticks out of the
  // live-toggle loop (a review is a draft until upgrade commits it), show
  // the policy-defaulted state, make the button say upgrade
  dress() {
    const sect = document.getElementById('awaiting');
    if (!sect || !this.added) return;
    for (const row of sect.querySelectorAll('.crow[data-path]')) {
      const path = row.getAttribute('data-path');
      if (!this.added.has(path)) continue;
      const text = row.querySelector('.ctext');
      if (text && !text.querySelector('.pnew')) {
        const b = document.createElement('span');
        b.className = 'pnew';
        b.textContent = 'new';
        text.appendChild(b);
      }
      const tick = row.querySelector('.ctick');
      if (!tick.dataset.upath) {
        tick.dataset.upath = path;
        tick.removeAttribute('data-ev'); // the loop's toggle stands down here
      }
      tick.classList.toggle('on', this.shown(path));
    }
    if (!sect.dataset.upWired) {
      sect.dataset.upWired = '1';
      sect.addEventListener('click', (e) => {
        const tick = e.target.closest('.ctick[data-upath]');
        if (!tick) return;
        const path = tick.dataset.upath;
        feature_Upgrade.chosen[path] = !feature_Upgrade.shown(path);
        feature_Upgrade.dress();
      });
    }
    const btn = document.getElementById('acceptBtn');
    if (btn && !btn.dataset.upgrade) {
      btn.dataset.upgrade = '1';
      btn.textContent = 'upgrade';
      const fm_accept = btn.onclick;
      btn.onclick = () => { feature_Upgrade.stamp(); fm_accept(); };
    }
  },

  // commit the review: any addition whose shown tick differs from the stored
  // effective state gets the tick's own event — durably queued through the
  // outbox, so the stamp survives the apply-and-reload
  stamp() {
    const sect = document.getElementById('awaiting');
    if (!sect || !this.added) return;
    const stored = typeof feature_Chooser !== 'undefined' ? feature_Chooser.ticks() : {};
    for (const row of sect.querySelectorAll('.crow[data-path]')) {
      const path = row.getAttribute('data-path');
      if (!this.added.has(path)) continue;
      const storedOn = stored[path] !== false;
      if (this.shown(path) !== storedOn)
        feature_Loop.send({ type: 'click', ev: 'ftick_' + path });
    }
  },
};
{
  if (typeof feature_Review !== 'undefined') {
    const fm_upgradeSection = feature_Review.section.bind(feature_Review);
    feature_Review.section = async function () {
      await fm_upgradeSection();
      feature_Upgrade.added = null; // the pending set may have changed
      await feature_Upgrade.load();
      feature_Upgrade.dress();
    };
  }
  // re-dress after every state change: /chooser's reflect re-asserts stored
  // ticks on apply, and this wrap (composing later) restores the draft view
  const fm_upgradeApply = feature_Loop.apply;
  feature_Loop.apply = function (p) {
    fm_upgradeApply.call(this, p);
    feature_Upgrade.dress();
  };
}
