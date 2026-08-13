struct feature_Threads;
impl feature_Threads {
    // one thread per connection: a parked long-poll no longer blocks the world
    fn handle(s: std::net::TcpStream) {
        std::thread::spawn(move || {
            existing.handle(s);
        });
    }
}
