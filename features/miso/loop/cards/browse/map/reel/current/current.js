{
  if (typeof feature_Reel !== 'undefined') {
    feature_Reel.mark = function () {
      const cur = this.current();
      for (const el of this.list.querySelectorAll('.reel-post'))
        el.classList.toggle('reel-current', el === cur);
    };
    const fm_curFollow = feature_Reel.follow;
    feature_Reel.follow = function () {
      fm_curFollow.call(this);
      try { this.mark(); } catch (e) { /* no list yet */ }
    };
    const fm_curRender = feature_Reel.render;
    feature_Reel.render = function () {
      fm_curRender.call(this);
      try { if (this.list) this.mark(); } catch (e) { /* no list yet */ }
    };
  }
}
