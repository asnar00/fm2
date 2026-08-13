// muon service worker. basic principle: the cache is only used when the
// network is unreachable — online always means current. every successful
// fetch refreshes the offline copy.
const CACHE = 'muon';

self.addEventListener('install', e => self.skipWaiting());
self.addEventListener('activate', e => e.waitUntil(self.clients.claim()));

self.addEventListener('fetch', e => {
  if (e.request.method !== 'GET') return;
  // auth state and the deploy stamp must never be answered from cache
  const path = new URL(e.request.url).pathname;
  if (path.includes('/auth/') || path.endsWith('/version')) return;
  e.respondWith(caches.open(CACHE).then(async cache => {
    try {
      const res = await fetch(e.request);
      if (res.ok) cache.put(e.request, res.clone());
      return res;
    } catch (err) {
      const hit = await cache.match(e.request);
      if (hit) return hit;
      throw err;
    }
  }));
});
