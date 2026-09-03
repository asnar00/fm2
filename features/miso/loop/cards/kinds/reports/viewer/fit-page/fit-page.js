{
  if (typeof feature_Viewer !== 'undefined') {
    // the paper's margins on screen, then the whole document zoomed to the
    // sheet's width — down only, never up
    feature_Viewer.fit = function () {
      const f = this.frame;
      const doc = f && f.contentDocument;
      if (!doc || !doc.documentElement || !doc.body) return;
      if (!doc.getElementById('fm-fit')) {
        const st = doc.createElement('style');
        st.id = 'fm-fit';
        st.textContent = 'body { padding: 16mm 15mm 14mm 15mm !important; }';
        doc.head.appendChild(st);
      }
      doc.documentElement.style.zoom = '1';
      const natural = Math.max(doc.documentElement.scrollWidth, doc.body.scrollWidth);
      const sheet = this.sheet ? this.sheet.clientWidth : window.innerWidth;
      const z = natural > sheet ? sheet / natural : 1;
      doc.documentElement.style.zoom = String(z);
    };
    const fm_fitOpen = feature_Viewer.open;
    feature_Viewer.open = async function (link) {
      await fm_fitOpen.call(this, link);
      if (!this.frame) return;
      const self = this;
      const go = () => { try { self.fit(); } catch (e) { /* the page as it is */ } };
      this.frame.addEventListener('load', go, { once: true });
      setTimeout(go, 600);
    };
    window.addEventListener('resize', () => {
      if (feature_Viewer.sheet && feature_Viewer.sheet.classList.contains('show')) {
        try { feature_Viewer.fit(); } catch (e) { /* as it is */ }
      }
    });
  }
}
