struct feature_LoginGuard;
impl feature_LoginGuard {
    // hardened passkey sign-in. Three changes from the base, all under the store
    // lock so the one-time challenge can't be double-spent by concurrent calls:
    //   1. consume the login challenge BEFORE looking up the credential, so a
    //      caller without a live challenge learns nothing about which credential
    //      ids exist (the base answered "unknown passkey" first);
    //   2. re-check the guest list — a passkey whose owner was dropped from
    //      users.json no longer logs in, matching the SMS path;
    //   3. otherwise identical: origin, rpIdHash, UP+UV flags, single-use
    //      challenge, ECDSA-P256 over authData||SHA256(clientData).
    fn passkey_login(r: request) -> response {
        with_store_lock(|| {
            let v: serde_json::Value = serde_json::from_str(&r.body)
                .unwrap_or(serde_json::Value::Null);
            let client_raw = b64u_decode(v["client_data"].as_str().unwrap_or("").to_string());
            let challenge = client_data_challenge(client_raw.clone(), "webauthn.get".to_string());
            if challenge.is_empty() {
                return json_response(400, "{\"ok\":false,\"error\":\"bad client data\"}".to_string());
            }
            if take_challenge(challenge, "login".to_string()).is_empty() {
                return json_response(403, "{\"ok\":false,\"error\":\"challenge expired — try again\"}".to_string());
            }
            let cred_id = v["id"].as_str().unwrap_or("").to_string();
            let rec = find_passkey(cred_id);
            if rec.is_empty() {
                return json_response(403, "{\"ok\":false,\"error\":\"unknown passkey — log in with SMS and enable it\"}".to_string());
            }
            let parts: Vec<&str> = rec.split(' ').collect();
            let xy = unhex(parts[0].to_string());
            let phone = parts[1].to_string();
            if find_user(phone.clone()).is_empty() {
                println!("auth: passkey login {} NO LONGER A GUEST", tag(phone));
                return json_response(403, "{\"ok\":false,\"error\":\"this account is no longer active\"}".to_string());
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
                "miso_auth={}; Max-Age=31536000; Path=/; Secure; HttpOnly; SameSite=Lax",
                make_token(phone));
            resp
        })
    }
}
