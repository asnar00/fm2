struct feature_PlainCookie;
impl feature_PlainCookie {
    // /rig's rule, restated at this provenance so it wraps every route link,
    // the ones newer than /rig included: on a rig (plain http, never the
    // tunnel) a cookie loses `Secure`, or WebKit drops it.
    fn route(r: request) -> response {
        let plain = !r.tunnel && rig_on();
        let mut out = existing.route(r);
        if plain && !out.set_cookie.is_empty() {
            out.set_cookie = out.set_cookie.replace("Secure; ", "");
        }
        out
    }
}
