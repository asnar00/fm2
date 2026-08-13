struct feature_Passkey;
impl feature_Passkey {
    // WebAuthn endpoints: register while SMS-logged-in, then sign in with
    // Face ID. runs ahead of the gate in the route chain; register requires
    // the cookie, login endpoints are public by nature.
    fn route(r: request) -> response {
        if r.path == "auth/passkey/register-challenge" && r.method == "POST" {
            return passkey_register_challenge(r);
        }
        if r.path == "auth/passkey/register" && r.method == "POST" {
            return passkey_register(r);
        }
        if r.path == "auth/passkey/challenge" && r.method == "POST" {
            return passkey_login_challenge(r);
        }
        if r.path == "auth/passkey/login" && r.method == "POST" {
            return passkey_login(r);
        }
        existing.route(r)
    }

    // the relying party is the public hostname (punycode form — this is what
    // the browser sees). passkeys only work through the tunnel domain.
    fn rp_id() -> String {
        "muon.xn--nb-lkaa.org".to_string()
    }

    fn origin() -> String {
        "https://muon.xn--nb-lkaa.org".to_string()
    }

    fn b64u_encode(bytes: Vec<u8>) -> String {
        use base64::Engine;
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes)
    }

    fn b64u_decode(s: String) -> Vec<u8> {
        use base64::Engine;
        base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(s).unwrap_or_default()
    }

    fn sha256(bytes: Vec<u8>) -> Vec<u8> {
        use sha2::Digest;
        let mut h = sha2::Sha256::new();
        h.update(&bytes);
        h.finalize().to_vec()
    }

    fn unhex(s: String) -> Vec<u8> {
        let bytes = s.into_bytes();
        let mut out = Vec::new();
        let mut i = 0;
        while i + 1 < bytes.len() {
            out.push(hex_val(bytes[i]) * 16 + hex_val(bytes[i + 1]));
            i = i + 2;
        }
        out
    }

    fn hex_val(c: u8) -> u8 {
        if c >= 48 && c <= 57 { return c - 48; }
        if c >= 97 && c <= 102 { return c - 97 + 10; }
        0
    }

    // ---- one-time challenges (5-minute expiry, single use, on disk)

    fn challenges_file() -> String {
        format!("{}/challenges.txt", auth_dir())
    }

    fn new_challenge(purpose: String, phone: String) -> String {
        let c = b64u_encode(random_bytes(32));
        let raw = std::fs::read_to_string(challenges_file()).unwrap_or_default();
        let mut out = String::new();
        for line in raw.lines() {
            let parts: Vec<&str> = line.split(' ').collect();
            if parts.len() == 4 && parts[3].parse::<u64>().unwrap_or(0) > now_ms() {
                out = format!("{}{}\n", out, line);
            }
        }
        out = format!("{}{} {} {} {}\n", out, c, purpose, phone, now_ms() + 300000);
        let _ = std::fs::create_dir_all(auth_dir());
        let _ = std::fs::write(challenges_file(), out);
        c
    }

    // consumes the challenge; returns the phone bound at issue ("-" for
    // login challenges), or "" if unknown/expired/wrong purpose
    fn take_challenge(c: String, purpose: String) -> String {
        let raw = std::fs::read_to_string(challenges_file()).unwrap_or_default();
        let mut out = String::new();
        let mut found = String::new();
        for line in raw.lines() {
            let parts: Vec<&str> = line.split(' ').collect();
            if parts.len() == 4 && parts[0] == c && parts[1] == purpose {
                if parts[3].parse::<u64>().unwrap_or(0) > now_ms() {
                    found = parts[2].to_string();
                }
                continue;
            }
            out = format!("{}{}\n", out, line);
        }
        let _ = std::fs::write(challenges_file(), out);
        found
    }

    // ---- stored passkeys: "cred_id_b64u xy_hex phone" per line

    fn passkeys_file() -> String {
        format!("{}/passkeys.txt", auth_dir())
    }

    fn add_passkey(cred_id: String, xy_hex: String, phone: String) {
        let raw = std::fs::read_to_string(passkeys_file()).unwrap_or_default();
        let _ = std::fs::create_dir_all(auth_dir());
        let _ = std::fs::write(passkeys_file(),
                               format!("{}{} {} {}\n", raw, cred_id, xy_hex, phone));
    }

    // returns "xy_hex phone" or ""
    fn find_passkey(cred_id: String) -> String {
        let raw = std::fs::read_to_string(passkeys_file()).unwrap_or_default();
        for line in raw.lines() {
            let parts: Vec<&str> = line.split(' ').collect();
            if parts.len() == 3 && parts[0] == cred_id {
                return format!("{} {}", parts[1], parts[2]);
            }
        }
        String::new()
    }

    // ---- webauthn plumbing

    // checks type + origin; returns the challenge string or ""
    fn client_data_challenge(client_data: Vec<u8>, expect_type: String) -> String {
        let v: serde_json::Value = serde_json::from_slice(&client_data)
            .unwrap_or(serde_json::Value::Null);
        if v["type"].as_str().unwrap_or("") != expect_type {
            return String::new();
        }
        if v["origin"].as_str().unwrap_or("") != origin() {
            return String::new();
        }
        v["challenge"].as_str().unwrap_or("").to_string()
    }

    // attestationObject (CBOR map) -> authData bytes
    fn attestation_auth_data(att: Vec<u8>) -> Vec<u8> {
        let value: ciborium::Value = match ciborium::de::from_reader(&att[..]) {
            Ok(v) => v,
            Err(_) => return Vec::new(),
        };
        let map = match value.as_map() {
            Some(m) => m.clone(),
            None => return Vec::new(),
        };
        for pair in map {
            if pair.0.as_text().unwrap_or("") == "authData" {
                return pair.1.as_bytes().cloned().unwrap_or_default();
            }
        }
        Vec::new()
    }

    // authData with attested credential -> (credential id, x||y public key)
    fn parse_attested(auth_data: Vec<u8>) -> (Vec<u8>, Vec<u8>) {
        if auth_data.len() < 55 || auth_data[32] & 0x40 == 0 {
            return (Vec::new(), Vec::new());
        }
        let idlen = ((auth_data[53] as usize) << 8) + (auth_data[54] as usize);
        if auth_data.len() < 55 + idlen {
            return (Vec::new(), Vec::new());
        }
        let cred_id = auth_data[55..55 + idlen].to_vec();
        let xy = cose_p256_xy(auth_data[55 + idlen..].to_vec());
        (cred_id, xy)
    }

    // COSE EC2 key (CBOR map, -2 = x, -3 = y) -> x||y (64 bytes) or empty
    fn cose_p256_xy(cose: Vec<u8>) -> Vec<u8> {
        let value: ciborium::Value = match ciborium::de::from_reader(&cose[..]) {
            Ok(v) => v,
            Err(_) => return Vec::new(),
        };
        let map = match value.as_map() {
            Some(m) => m.clone(),
            None => return Vec::new(),
        };
        let mut x = Vec::new();
        let mut y = Vec::new();
        for pair in map {
            let key = match pair.0.as_integer() {
                Some(i) => i128::from(i),
                None => continue,
            };
            if key == -2 {
                x = pair.1.as_bytes().cloned().unwrap_or_default();
            }
            if key == -3 {
                y = pair.1.as_bytes().cloned().unwrap_or_default();
            }
        }
        if x.len() == 32 && y.len() == 32 {
            let mut out = x;
            out.extend(y);
            out
        } else {
            Vec::new()
        }
    }

    fn p256_verify(xy: Vec<u8>, msg: Vec<u8>, sig_der: Vec<u8>) -> bool {
        use p256::ecdsa::signature::Verifier;
        if xy.len() != 64 {
            return false;
        }
        let point = p256::EncodedPoint::from_affine_coordinates(
            p256::FieldBytes::from_slice(&xy[0..32]),
            p256::FieldBytes::from_slice(&xy[32..64]),
            false);
        let vk = match p256::ecdsa::VerifyingKey::from_encoded_point(&point) {
            Ok(k) => k,
            Err(_) => return false,
        };
        let sig = match p256::ecdsa::Signature::from_der(&sig_der) {
            Ok(s) => s,
            Err(_) => return false,
        };
        vk.verify(&msg, &sig).is_ok()
    }

    // ---- endpoints

    fn passkey_register_challenge(r: request) -> response {
        let t = cookie_token(r.cookie);
        if t.is_empty() || !token_valid(t.clone()) {
            return json_response(401, "{\"ok\":false,\"error\":\"log in first\"}".to_string());
        }
        let phone = token_phone(t);
        let name = find_user(phone.clone());
        let c = new_challenge("reg".to_string(), phone.clone());
        json_response(200, format!(
            "{{\"ok\":true,\"challenge\":\"{}\",\"rp_id\":\"{}\",\"user_id\":\"{}\",\"user_name\":\"{}\"}}",
            c, rp_id(), b64u_encode(phone.into_bytes()), name))
    }

    fn passkey_register(r: request) -> response {
        let t = cookie_token(r.cookie.clone());
        if t.is_empty() || !token_valid(t.clone()) {
            return json_response(401, "{\"ok\":false,\"error\":\"log in first\"}".to_string());
        }
        let phone = token_phone(t);
        let v: serde_json::Value = serde_json::from_str(&r.body)
            .unwrap_or(serde_json::Value::Null);
        let client_raw = b64u_decode(v["client_data"].as_str().unwrap_or("").to_string());
        let challenge = client_data_challenge(client_raw, "webauthn.create".to_string());
        if challenge.is_empty() {
            return json_response(400, "{\"ok\":false,\"error\":\"bad client data\"}".to_string());
        }
        if take_challenge(challenge, "reg".to_string()) != phone {
            return json_response(403, "{\"ok\":false,\"error\":\"challenge mismatch\"}".to_string());
        }
        let att = b64u_decode(v["attestation"].as_str().unwrap_or("").to_string());
        let pair = parse_attested(attestation_auth_data(att));
        if pair.0.is_empty() || pair.1.is_empty() {
            return json_response(400, "{\"ok\":false,\"error\":\"no credential in attestation\"}".to_string());
        }
        let cred_id = b64u_encode(pair.0);
        if cred_id != v["id"].as_str().unwrap_or("") {
            return json_response(400, "{\"ok\":false,\"error\":\"credential id mismatch\"}".to_string());
        }
        add_passkey(cred_id, hex(pair.1), phone.clone());
        println!("auth: passkey registered for {}", tag(phone));
        json_response(200, "{\"ok\":true}".to_string())
    }

    fn passkey_login_challenge(r: request) -> response {
        let c = new_challenge("login".to_string(), "-".to_string());
        json_response(200, format!(
            "{{\"ok\":true,\"challenge\":\"{}\",\"rp_id\":\"{}\"}}", c, rp_id()))
    }

    fn passkey_login(r: request) -> response {
        let v: serde_json::Value = serde_json::from_str(&r.body)
            .unwrap_or(serde_json::Value::Null);
        let cred_id = v["id"].as_str().unwrap_or("").to_string();
        let rec = find_passkey(cred_id);
        if rec.is_empty() {
            return json_response(403, "{\"ok\":false,\"error\":\"unknown passkey — log in with SMS and enable it\"}".to_string());
        }
        let parts: Vec<&str> = rec.split(' ').collect();
        let xy = unhex(parts[0].to_string());
        let phone = parts[1].to_string();
        let client_raw = b64u_decode(v["client_data"].as_str().unwrap_or("").to_string());
        let challenge = client_data_challenge(client_raw.clone(), "webauthn.get".to_string());
        if challenge.is_empty() {
            return json_response(400, "{\"ok\":false,\"error\":\"bad client data\"}".to_string());
        }
        if take_challenge(challenge, "login".to_string()).is_empty() {
            return json_response(403, "{\"ok\":false,\"error\":\"challenge expired — try again\"}".to_string());
        }
        let auth_data = b64u_decode(v["auth_data"].as_str().unwrap_or("").to_string());
        if auth_data.len() < 37
            || auth_data[0..32].to_vec() != sha256(rp_id().into_bytes())
            || auth_data[32] & 0x05 != 0x05 {
            return json_response(403, "{\"ok\":false,\"error\":\"verification failed\"}".to_string());
        }
        let mut msg = auth_data.clone();
        msg.extend(sha256(client_raw));
        let sig = b64u_decode(v["signature"].as_str().unwrap_or("").to_string());
        if !p256_verify(xy, msg, sig) {
            println!("auth: passkey login {} BAD SIGNATURE", tag(phone));
            return json_response(403, "{\"ok\":false,\"error\":\"signature check failed\"}".to_string());
        }
        println!("auth: passkey login {} OK — cookie issued", tag(phone.clone()));
        let mut resp = json_response(200, "{\"ok\":true}".to_string());
        resp.set_cookie = format!(
            "muon_auth={}; Max-Age=31536000; Path=/; Secure; HttpOnly; SameSite=Lax",
            make_token(phone));
        resp
    }
}
