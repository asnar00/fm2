const feature_Honest = {
  // a failed check must never masquerade as "up to date"
  retry() {
    let tries = 0;
    const timer = setInterval(async () => {
      const got = typeof feature_Watch !== 'undefined'
        ? await feature_Watch.check() : null;
      if (got || ++tries >= 12) clearInterval(timer);
    }, 5000);
  },
  statusText(live) {
    return live ? ' — up to date' : ' — can’t reach the server';
  },
};
