struct feature_Loopback;
impl feature_Loopback {
    // same-host only: cloudflared reaches localhost:8095, LAN cannot. This is
    // what makes /gate's `!r.tunnel == trusted` sound — a cookieless request
    // can now only come from the mini itself.
    fn bind_host() -> String {
        "127.0.0.1".to_string()
    }
}
