const feature_Engineer = {
  open: false,
  // the extensible function: what the engineer section holds. The default
  // holds the first tenant — /self-check's report, when that node is here
  // (it predates the gear, so the gear reaches for it). A later tenant
  // replaces this property AT LOAD, calling the one it captured first and
  // then appending its own block. Load-time replacement cannot race.
  fill(box) {
    if (typeof feature_SelfCheck !== 'undefined') {
      const block = document.createElement('div');
      block.id = 'selfCheck';
      block.textContent = feature_SelfCheck.text();
      box.appendChild(block);
    }
  },
  toggle() { this.open = !this.open; this.render(); },
  refresh() { if (this.open) this.render(); },
  render() {
    const box = $('engineer');
    if (!box) return;
    box.style.display = this.open ? '' : 'none';
    const btn = $('engineerBtn');
    if (btn) btn.classList.toggle('on', this.open);
    if (!this.open) return;
    box.innerHTML = '';
    try { this.fill(box); } catch (e) {
      const err = document.createElement('div');
      err.textContent = 'engineer section: ' + (e && e.message ? e.message : String(e));
      box.appendChild(err);
    }
    if (!box.childNodes.length) {
      const empty = document.createElement('div');
      empty.className = 'eng-empty';
      empty.textContent = 'nothing here yet';
      box.appendChild(empty);
    }
  },
};
{
  const fm_engPanel = $('panel');
  if (fm_engPanel) {
    const fm_gear = document.createElement('button');
    fm_gear.id = 'engineerBtn';
    fm_gear.title = 'engineer';
    fm_gear.setAttribute('aria-label', 'engineer');
    // a drawn gear in currentColor (never an emoji-presentation character)
    fm_gear.innerHTML = '<svg class="eng-gear" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">'
      + '<circle cx="12" cy="12" r="3.2"/>'
      + '<path d="M12 2.8v2.6M12 18.6v2.6M2.8 12h2.6M18.6 12h2.6M5.5 5.5l1.9 1.9M16.6 16.6l1.9 1.9M5.5 18.5l1.9-1.9M16.6 7.4l1.9-1.9"/>'
      + '</svg>';
    fm_gear.onclick = () => feature_Engineer.toggle();
    let fm_engRow = $('buildRow');
    if (!fm_engRow) {
      fm_engRow = document.createElement('div');
      fm_engRow.className = 'row';
      fm_engRow.id = 'engineerRow';
      fm_engPanel.appendChild(fm_engRow);
    }
    fm_engRow.appendChild(fm_gear);
    const fm_engBox = document.createElement('div');
    fm_engBox.id = 'engineer';
    fm_engBox.style.display = 'none';
    fm_engRow.after(fm_engBox);
    // a finished self-check redraws an open section
    if (typeof feature_SelfCheck !== 'undefined') {
      const fm_engRun = feature_SelfCheck.run.bind(feature_SelfCheck);
      feature_SelfCheck.run = function () {
        const p = fm_engRun();
        Promise.resolve(p).then(() => feature_Engineer.refresh()).catch(() => {});
        return p;
      };
    }
    // every open of the sheet starts folded
    if (typeof feature_Panel !== 'undefined') {
      const fm_engOpen = feature_Panel.open.bind(feature_Panel);
      feature_Panel.open = async function () {
        await fm_engOpen();
        feature_Engineer.open = false;
        feature_Engineer.render();
      };
    }
  }
}
