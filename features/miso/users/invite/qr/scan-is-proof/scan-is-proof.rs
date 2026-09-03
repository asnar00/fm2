struct feature_ScanIsProof;
impl feature_ScanIsProof {
    // the base claim is the whole invite — token, shape, the guest-list row,
    // the spent use. This link only decides what the answer carries: for a
    // number the list did not hold a moment ago, the session cookie /auth/verify
    // would have issued, so the scan is the login. A number already on the list
    // keeps the base's answer and the PIN road (the code proves the canvasser
    // let you in, not that you own somebody's number).
    fn qr_claim(r: request) -> response {
        let v: serde_json::Value = serde_json::from_str(&r.body)
            .unwrap_or(serde_json::Value::Null);
        let phone = normalise_phone(v["phone"].as_str().unwrap_or("").to_string());
        let seen = scan_is_proof_seen(phone.clone());
        let mut resp = existing.qr_claim(r);
        if resp.status != 200 || seen || phone.is_empty() {
            return resp;
        }
        // the base held the store lock and let it go; the stamp takes it again
        invite_stamp_joined(phone.clone());
        println!("qr: {} is in on the scan — cookie issued", tag(phone.clone()));
        resp.body = "{\"ok\":true,\"in\":true}".to_string().into_bytes();
        resp.set_cookie = format!(
            "miso_auth={}; Max-Age=31536000; Path=/; Secure; HttpOnly; SameSite=Lax",
            make_token(phone));
        resp
    }

    // was this number on the guest list before the claim? Read outside the
    // lock: the base takes it and the lock is not reentrant. An unreadable
    // list counts as seen — the base refuses such a claim anyway, and nothing
    // here may hand out a cookie on a list it could not read.
    fn scan_is_proof_seen(phone: String) -> bool {
        let list = invite_list();
        let arr = match list.as_array() {
            Some(a) => a.clone(),
            None => {
                return true;
            }
        };
        for u in arr.iter() {
            let up = normalise_phone(u["phone"].as_str().unwrap_or("").to_string());
            if !up.is_empty() && up == phone {
                return true;
            }
        }
        false
    }
}
