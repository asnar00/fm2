struct feature_Users;
impl feature_Users {
    fn auth_dir() -> String {
        format!("{}/.miso-auth", std::env::var("HOME").unwrap_or_default())
    }

    // "07890 123456" and "+44 7890…" normalise the same way: digits with + restored
    fn normalise_phone(p: String) -> String {
        let digits: String = p.chars().filter(|c| c.is_ascii_digit()).collect();
        if digits.is_empty() {
            String::new()
        } else {
            format!("+{}", digits)
        }
    }

    // last 4 digits: enough to follow a login in the log, not enough to leak
    fn tag(p: String) -> String {
        let n = p.len();
        if n >= 4 {
            format!("…{}", &p[n - 4..])
        } else {
            "…".to_string()
        }
    }

    // the guest list: ~/.miso-auth/users.json, [{ "name": "...", "phone": "+..." }],
    // read fresh every request so adding someone needs no restart.
    // returns the user's name, or "" if the phone isn't invited.
    fn find_user(phone: String) -> String {
        let raw = std::fs::read_to_string(format!("{}/users.json", auth_dir()))
            .unwrap_or_default();
        let v: serde_json::Value = serde_json::from_str(&raw)
            .unwrap_or(serde_json::Value::Null);
        let list = v.as_array().cloned().unwrap_or_default();
        for u in list {
            let uphone = normalise_phone(u["phone"].as_str().unwrap_or("").to_string());
            if !uphone.is_empty() && uphone == phone {
                return u["name"].as_str().unwrap_or("").to_string();
            }
        }
        String::new()
    }

    fn random_bytes(n: usize) -> Vec<u8> {
        use std::io::Read;
        let mut buf = vec![0u8; n];
        let mut f = std::fs::File::open("/dev/urandom").expect("miso: no /dev/urandom");
        let _ = f.read_exact(&mut buf);
        buf
    }

    // 32-byte signing secret, generated once into ~/.miso-auth/secret
    fn secret() -> Vec<u8> {
        let file = format!("{}/secret", auth_dir());
        let have = std::fs::read(&file);
        match have {
            Ok(bytes) => bytes,
            Err(_) => {
                let fresh = random_bytes(32);
                let _ = std::fs::create_dir_all(auth_dir());
                let _ = std::fs::write(&file, &fresh);
                fresh
            }
        }
    }

    fn hex(bytes: Vec<u8>) -> String {
        bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }

    // HMAC-SHA256 (the standard construction over the sha2 crate)
    fn hmac_sha256(key: Vec<u8>, msg: String) -> String {
        use sha2::Digest;
        let mut k = key;
        if k.len() > 64 {
            let mut h = sha2::Sha256::new();
            h.update(&k);
            k = h.finalize().to_vec();
        }
        k.resize(64, 0);
        let ipad: Vec<u8> = k.iter().map(|b| b ^ 0x36).collect();
        let opad: Vec<u8> = k.iter().map(|b| b ^ 0x5c).collect();
        let mut inner = sha2::Sha256::new();
        inner.update(&ipad);
        inner.update(msg.as_bytes());
        let inner_hash = inner.finalize();
        let mut outer = sha2::Sha256::new();
        outer.update(&opad);
        outer.update(inner_hash);
        hex(outer.finalize().to_vec())
    }

    fn now_ms() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }

    // stateless session token "<digits>.<expiry>.<hmac>" — survives every
    // deploy and restart because nothing is stored server-side per session
    fn make_token(phone: String) -> String {
        let exp = now_ms() + 31536000000u64;
        let payload = format!("{}.{}", phone.replace("+", ""), exp);
        format!("{}.{}", payload, hmac_sha256(secret(), payload.clone()))
    }

    fn token_valid(token: String) -> bool {
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return false;
        }
        let exp: u64 = parts[1].parse().unwrap_or(0);
        if exp < now_ms() {
            return false;
        }
        let payload = format!("{}.{}", parts[0], parts[1]);
        constant_eq(hmac_sha256(secret(), payload), parts[2].to_string())
    }

    fn constant_eq(a: String, b: String) -> bool {
        if a.len() != b.len() {
            return false;
        }
        let mut diff = 0u8;
        for pair in a.bytes().zip(b.bytes()) {
            diff = diff | (pair.0 ^ pair.1);
        }
        diff == 0
    }

    fn token_phone(token: String) -> String {
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() == 3 {
            format!("+{}", parts[0])
        } else {
            String::new()
        }
    }

    fn cookie_token(cookie: String) -> String {
        for part in cookie.split(';') {
            let t = part.trim();
            if t.starts_with("miso_auth=") {
                return t[10..].to_string();
            }
        }
        String::new()
    }

    fn authed(cookie: String) -> bool {
        let t = cookie_token(cookie);
        !t.is_empty() && token_valid(t)
    }
}
