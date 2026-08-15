struct feature_AutoExport;
impl feature_AutoExport {
    // /features/* answers from a bake; where the sources sit beside the
    // server (a dev machine), a stale bake re-exports before it answers
    fn route(r: request) -> response {
        if r.path == "features" || r.path.starts_with("features/") {
            refresh_if_stale();
        }
        existing.route(r)
    }

    fn newest_under(dir: &std::path::Path) -> Option<std::time::SystemTime> {
        let mut newest: Option<std::time::SystemTime> = None;
        let entries = std::fs::read_dir(dir).ok()?;
        for e in entries.flatten() {
            let p = e.path();
            let t = if p.is_dir() {
                newest_under(&p)
            } else {
                e.metadata().ok().and_then(|m| m.modified().ok())
            };
            if let Some(t) = t {
                if newest.map_or(true, |n| t > n) {
                    newest = Some(t);
                }
            }
        }
        newest
    }

    fn refresh_if_stale() {
        let src = std::path::Path::new("../../../features");
        let script = std::path::Path::new("../../../tools/export_features.py");
        if !src.is_dir() || !script.is_file() {
            return;
        }
        let baked = std::fs::metadata("site/features/tree.json")
            .ok()
            .and_then(|m| m.modified().ok());
        let stale = match (newest_under(src), baked) {
            (Some(n), Some(b)) => n > b,
            (Some(_), None) => true,
            _ => false,
        };
        if !stale {
            return;
        }
        let ok = std::process::Command::new("python3")
            .arg("../../../tools/export_features.py")
            .status()
            .map(|s| s.success())
            .unwrap_or(false);
        if ok {
            // embeddings follow along, off the request's clock
            let _ = std::process::Command::new("python3")
                .arg("../../../tools/embed_catalog.py")
                .spawn();
        } else {
            println!("auto-export: export failed; serving the stale bake");
        }
    }
}
