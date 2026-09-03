{
  if (typeof feature_Map !== 'undefined') {
    const fm_overSync = feature_Map.sync;
    feature_Map.sync = function () {
      fm_overSync.call(this);
      try {
        const page = document.querySelector('.card-page');
        const mapView = document.querySelector('.browse-view.browse-on[data-ev="browse_map"]');
        const behind = !document.getElementById('mapData') && !!page && !!mapView
          && !!document.querySelector('.toolbar .tool-button.sel[data-ev="tool_posts"]');
        document.body.classList.toggle('fm-map-behind', behind);
        if (behind) {
          this.show();
          // the map is the ground now, and /backdrop leaves the map alone: a
          // plain tap on it (Leaflet's click, never a drag) puts the page away
          // the way /backdrop does, by the tool's own button
          if (this.map && !this.map.fm_overTap) {
            this.map.fm_overTap = true;
            this.map.on('click', () => {
              if (!document.body.classList.contains('fm-map-behind')) return;
              if (typeof feature_Loop === 'undefined' || !feature_Loop.state) return;
              let open = '';
              try { open = JSON.parse(feature_Loop.state).open_tool || ''; } catch (err) {}
              feature_Loop.send({ type: 'click', ev: 'tool_' + (open || 'posts') });
            });
          }
          if (typeof feature_Reel !== 'undefined' && feature_Reel.host) feature_Reel.host.style.display = 'none';
        }
      } catch (e) { /* the map is as /map left it */ }
    };
  }
}
