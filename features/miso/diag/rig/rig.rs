struct feature_Rig;
impl feature_Rig {
    // a server started with MISO_RIG=1 is a test rig: every page it serves may
    // switch its eyes and hands on without a query string. Never through the
    // tunnel — a rig is a laptop talking to itself.
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
