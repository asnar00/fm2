const feature_Review = {
  applying: false,

  running() {
    return typeof feature_Update !== 'undefined'
      ? parseInt(feature_Update.running, 10) || 0 : 0;
  },
  server() {
    return typeof feature_Update !== 'undefined'
      ? parseInt(feature_Update.server, 10) || 0 : 0;
  },

  // the version-stamp + cache-clear + reload ritual (the panel button's move)
  async apply(build) {
    if (this.applying) return;
    this.applying = true;
    try { localStorage.misoVersion = String(build); } catch (e) {}
    if (typeof feature_Update !== 'undefined') await feature_Update.evict();
    else { try { await caches.delete('miso'); } catch (e) {} }
    location.reload();
  },

  // seam: which changes.json entries earn a release line — every pending
  // build in the gap that no feature row already represents. A subfeature
  // may narrow the set (see /bookkeeping).
  releases(changes, running, server, covered) {
    return changes.filter((c) =>
      c.build > running && c.build <= server && !covered.has(c.build));
  },

  // seam: how many releases the header claims. Default: the build gap.
  count(running, server) {
    return server - running;
  },

  // the awaiting section: pending features from the server's LIVE tree —
  // any node whose build exceeds what's running here
  async section() {
    const running = this.running();
    const server = this.server();
    const box = $('changes');
    const old = document.getElementById('awaiting');
    if (old) old.remove();
    const upBtn = $('updateBtn');
    if (server <= running || !box || !box.classList.contains('chooser-home')) {
      return; // up to date (or list absent): the plain button's world
    }
    const tree = await fetch('features/tree.json', { cache: 'no-store' })
      .then((r) => r.ok ? r.json() : null).catch(() => null);
    if (!tree) return; // degrade honestly to the standing update button
    if (upBtn) upBtn.style.display = 'none';
    const pending = [];
    const walk = (ns, parent) => {
      for (const n of ns) {
        n.parent = parent;
        if (n.build > running) pending.push(n);
        walk(n.children, n.path);
      }
    };
    walk(tree, '');
    pending.sort((a, b) => b.build - a.build || (a.path < b.path ? -1 : 1));
    let rows = (typeof feature_Chooser !== 'undefined' && pending.length)
      ? pending.map((n) => { feature_Chooser.byPath[n.path] = n; return feature_Chooser.row(n); }).join('')
      : '';
    // a pending build no feature row represents (scaffolding, fixes outside
    // the tree) still shows its release line — an update never lists nothing
    const covered = new Set(pending.map((n) => n.build));
    const changes = await fetch('changes.json', { cache: 'no-store' })
      .then((r) => r.ok ? r.json() : []).catch(() => []);
    rows += this.releases(changes, running, server, covered)
      .map((c) => '<div class="crow"><span class="cnum">' + c.build + '</span>'
        + '<div class="ctext"><span class="cpurpose">'
        + String(c.text).replace(/&/g, '&amp;').replace(/</g, '&lt;')
        + '</span></div></div>').join('');
    const sect = document.createElement('div');
    sect.id = 'awaiting';
    const n = this.count(running, server);
    sect.innerHTML =
      '<div class="awhead">awaiting update — build ' + server
      + (n > 1 ? ' (' + n + ' releases)' : '') + '</div>'
      + rows
      + '<div class="awrow"><button id="acceptBtn">update</button></div>';
    box.prepend(sect);
    document.getElementById('acceptBtn').onclick = () => {
      feature_Loop.send({ type: 'AcceptUpdate', data: { build: server } });
      feature_Review.apply(server);
    };
    if (typeof feature_Chooser !== 'undefined') feature_Chooser.reflect();
  },

  // the one OK arriving over sync: an acceptance newer than what's running
  watch() {
    if (typeof feature_Replay !== 'undefined' && feature_Replay.active) return;
    let s = {};
    try { s = JSON.parse(feature_Loop.state || '{}'); } catch (e) {}
    const accepted = parseInt(s.update_accepted || '0', 10) || 0;
    const running = this.running();
    if (accepted > running && running > 0 && this.server() >= accepted) {
      this.apply(accepted);
    }
  },
};
{
  // ride the chooser's mount: the awaiting section tops the feature list
  if (typeof feature_Chooser !== 'undefined') {
    const fm_reviewMount = feature_Chooser.mount.bind(feature_Chooser);
    feature_Chooser.mount = async function () {
      await fm_reviewMount();
      await feature_Review.section();
    };
  }
  const fm_reviewApply = feature_Loop.apply;
  feature_Loop.apply = function (p) {
    fm_reviewApply.call(this, p);
    feature_Review.watch();
  };
}
