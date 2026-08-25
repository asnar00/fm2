pub struct request {
    pub method: String,
    pub path: String,
    pub cookie: String,
    pub body: String,
    pub raw: Vec<u8>,
    pub tunnel: bool,
    // the request line's query string, without the '?'. Parsed here because
    // parsing the request line is this feature's job; clean_path threw it away
    // before, which left routes no way to see a parameter at all.
    pub query: String,
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
    // the interface to bind. Base is all-interfaces (dev convenience); the
    // /loopback subfeature overrides this to 127.0.0.1 so only same-host
    // callers (cloudflared, local tooling) can reach the port at all.
    fn bind_host() -> String {
        "0.0.0.0".to_string()
    }

    // the port to listen on. A seam because the number was inline until
    // 2026-08-25, which cost a rig worker real time; nothing about serve
    // changes, the constant simply has a name a feature can redefine.
    fn serve_port() -> u16 {
        8095
    }

    // how the listener is made, separated from the accept loop so a feature
    // can change the socket without owning the loop. /reuseport redefines it
    // to a socket two processes may share during a handover.
    fn bind_listener() -> std::net::TcpListener {
        std::net::TcpListener::bind((bind_host(), serve_port()))
            .expect("miso: cannot bind the server port")
    }

    fn serve() {
        let listener = bind_listener();
        println!("miso serving site/ on http://localhost:{}", serve_port());
        for stream in listener.incoming() {
            match stream {
                Ok(s) => handle(s),
                // never spin on a broken listener: /handover closes this
                // socket to leave the port, after which accept fails at once
                // and forever, and a hot loop would eat the box on the way
                // out. In ordinary running this arm is never taken.
                Err(_) => std::thread::sleep(std::time::Duration::from_millis(20)),
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
        // body arrives as raw bytes (binary-safe, e.g. audio uploads) with a
        // lossy String view beside it for the JSON endpoints
        let mut raw: Vec<u8> = Vec::new();
        if content_length > 0 && content_length < 16 * 1024 * 1024 {
            let mut buf = vec![0u8; content_length];
            if reader.read_exact(&mut buf).is_ok() {
                raw = buf;
            }
        }
        let body = String::from_utf8_lossy(&raw).to_string();
        request { method: method, path: clean_path(raw_path.clone()), cookie: cookie,
                  body: body, raw: raw, tunnel: tunnel,
                  query: query_of(raw_path) }
    }

    // "/x?a=1&b=2" -> "a=1&b=2"; no query -> "". The other half of clean_path:
    // what it strips, this keeps, so a route can read a parameter.
    fn query_of(raw: String) -> String {
        raw.splitn(2, '?').nth(1).unwrap_or("").to_string()
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
        if path.ends_with(".css") { return "text/css"; }
        if path.ends_with(".json") { return "application/manifest+json"; }
        if path.ends_with(".wasm") { return "application/wasm"; }
        if path.ends_with(".png") { return "image/png"; }
        if path.ends_with(".svg") { return "image/svg+xml"; }
        "application/octet-stream"
    }
}
