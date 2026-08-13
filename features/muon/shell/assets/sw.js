// muon service worker: stale-while-revalidate — serve from cache instantly
// (offline included), refresh the cache in the background so the next load
// picks up deployed changes without version bookkeeping.
const CACHE = 'muon';

self.addEventListener('install', e => self.skipWaiting());
self.addEventListener('activate', e => e.waitUntil(self.clients.claim()));

self.addEventListener('fetch', e => {
  if (e.request.method !== 'GET') return;
  // auth state must never be answered from cache
  if (new URL(e.request.url).pathname.includes('/auth/')) return;
  e.respondWith(caches.open(CACHE).then(async cache => {
    const hit = await cache.match(e.request);
    const net = fetch(e.request).then(res => {
      if (res.ok) cache.put(e.request, res.clone());
      return res;
    }).catch(() => hit);
    return hit || net;
  }));
});
