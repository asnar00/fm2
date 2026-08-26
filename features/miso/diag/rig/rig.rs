struct feature_Rig;
impl feature_Rig {
    // a server started with MISO_RIG=1 is a test rig: every page it serves may
    // switch its eyes and hands on without a query string. Never through the
    // tunnel — a rig is a laptop talking to itself.
    fn route(r: request) -> response {
        if r.path == "diag/rig" {
            let on = !r.tunnel && std::env::var("MISO_RIG").unwrap_or_default() == "1";
            return json_response(200, format!("{{\"rig\":{}}}", on));
        }
        existing.route(r)
    }
}
