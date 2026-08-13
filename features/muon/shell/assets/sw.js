// muon service worker. basic principle: the cache serves only when the
// network can't deliver IN TIME — a fresh copy that arrives within the
// deadline always wins; a slow network degrades to last-known-good (the
// fetch still completes in the background and refreshes the offline copy);
// offline is just the deadline missed instantly. every successful fetch
// refreshes the cache.
const CACHE = 'muon';
const DEADLINE_MS = 1200;

self.addEventListener('install', e => self.skipWaiting());
self.addEventListener('activate', e => e.waitUntil(self.clients.claim()));

// web push: show the notification even when the app is closed; tapping it
// opens (or focuses) the app
self.addEventListener('push', e => {
  let d = {};
  try { d = e.data ? e.data.json() : {}; } catch (err) {}
  e.waitUntil(self.registration.showNotification(d.title || 'muon', {
    body: d.body || '', icon: 'icon-192.png', badge: 'icon-192.png' }));
});
self.addEventListener('notificationclick', e => {
  e.notification.close();
  e.waitUntil(clients.matchAll({ type: 'window' }).then(list =>
    list.length ? list[0].focus() : clients.openWindow('/')));
});

self.addEventListener('fetch', e => {
  if (e.request.method !== 'GET') return;
  // auth state and the deploy stamp must never be answered from cache
  const path = new URL(e.request.url).pathname;
  if (path.includes('/auth/') || path.endsWith('/version')) return;
  e.respondWith(caches.open(CACHE).then(async cache => {
    const hit = await cache.match(e.request);
    const net = fetch(e.request).then(res => {
      if (res.ok) cache.put(e.request, res.clone());
      return res;
    });
    net.catch(() => {});             // background refresh failing is fine
    if (!hit) return net;            // nothing cached: the network is all we have
    return Promise.race([
      net.catch(() => hit),          // network error -> stale
      new Promise(resolve => setTimeout(() => resolve(hit), DEADLINE_MS)),
    ]);
  }));
});
