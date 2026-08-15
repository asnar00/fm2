const feature_LessBusy = {
  who() {
    const w = typeof feature_Panel !== 'undefined' ? feature_Panel.lastWho : null;
    return w && w.name ? 'logged in as ' + w.name : 'not logged in';
  },
  status() {
    const up = typeof feature_Update !== 'undefined' ? feature_Update : null;
    if (!up) return '';
    if (up.newer()) return 'build ' + up.running + ' → ' + up.server + ' available';
    const suffix = typeof feature_Honest !== 'undefined'
      ? feature_Honest.statusText(up.server)
      : (up.server ? ' — up to date' : '');
    return 'build ' + up.running + suffix;
  },
  refresh() {
    const who = $('who'), line = $('buildLine');
    if (who) who.textContent = this.who();
    if (line) line.textContent = this.status();
  },
};
{
  const fm_panel = $('panel');
  if (fm_panel) {
    const fm_line = document.createElement('div');
    fm_line.id = 'buildLine';
    const fm_whoRow = document.createElement('div');
    fm_whoRow.className = 'row';
    fm_whoRow.id = 'whoRow';
    if ($('who')) fm_whoRow.appendChild($('who'));
    if ($('logoutBtn')) fm_whoRow.appendChild($('logoutBtn'));
    // the spec's order, each piece optional: ask, build line, updates,
    // policy, (features-button slots itself after policy), visitors, who
    const fm_updateRow = $('updateBtn') ? $('updateBtn').closest('.row') : null;
    for (const el of [$('askRow'), fm_line, $('changes'), fm_updateRow,
                      $('policySeg'), $('passkeyRow'), $('pushRow'), fm_whoRow]) {
      if (el) fm_panel.appendChild(el);
    }
    if (typeof feature_Panel !== 'undefined') {
      const fm_lessBusyOpen = feature_Panel.open.bind(feature_Panel);
      feature_Panel.open = async function () {
        await fm_lessBusyOpen();
        feature_LessBusy.refresh(); // the who-line's build freight moves here
      };
    }
  }
}
