struct feature_Diag;
impl feature_Diag {
    // client-side eyes: the app posts launch reports and errors here, so a
    // developer can debug an installed PWA on a phone they can't touch.
    // public on purpose — the broken or logged-out client is exactly the one
    // that needs to be able to report in.
    fn route(r: request) -> response {
        if r.path == "diag/report" && r.method == "POST" {
            return diag_report(r);
        }
        existing.route(r)
    }

    fn diag_report(r: request) -> response {
        let mut body = r.body;
        if body.len() > 2048 {
            body = body.chars().take(2048).collect();
        }
        rotate_diag_log();
        append_diag(format!("{} {}\n", now_ms(), body.replace("\n", " ")));
        json_response(200, "{\"ok\":true}".to_string())
    }

    fn diag_file() -> String {
        "/tmp/muon-diag.log".to_string()
    }

    fn rotate_diag_log() {
        let size = std::fs::metadata(diag_file()).map(|m| m.len()).unwrap_or(0);
        if size > 1000000 {
            let _ = std::fs::rename(diag_file(), format!("{}.old", diag_file()));
        }
    }

    fn append_diag(line: String) {
        use std::io::Write;
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(diag_file());
        if let Ok(f) = file {
            let mut f = f;
            let _ = f.write_all(line.as_bytes());
        }
    }
}
