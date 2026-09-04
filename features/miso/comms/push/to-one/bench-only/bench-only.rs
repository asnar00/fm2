struct feature_BenchOnly;
impl feature_BenchOnly {
    // /to-one shipped with `pic/retrofit`'s screen: a caller through the
    // tunnel had only to be logged in. That is the wrong bar for a door that
    // sends a notification to a number of the caller's choosing with words of
    // the caller's choosing — every canvasser with a session could have rung
    // anyone. This door is the bench's, so it answers the bench alone.
    //
    // The screen is `r.tunnel`, the one /diag/context already trusts: /serve
    // sets it from the `cf-connecting-ip` header cloudflared adds, and
    // /loopback binds the listener to 127.0.0.1, so a request that is NOT
    // through the tunnel can only have been made on the box itself. A cookie
    // cannot change that either way, which is the point — no session, however
    // senior, opens this.
    //
    // A refused caller gets the base's own miss for a path with no route
    // (`text_response(404, "not found")`, /serve's `route`), not a 401 or a
    // 403: from the tunnel this road does not exist, and a probe learns
    // nothing about whether it is composed.
    fn push_one_route(r: request) -> response {
        if r.tunnel {
            return text_response(404, "not found");
        }
        existing.push_one_route(r)
    }
}
