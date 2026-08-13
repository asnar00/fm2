pub struct request {
    pub method: String,
    pub path: String,
    pub cookie: String,
    pub body: String,
    pub tunnel: bool,
}

pub struct response {
    pub status: u16,
    pub ctype: String,
    pub body: Vec<u8>,
    pub set_cookie: String,
    pub cache: String,
}

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
        let r = parse_request(&s);
        let resp = route(r);
        write_response(s, resp);
    }

    fn parse_request(s: &std::net::TcpStream) -> request {
        use std::io::{BufRead, Read};
        let mut reader = std::io::BufReader::new(s);
        let mut line = String::new();
        let _ = reader.read_line(&mut line);
        let method = line.split_whitespace().next().unwrap_or("GET").to_string();
        let raw_path = line.split_whitespace().nth(1).unwrap_or("/").to_string();
        let mut cookie = String::new();
        let mut tunnel = false;
        let mut content_length = 0usize;
        loop {
            let mut h = String::new();
            if reader.read_line(&mut h).unwrap_or(0) == 0 {
                break;
            }
            let t = h.trim_end().to_string();
            if t.is_empty() {
                break;
            }
            let lower = t.to_lowercase();
            if lower.starts_with("cookie:") {
                cookie = t[7..].trim().to_string();
            }
            if lower.starts_with("cf-connecting-ip:") {
                tunnel = true;
            }
            if lower.starts_with("content-length:") {
                content_length = t[15..].trim().parse().unwrap_or(0);
            }
        }
        let mut body = String::new();
        if content_length > 0 && content_length < 65536 {
            let mut buf = vec![0u8; content_length];
            if reader.read_exact(&mut buf).is_ok() {
                body = String::from_utf8_lossy(&buf).to_string();
            }
        }
        request { method: method, path: clean_path(raw_path), cookie: cookie,
                  body: body, tunnel: tunnel }
    }

    // "/x?q" -> "x"; "/" -> "index.html"; ".." refused (falls back to index)
    fn clean_path(raw: String) -> String {
        let clean = raw.trim_start_matches('/').split('?').next().unwrap_or("").to_string();
        if clean.is_empty() || clean.contains("..") {
            "index.html".to_string()
        } else {
            clean
        }
    }

    // base route: static files from site/, with directory-index fallback
    // (site/<path>/index.html — the exported feature tree relies on it).
    // features extend this chain (auth gate, endpoints) via existing.route().
    fn route(r: request) -> response {
        let file = std::fs::read(format!("site/{}", r.path));
        match file {
            Ok(bytes) => response { status: 200, ctype: content_type(r.path).to_string(),
                                    body: bytes, set_cookie: String::new(),
                                    cache: "no-cache".to_string() },
            Err(_) => {
                let index = format!("site/{}/index.html", r.path.trim_end_matches('/'));
                match std::fs::read(index) {
                    Ok(bytes) => response { status: 200,
                                            ctype: "text/html; charset=utf-8".to_string(),
                                            body: bytes, set_cookie: String::new(),
                                            cache: "no-cache".to_string() },
                    Err(_) => text_response(404, "not found"),
                }
            }
        }
    }

    fn text_response(status: u16, text: &'static str) -> response {
        response { status: status, ctype: "text/plain".to_string(),
                   body: text.as_bytes().to_vec(), set_cookie: String::new(),
                   cache: "no-store".to_string() }
    }

    fn json_response(status: u16, json: String) -> response {
        response { status: status, ctype: "application/json".to_string(),
                   body: json.into_bytes(), set_cookie: String::new(),
                   cache: "no-store".to_string() }
    }

    fn write_response(s: std::net::TcpStream, r: response) {
        use std::io::Write;
        let mut s = s;
        let mut head = format!(
            "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nCache-Control: {}\r\nConnection: close\r\n",
            r.status, status_name(r.status), r.ctype, r.body.len(), r.cache);
        if !r.set_cookie.is_empty() {
            head = format!("{}Set-Cookie: {}\r\n", head, r.set_cookie);
        }
        head = format!("{}\r\n", head);
        let _ = s.write_all(head.as_bytes());
        let _ = s.write_all(&r.body);
    }

    fn status_name(status: u16) -> &'static str {
        if status == 200 { return "OK"; }
        if status == 401 { return "Unauthorized"; }
        if status == 403 { return "Forbidden"; }
        if status == 404 { return "Not Found"; }
        if status == 429 { return "Too Many Requests"; }
        "Error"
    }

    fn content_type(path: String) -> &'static str {
        if path.ends_with(".html") { return "text/html; charset=utf-8"; }
        if path.ends_with(".js") { return "text/javascript"; }
        if path.ends_with(".json") { return "application/manifest+json"; }
        if path.ends_with(".wasm") { return "application/wasm"; }
        if path.ends_with(".png") { return "image/png"; }
        if path.ends_with(".svg") { return "image/svg+xml"; }
        "application/octet-stream"
    }
}
