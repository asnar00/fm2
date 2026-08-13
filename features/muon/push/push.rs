struct feature_Push;
impl feature_Push {
    // deploy notifications need no deploy-script coupling: the server restarts
    // on every deploy, so on startup we compare site/version with the last
    // build we announced, and push to every subscribed device if it moved.
    fn serve() {
        notify_if_updated();
        existing.serve();
    }

    fn route(r: request) -> response {
        if r.path == "push/vapid-key" {
            return vapid_key_response(r);
        }
        if r.path == "push/subscribe" && r.method == "POST" {
            return push_subscribe(r);
        }
        existing.route(r)
    }

    // ---- VAPID keypair (generated once, kept beside the auth state)

    fn vapid_secret() -> Vec<u8> {
        let file = format!("{}/vapid-secret", auth_dir());
        match std::fs::read(&file) {
            Ok(bytes) => bytes,
            Err(_) => {
                let mut bytes = random_bytes(32);
                while p256::SecretKey::from_slice(&bytes).is_err() {
                    bytes = random_bytes(32);
                }
                let _ = std::fs::create_dir_all(auth_dir());
                let _ = std::fs::write(&file, &bytes);
                bytes
            }
        }
    }

    fn vapid_public_b64u() -> String {
        use p256::elliptic_curve::sec1::ToEncodedPoint;
        let secret = p256::SecretKey::from_slice(&vapid_secret()).expect("vapid key");
        let point = secret.public_key().to_encoded_point(false);
        b64u_encode(point.as_bytes().to_vec())
    }

    fn vapid_key_response(r: request) -> response {
        response { status: 200, ctype: "text/plain".to_string(),
                   body: vapid_public_b64u().into_bytes(),
                   set_cookie: String::new(), cache: "no-store".to_string() }
    }

    // ---- subscriptions: "endpoint p256dh_b64u auth_b64u phone" per line

    fn subs_file() -> String {
        format!("{}/push-subs.txt", auth_dir())
    }

    fn push_subscribe(r: request) -> response {
        let t = cookie_token(r.cookie.clone());
        if t.is_empty() || !token_valid(t.clone()) {
            return json_response(401, "{\"ok\":false,\"error\":\"log in first\"}".to_string());
        }
        let phone = token_phone(t);
        let v: serde_json::Value = serde_json::from_str(&r.body)
            .unwrap_or(serde_json::Value::Null);
        let endpoint = v["endpoint"].as_str().unwrap_or("").to_string();
        let p256dh = v["p256dh"].as_str().unwrap_or("").to_string();
        let auth = v["auth"].as_str().unwrap_or("").to_string();
        if endpoint.is_empty() || p256dh.is_empty() || auth.is_empty() {
            return json_response(400, "{\"ok\":false,\"error\":\"incomplete subscription\"}".to_string());
        }
        upsert_sub(endpoint, format!("{} {} {}", p256dh, auth, phone));
        println!("push: subscription stored for {}", tag(phone));
        json_response(200, "{\"ok\":true}".to_string())
    }

    // replaces any existing line for the same endpoint
    fn upsert_sub(endpoint: String, rest: String) {
        let raw = std::fs::read_to_string(subs_file()).unwrap_or_default();
        let mut out = String::new();
        for line in raw.lines() {
            if !line.starts_with(&format!("{} ", endpoint)) && !line.is_empty() {
                out = format!("{}{}\n", out, line);
            }
        }
        if !rest.is_empty() {
            out = format!("{}{} {}\n", out, endpoint, rest);
        }
        let _ = std::fs::create_dir_all(auth_dir());
        let _ = std::fs::write(subs_file(), out);
    }

    fn remove_sub(endpoint: String) {
        upsert_sub(endpoint, String::new());
    }

    // ---- the deploy announcement

    fn notify_if_updated() {
        let version = std::fs::read_to_string("site/version")
            .unwrap_or_default().trim().to_string();
        if version.is_empty() {
            return;
        }
        let marker = format!("{}/last-notified", auth_dir());
        let last = std::fs::read_to_string(&marker).unwrap_or_default().trim().to_string();
        if last == version {
            return;
        }
        let _ = std::fs::create_dir_all(auth_dir());
        let _ = std::fs::write(&marker, &version);
        if last.is_empty() {
            return; // first run under this feature: nothing to announce yet
        }
        let payload = format!(
            "{{\"title\":\"muon\",\"body\":\"updated to build {}{}\"}}",
            version, latest_change());
        send_all(payload);
    }

    fn latest_change() -> String {
        let raw = std::fs::read_to_string("site/changes.json").unwrap_or_default();
        let v: serde_json::Value = serde_json::from_str(&raw)
            .unwrap_or(serde_json::Value::Null);
        let text = v[0]["text"].as_str().unwrap_or("").replace("\"", "").replace("\\", "");
        if text.is_empty() {
            String::new()
        } else {
            format!(" — {}", text)
        }
    }

    fn send_all(payload: String) {
        let raw = std::fs::read_to_string(subs_file()).unwrap_or_default();
        for line in raw.lines() {
            let parts: Vec<&str> = line.split(' ').collect();
            if parts.len() != 4 {
                continue;
            }
            let status = send_push(parts[0].to_string(), parts[1].to_string(),
                                   parts[2].to_string(), payload.clone());
            println!("push: {} -> {} (status {})", tag(parts[3].to_string()),
                     endpoint_origin(parts[0].to_string()), status);
            if status == 404 || status == 410 {
                remove_sub(parts[0].to_string()); // subscription expired
            }
        }
    }

