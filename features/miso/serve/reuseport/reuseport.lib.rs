// SO_REUSEPORT lets several sockets hold one address and port, which is the
// one thing standing between a release and a handover. std exposes no
// setsockopt, so the socket is made by hand and adopted as a raw descriptor;
// everything above bind_listener is unchanged by that.
#[cfg(unix)]
pub fn fm_bind_reuseport(host: String, port: u16) -> std::net::TcpListener {
    use std::os::unix::io::FromRawFd;
    let ip: std::net::Ipv4Addr = host
        .parse()
        .unwrap_or(std::net::Ipv4Addr::new(127, 0, 0, 1));
    unsafe {
        let fd = libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0);
        if fd < 0 {
            panic!("miso: cannot create a listening socket");
        }
        let on: libc::c_int = 1;
        let on_ptr = &on as *const libc::c_int as *const libc::c_void;
        let on_len = std::mem::size_of::<libc::c_int>() as libc::socklen_t;
        // REUSEADDR for the ordinary reason (no TIME_WAIT wait on restart),
        // REUSEPORT for this node's reason (a successor beside the incumbent)
        libc::setsockopt(fd, libc::SOL_SOCKET, libc::SO_REUSEADDR, on_ptr, on_len);
        libc::setsockopt(fd, libc::SOL_SOCKET, libc::SO_REUSEPORT, on_ptr, on_len);
        let mut addr: libc::sockaddr_in = std::mem::zeroed();
        addr.sin_family = libc::AF_INET as libc::sa_family_t;
        addr.sin_port = port.to_be();
        // octets() is already network order, so the bytes go across as they are
        addr.sin_addr.s_addr = u32::from_ne_bytes(ip.octets());
        #[cfg(any(target_os = "macos", target_os = "ios"))]
        {
            addr.sin_len = std::mem::size_of::<libc::sockaddr_in>() as u8;
        }
        let addr_ptr = &addr as *const libc::sockaddr_in as *const libc::sockaddr;
        let addr_len = std::mem::size_of::<libc::sockaddr_in>() as libc::socklen_t;
        if libc::bind(fd, addr_ptr, addr_len) != 0 {
            libc::close(fd);
            panic!("miso: cannot bind {}:{}", host, port);
        }
        if libc::listen(fd, 128) != 0 {
            libc::close(fd);
            panic!("miso: cannot listen on {}:{}", host, port);
        }
        std::net::TcpListener::from_raw_fd(fd)
    }
}

// the wasm place compiles this body too and never calls it; the plain bind is
// both what that place would have done and what any non-unix host needs.
#[cfg(not(unix))]
pub fn fm_bind_reuseport(host: String, port: u16) -> std::net::TcpListener {
    std::net::TcpListener::bind((host, port)).expect("miso: cannot bind the server port")
}
