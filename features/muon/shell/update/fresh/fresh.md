# fresh
*the cache serves only when the network can't*

> (transcripts/2026-08-13-fm-spec.md#p53)
> yeah - the cache should only be used if we can't access the network - basic principle

## spec

The caching principle, stated as policy after stale-while-revalidate left launches one deploy behind: online always means current. The service worker fetches network-first for everything; every successful fetch refreshes the offline copy; only failure falls back to cache. `/auth/*` and `version` never touch the cache at all. `/deadline` refines what "can't access" means on slow networks.

## user

Online, you always see the latest deploy. Offline, you get the newest build your device has ever seen.

## glossary

(no new terms)

## code description

The fetch handler in `/shell`'s `assets/sw.js`: try the network, `cache.put` on success, fall back to the cached copy on failure; auth and version paths return early, uncached.
