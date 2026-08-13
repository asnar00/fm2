// muon service worker: a skeleton the features fill in. caching policy lives
// in /shell/update/fresh (+/deadline); push listeners in /push.
const CACHE = 'muon';

self.addEventListener('install', (e) => self.skipWaiting());
self.addEventListener('activate', (e) => e.waitUntil(self.clients.claim()));

// fm:script
