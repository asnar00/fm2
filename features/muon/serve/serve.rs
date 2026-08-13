struct feature_Serve;
impl feature_Serve {
    fn serve() {
        let listener = std::net::TcpListener::bind(("0.0.0.0", 8095u16))
            .expect("muon: cannot bind port 8095");
        println!("muon serving site/ on http://localhost:8095");
        for stream in listener.incoming() {
            if let Ok(s) = stream {
                handle(s);
            }
        }
    }

    fn handle(s: std::net::TcpStream) {
        let path = request_path(&s);
        let file = site_file(&path);
        respond(s, file, content_type(&path));
    }

    // first line of the request: "GET /x HTTP/1.1" -> "x"; "/" -> "index.html"
    fn request_path(s: &std::net::TcpStream) -> String {
        use std::io::{BufRead, BufReader};
        let mut line = String::new();
        let _ = BufReader::new(s).read_line(&mut line);
        let raw = line.split_whitespace().nth(1).unwrap_or("/");
        let clean = raw.trim_start_matches('/').split('?').next().unwrap_or("");
        if clean.is_empty() || clean.contains("..") {
            "index.html".to_string()
        } else {
            clean.to_string()
        }
    }

    fn site_file(path: &String) -> Option<Vec<u8>> {
        std::fs::read(format!("site/{}", path)).ok()
    }

    fn respond(s: std::net::TcpStream, file: Option<Vec<u8>>, ctype: &'static str) {
        use std::io::Write;
        let mut s = s;
        let body = match file {
            Some(bytes) => bytes,
            None => {
                let _ = s.write_all(
                    b"HTTP/1.1 404 Not Found\r\nContent-Length: 9\r\n\r\nnot found");
                return;
            }
        };
        let head = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\nCache-Control: no-cache\r\n\r\n",
            ctype, body.len());
        let _ = s.write_all(head.as_bytes());
        let _ = s.write_all(&body);
    }

    fn content_type(path: &String) -> &'static str {
        if path.ends_with(".html") { return "text/html; charset=utf-8"; }
        if path.ends_with(".js") { return "text/javascript"; }
        if path.ends_with(".json") { return "application/manifest+json"; }
        if path.ends_with(".wasm") { return "application/wasm"; }
        if path.ends_with(".png") { return "image/png"; }
        if path.ends_with(".svg") { return "image/svg+xml"; }
        "application/octet-stream"
    }
}
