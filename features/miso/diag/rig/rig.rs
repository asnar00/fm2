struct feature_Rig;
impl feature_Rig {
    // a server started with MISO_RIG=1 is a test rig: every page it serves may
    // switch its eyes and hands on without a query string. Never through the
    // tunnel — a rig is a laptop talking to itself.
    // a rig picks its port from the environment: MISO_PORT=8099 — so a
    // relink (which rebuilds the binary) never puts a rig back on 8095
    fn serve_port() -> u16 {
        match std::env::var("MISO_PORT").ok().and_then(|p| p.parse::<u16>().ok()) {
            Some(p) => p,
            None => existing.serve_port(),
        }
    }

    fn rig_on() -> bool {
        std::env::var("MISO_RIG").unwrap_or_default() == "1"
    }

    fn route(r: request) -> response {
        if r.path == "diag/rig" {
            let on = !r.tunnel && rig_on();
            return json_response(200, format!("{{\"rig\":{}}}", on));
        }
        // a rig is plain http on localhost, and WebKit drops a `Secure` cookie
        // there (Chrome keeps it, which is why the desktop rigs never saw it):
        // on a rig, a localhost response's cookie loses the flag
        let plain = !r.tunnel && rig_on();
        let mut out = existing.route(r);
        if plain && !out.set_cookie.is_empty() {
            out.set_cookie = out.set_cookie.replace("Secure; ", "");
        }
        out
    }
}