    // ---- web push protocol: VAPID auth + RFC 8291 aes128gcm encryption

    fn send_push(endpoint: String, p256dh: String, auth: String, payload: String) -> u16 {
        let body = encrypt_payload(b64u_decode(p256dh), b64u_decode(auth),
                                   payload.into_bytes());
        if body.is_empty() {
            return 0;
        }
        let tmp = format!("/tmp/muon-push-{}.bin", now_ms());
        let _ = std::fs::write(&tmp, &body);
        let jwt = vapid_jwt(endpoint_origin(endpoint.clone()));
        let out = std::process::Command::new("curl")
            .arg("-s").arg("-o").arg("/dev/null").arg("-w").arg("%{http_code}")
            .arg("-X").arg("POST").arg(&endpoint)
            .arg("-H").arg(format!("Authorization: vapid t={}, k={}", jwt, vapid_public_b64u()))
            .arg("-H").arg("TTL: 3600")
            .arg("-H").arg("Content-Encoding: aes128gcm")
            .arg("-H").arg("Content-Type: application/octet-stream")
            .arg("--data-binary").arg(format!("@{}", tmp))
            .output();
        let _ = std::fs::remove_file(&tmp);
        match out {
            Ok(o) => String::from_utf8_lossy(&o.stdout).trim().parse().unwrap_or(0),
            Err(_) => 0,
        }
    }

    // "https://web.push.apple.com/xyz" -> "https://web.push.apple.com"
    fn endpoint_origin(endpoint: String) -> String {
        let after = match endpoint.find("://") {
            Some(i) => i + 3,
            None => return endpoint,
        };
        match endpoint[after..].find('/') {
            Some(i) => endpoint[..after + i].to_string(),
            None => endpoint,
        }
    }

    fn vapid_jwt(aud: String) -> String {
        use p256::ecdsa::signature::Signer;
        let header = b64u_encode(b"{\"typ\":\"JWT\",\"alg\":\"ES256\"}".to_vec());
        let claims = b64u_encode(format!(
            "{{\"aud\":\"{}\",\"exp\":{},\"sub\":\"mailto:ash.nehru@gmail.com\"}}",
            aud, now_ms() / 1000 + 43200).into_bytes());
        let msg = format!("{}.{}", header, claims);
        let key = p256::ecdsa::SigningKey::from_slice(&vapid_secret()).expect("vapid key");
        let sig: p256::ecdsa::Signature = key.sign(msg.as_bytes());
        format!("{}.{}", msg, b64u_encode(sig.to_bytes().to_vec()))
    }

    // RFC 8291: ECDH(P-256) -> HKDF-SHA256 -> AES-128-GCM, aes128gcm framing
    fn encrypt_payload(ua_pub: Vec<u8>, auth: Vec<u8>, plaintext: Vec<u8>) -> Vec<u8> {
        use p256::elliptic_curve::sec1::ToEncodedPoint;
        use aes_gcm::KeyInit;
        use aes_gcm::aead::Aead;
        if ua_pub.len() != 65 || auth.len() != 16 {
            return Vec::new();
        }
        let ua_key = match p256::PublicKey::from_sec1_bytes(&ua_pub) {
            Ok(k) => k,
            Err(_) => return Vec::new(),
        };
        let mut eph_bytes = random_bytes(32);
        while p256::SecretKey::from_slice(&eph_bytes).is_err() {
            eph_bytes = random_bytes(32);
        }
        let eph = p256::SecretKey::from_slice(&eph_bytes).expect("eph key");
        let as_pub = eph.public_key().to_encoded_point(false).as_bytes().to_vec();
        let shared = p256::ecdh::diffie_hellman(eph.to_nonzero_scalar(), ua_key.as_affine());
        let ecdh_secret = shared.raw_secret_bytes().to_vec();
        let mut info = b"WebPush: info\x00".to_vec();
        info.extend(ua_pub.clone());
        info.extend(as_pub.clone());
        let ikm = hkdf_bytes(auth, ecdh_secret, info, 32);
        let salt = random_bytes(16);
        let cek = hkdf_bytes(salt.clone(), ikm.clone(),
                             b"Content-Encoding: aes128gcm\x00".to_vec(), 16);
        let nonce = hkdf_bytes(salt.clone(), ikm,
                               b"Content-Encoding: nonce\x00".to_vec(), 12);
        let cipher = match aes_gcm::Aes128Gcm::new_from_slice(&cek) {
            Ok(c) => c,
            Err(_) => return Vec::new(),
        };
        let mut padded = plaintext;
        padded.push(0x02); // last-record delimiter
        let ct = match cipher.encrypt(aes_gcm::Nonce::from_slice(&nonce),
                                      padded.as_slice()) {
            Ok(c) => c,
            Err(_) => return Vec::new(),
        };
        // header: salt(16) | record size 4096 (4, BE) | keyid len | as_pub | ct
        let mut out = salt;
        out.extend(vec![0u8, 0u8, 16u8, 0u8]);
        out.push(65u8);
        out.extend(as_pub);
        out.extend(ct);
        out
    }

    fn hkdf_bytes(salt: Vec<u8>, ikm: Vec<u8>, info: Vec<u8>, n: usize) -> Vec<u8> {
        let hk = hkdf::Hkdf::<sha2::Sha256>::new(Some(&salt), &ikm);
        let mut okm = vec![0u8; n];
        let _ = hk.expand(&info, &mut okm);
        okm
    }
}
