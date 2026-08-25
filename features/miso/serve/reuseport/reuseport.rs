struct feature_Reuseport;
impl feature_Reuseport {
    // the whole node: the same host and port /serve and /loopback already
    // decided, on a socket that does not exclude a second holder. Everything
    // unix-specific lives in reuseport.lib.rs so this file stays readable.
    fn bind_listener() -> std::net::TcpListener {
        fm_bind_reuseport(bind_host(), serve_port())
    }
}
