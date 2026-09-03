const feature_Viewer = {
  sheet: null, frame: null, pdf: '', title: '',

  make() {
    if (this.sheet) return;
    const s = document.createElement('div');
    s.id = 'repView';
    s.innerHTML = '<div class="repview-bar"><div class="repview-back" role="button">‹</div>'
      + '<div class="repview-title"></div><div class="repview-share" role="button">export PDF</div></div>'
      + '<iframe class="repview-frame" title="report"></iframe>';
    document.body.appendChild(s);
    this.sheet = s;
    this.frame = s.querySelector('.repview-frame');
    s.querySelector('.repview-back').addEventListener('click', () => this.close());
    s.querySelector('.repview-share').addEventListener('click', () => this.share());
  },

  // the id from the link's own href: reports/<slug>.pdf?id=<id>
  idOf(href) {
    const m = /[?&]id=([^&]+)/.exec(href || '');
    return m ? m[1] : '';
  },

  async open(link) {
    const href = link.getAttribute('href') || '';
    const id = this.idOf(href);
    if (!id) { window.open(href, '_blank'); return; }
    // a report printed before this node has no kept page: the old way. The
    // page is fetched here (GET — the routes answer GET) and put into the
    // frame as a document of its own, so the frame never asks the server twice
    let html = '';
    try {
      const r = await fetch('reports/view?id=' + id, { cache: 'no-store' });
      if (r.ok) html = await r.text();
    } catch (e) { html = ''; }
    if (!html) { window.open(href, '_blank'); return; }
    this.make();
    this.pdf = href;
    const card = link.closest('.card-page, .rep-card, .crow');
    const t = card ? card.querySelector('.card-title, .rep-title, .browse-title') : null;
    this.title = t ? t.textContent.trim() : 'report';
    this.sheet.querySelector('.repview-title').textContent = this.title;
    this.frame.removeAttribute('src');
    this.frame.srcdoc = html;
    this.sheet.classList.add('show');
  },

  close() {
    if (!this.sheet) return;
    this.sheet.classList.remove('show');
    this.frame.srcdoc = '';
  },

  // the PDF to the phone's share sheet as a file; a browser that cannot
  // share files gets the PDF in a new tab, the old way
  async share() {
    try {
      const r = await fetch(this.pdf, { cache: 'no-store' });
      if (!r.ok) throw new Error('no pdf');
      const blob = await r.blob();
      const name = (this.title || 'report').replace(/[^\w\- ]+/g, '').trim() || 'report';
      const file = new File([blob], name + '.pdf', { type: 'application/pdf' });
      if (navigator.canShare && navigator.canShare({ files: [file] })) {
        await navigator.share({ files: [file], title: this.title });
        return;
      }
    } catch (e) {
      /* below */
    }
    window.open(this.pdf, '_blank');
  },
};

{
  // capture, ahead of the link's own navigation: the tap is the sheet's
  document.addEventListener('click', (e) => {
    if (!e.target || !e.target.closest) return;
    const link = e.target.closest('a.rep-doc');
    if (!link) return;
    e.preventDefault();
    e.stopPropagation();
    feature_Viewer.open(link);
  }, true);
}
