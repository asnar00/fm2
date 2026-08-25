struct feature_Pin;
impl feature_Pin {
    fn pending_file() -> String {
        format!("{}/pending.txt", auth_dir())
    }

    fn sends_file() -> String {
        format!("{}/sends.txt", auth_dir())
    }

    fn make_pin() -> String {
        let b = random_bytes(2);
        let n = ((b[0] as u32) * 256 + (b[1] as u32)) % 10000;
        format!("{:04}", n)
    }

    // pending PINs are on disk, not in memory: the server restarts on every
    // deploy, and a code already texted out must survive that
    fn load_pending(phone: String) -> String {
        let raw = std::fs::read_to_string(pending_file()).unwrap_or_default();
        for line in raw.lines() {
            let parts: Vec<&str> = line.split(' ').collect();
            if parts.len() == 4 && parts[0] == phone {
                return format!("{} {} {}", parts[1], parts[2], parts[3]);
            }
        }
        String::new()
    }

    fn set_pending_line(phone: String, rest: String) {
        let raw = std::fs::read_to_string(pending_file()).unwrap_or_default();
        let mut out = String::new();
        for line in raw.lines() {
            if !line.starts_with(&format!("{} ", phone)) && !line.is_empty() {
                out = format!("{}{}\n", out, line);
            }
        }
        if !rest.is_empty() {
            out = format!("{}{} {}\n", out, phone, rest);
        }
        let _ = std::fs::create_dir_all(auth_dir());
        let _ = std::fs::write(pending_file(), out);
    }

    fn save_pending(phone: String, pin: String) {
        set_pending_line(phone, format!("{} {} 0", pin, now_ms() + 300000));
    }

    fn clear_pending(phone: String) {
        set_pending_line(phone, String::new());
    }

    // a lost-code retry is fine; a code fountain is not (5 texts/phone/hour)
    fn sms_count_last_hour(phone: String) -> u32 {
        let raw = std::fs::read_to_string(sends_file()).unwrap_or_default();
        let cutoff = now_ms() - 3600000;
        let mut count = 0u32;
        for line in raw.lines() {
            let parts: Vec<&str> = line.split(' ').collect();
            if parts.len() == 2 && parts[0] == phone {
                let ts: u64 = parts[1].parse().unwrap_or(0);
                if ts > cutoff {
                    count = count + 1;
                }
            }
        }
        count
    }

    fn record_sms(phone: String) {
        let raw = std::fs::read_to_string(sends_file()).unwrap_or_default();
        let cutoff = now_ms() - 3600000;
        let mut out = String::new();
        for line in raw.lines() {
            let parts: Vec<&str> = line.split(' ').collect();
            if parts.len() == 2 && parts[1].parse::<u64>().unwrap_or(0) > cutoff {
                out = format!("{}{}\n", out, line);
            }
        }
        out = format!("{}{} {}\n", out, phone, now_ms());
        let _ = std::fs::create_dir_all(auth_dir());
        let _ = std::fs::write(sends_file(), out);
    }

    // base SMS delivery: console (test/dev). the vonage subfeature extends this.
    fn send_sms(to: String, text: String) -> String {
        println!("sms (console): to {} : {}", to, text);
        String::new()
    }

    fn auth_request(r: request) -> response {
        let v: serde_json::Value = serde_json::from_str(&r.body)
            .unwrap_or(serde_json::Value::Null);
        let phone = normalise_phone(v["phone"].as_str().unwrap_or("").to_string());
        let name = find_user(phone.clone());
        if name.is_empty() {
            println!("auth: request {} -> NOT ON LIST", tag(phone));
            return json_response(403,
                "{\"ok\":false,\"error\":\"that number isn't on the guest list\"}".to_string());
        }
        println!("auth: request {} -> {}", tag(phone.clone()), name);
        if sms_count_last_hour(phone.clone()) >= 5 {
            return json_response(429,
                "{\"ok\":false,\"error\":\"too many codes sent — try later\"}".to_string());
        }
        let pin = make_pin();
        save_pending(phone.clone(), pin.clone());
        if name.starts_with("_") {
            println!("auth: test user {} pin {}", name, pin);
        } else {
            let err = send_sms(phone.clone(), format!("miso login code: {}", pin));
            if !err.is_empty() {
                clear_pending(phone.clone());
                println!("auth: sms send failed: {}", err);
                return json_response(500,
                    "{\"ok\":false,\"error\":\"couldn't send the code — try again\"}".to_string());
            }
            record_sms(phone.clone());
            println!("auth: sms sent {}", tag(phone));
        }
        json_response(200, format!("{{\"ok\":true,\"name\":\"{}\"}}", name))
    }

    fn auth_verify(r: request) -> response {
        let v: serde_json::Value = serde_json::from_str(&r.body)
            .unwrap_or(serde_json::Value::Null);
        let phone = normalise_phone(v["phone"].as_str().unwrap_or("").to_string());
        let pin = v["pin"].as_str().unwrap_or("").trim().to_string();
        let p = load_pending(phone.clone());
        if p.is_empty() {
            return json_response(403,
                "{\"ok\":false,\"error\":\"no code pending — request one\"}".to_string());
        }
        let parts: Vec<&str> = p.split(' ').collect();
        let want = parts[0].to_string();
        let expires: u64 = parts[1].parse().unwrap_or(0);
        let attempts: u64 = parts[2].parse().unwrap_or(0);
        if expires < now_ms() {
            clear_pending(phone);
            return json_response(403,
                "{\"ok\":false,\"error\":\"code expired — request another\"}".to_string());
        }
        if attempts + 1 > 3 {
            clear_pending(phone);
            return json_response(403,
                "{\"ok\":false,\"error\":\"too many tries — request another code\"}".to_string());
        }
        if !constant_eq(want.clone(), pin) {
            set_pending_line(phone.clone(),
                             format!("{} {} {}", want, expires, attempts + 1));
            println!("auth: verify {} WRONG CODE (attempt {})", tag(phone), attempts + 1);
            return json_response(401,
                "{\"ok\":false,\"error\":\"wrong code\"}".to_string());
        }
        println!("auth: verify {} OK — cookie issued", tag(phone.clone()));
        clear_pending(phone.clone());
        let mut resp = json_response(200, "{\"ok\":true}".to_string());
        resp.set_cookie = format!(
            "miso_auth={}; Max-Age=31536000; Path=/; Secure; HttpOnly; SameSite=Lax",
            make_token(phone));
        resp
    }

    fn auth_whoami(r: request) -> response {
        let t = cookie_token(r.cookie);
        if !t.is_empty() && token_valid(t.clone()) {
            let name = find_user(token_phone(t));
            return json_response(200,
                format!("{{\"ok\":true,\"authed\":true,\"name\":\"{}\"}}", name));
        }
        json_response(200, "{\"ok\":true,\"authed\":false}".to_string())
    }

    // stateless tokens can't be revoked server-side; logout clears the cookie
    fn auth_logout(r: request) -> response {
        println!("auth: logout {}", tag(token_phone(cookie_token(r.cookie))));
        let mut resp = json_response(200, "{\"ok\":true}".to_string());
        resp.set_cookie =
            "miso_auth=; Max-Age=0; Path=/; Secure; HttpOnly; SameSite=Lax".to_string();
        resp
    }
}
