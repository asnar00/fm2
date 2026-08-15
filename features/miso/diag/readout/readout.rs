struct feature_Readout;
impl feature_Readout {
    // screen contents are user data: free on localhost (the tooling case),
    // cookie-gated through the tunnel — in both directions.
    fn route(r: request) -> response {
        if r.path == "diag/readout" && r.method == "POST" {
            return readout_store(r);
        }
        if r.path == "diag/readout" {
            return readout_get(r);
        }
        existing.route(r)
    }

    fn readout_guarded(r: &request) -> bool {
        !r.tunnel || authed(r.cookie.clone())
    }

    fn readout_store(r: request) -> response {
        if !readout_guarded(&r) {
            return json_response(401, "{\"ok\":false,\"error\":\"log in first\"}".to_string());
        }
        let mut body = r.body;
        if body.len() > 262144 {
            body = body.chars().take(262144).collect();
        }
        let _ = std::fs::write(readout_file(), body);
        json_response(200, "{\"ok\":true}".to_string())
    }

    fn readout_get(r: request) -> response {
        if !readout_guarded(&r) {
            return json_response(401, "{\"ok\":false,\"error\":\"log in first\"}".to_string());
        }
        let body = std::fs::read_to_string(readout_file())
            .unwrap_or("{}".to_string());
        json_response(200, body)
    }

    fn readout_file() -> String {
        "/tmp/miso-readout.json".to_string()
    }
}
