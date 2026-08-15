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

This node owns `fresh.sw.js`: the service worker's fetch handler — network-first with cache refresh on success and cache fallback on failure, `/auth/*` and `version` exempt. It consults `feature_Deadline` defensively: absent (`/deadline` unticked) means pure network-first.
