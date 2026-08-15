const feature_Fresh = {
  // the caching principle: the cache serves only when the network can't
  // deliver — within /deadline's time budget when that feature is present,
  // otherwise only on outright failure.
  deadline: () => (typeof feature_Deadline !== 'undefined' ? feature_Deadline.ms : 0),
};
self.addEventListener('fetch', (e) => {
  if (e.request.method !== 'GET') return;
  // auth state and the deploy stamp must never be answered from cache
  const path = new URL(e.request.url).pathname;
  if (path.includes('/auth/') || path.endsWith('/version')) return;
  e.respondWith(caches.open(CACHE).then(async (cache) => {
    const hit = await cache.match(e.request);
    const net = fetch(e.request).then((res) => {
      if (res.ok) cache.put(e.request, res.clone());
      return res;
    });
    net.catch(() => {});
    if (!hit) return net;
    const ms = feature_Fresh.deadline();
    if (!ms) return net.catch(() => hit);
    return Promise.race([
      net.catch(() => hit),
      new Promise((resolve) => setTimeout(() => resolve(hit), ms)),
    ]);
  }));
});
