struct feature_KeepWorker;
impl feature_KeepWorker {
    // a rig started with MISO_RIG_KEEP=1 keeps the page's service worker and
    // caches: the rig answer carries keep:true and the page's arming leaves
    // them alone. Off by default — a plain rig still runs the code it was
    // given.
    fn rig_keep() -> bool {
        std::env::var("MISO_RIG_KEEP").unwrap_or_default() == "1"
    }

    fn route(r: request) -> response {
        if r.path == "diag/rig" {
            let on = !r.tunnel && rig_on();
            let keep = on && rig_keep();
            return json_response(200, format!("{{\"rig\":{},\"keep\":{}}}", on, keep));
        }
        existing.route(r)
    }
}
