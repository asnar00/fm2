// muon service worker: stale-while-revalidate — serve from cache instantly
// (offline included), refresh the cache in the background so the next load
// picks up deployed changes without version bookkeeping.
const CACHE = 'muon';

self.addEventListener('install', e => self.skipWaiting());
self.addEventListener('activate', e => e.waitUntil(self.clients.claim()));

self.addEventListener('fetch', e => {
  if (e.request.method !== 'GET') return;
  // auth state and the deploy stamp must never be answered from cache
  const path = new URL(e.request.url).pathname;
  if (path.includes('/auth/') || path.endsWith('/version')) return;
  // the entry page is network-first: launches always see the latest deploy
  // when online (no one-launch-behind), cache only covers offline
  const entry = e.request.mode === 'navigate' || path === '/' || path.endsWith('/index.html');
  e.respondWith(caches.open(CACHE).then(async cache => {
    if (entry) {
      try {
        const res = await fetch(e.request);
        if (res.ok) cache.put(e.request, res.clone());
        return res;
      } catch (err) {
        const hit = await cache.match(e.request);
        if (hit) return hit;
        throw err;
      }
    }
    const hit = await cache.match(e.request);
    const net = fetch(e.request).then(res => {
      if (res.ok) cache.put(e.request, res.clone());
      return res;
    }).catch(() => hit);
    return hit || net;
  }));
});
