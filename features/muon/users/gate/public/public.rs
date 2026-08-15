struct feature_Public;
impl feature_Public {
    // the policy itself: the app shell is publicly served so installed apps
    // can always update (gating it once froze logged-out PWAs solid), and the
    // feature tree is deliberately public — a shareable artefact.
    fn is_public(path: String) -> bool {
        if path == "index.html" || path == "sw.js" || path == "client.wasm"
            || path == "manifest.json" || path == "login.html"
            || path == "install.html" || path == "version"
            || path == "changes.json" || path == "hashes.json"
            || path.starts_with("f/") || path.starts_with("icon-")
            || path == "features" || path.starts_with("features/") {
            return true;
        }
        existing.is_public(path)
    }
}
