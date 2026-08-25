struct feature_CodeGuard;
impl feature_CodeGuard {
    // six digits, drawn uniformly. The base took 4 digits as `u16 % 10000`,
    // which is modulo-biased (low codes ~2x as likely) and only 10k wide; this
    // rejects the unrepresentable tail of a u32 so every code is equally likely,
    // and 1e6 codes means the 15-guesses-an-hour ceiling is negligible.
    fn make_pin() -> String {
        loop {
            let b = random_bytes(4);
            let n = ((b[0] as u32) << 24) | ((b[1] as u32) << 16)
                | ((b[2] as u32) << 8) | (b[3] as u32);
            if n < 4294000000u32 {
                return format!("{:06}", n % 1000000);
            }
        }
    }

    // the base answered "not on the guest list" with a 403 and echoed the
    // member's NAME on success — an unauthenticated oracle that turns a phone
    // book into the campaign's membership list. Now every request gets the same
    // opaque 200: a code is sent only to a real guest under the rate limit, and
    // nothing in the reply distinguishes a member from a stranger. The whole
    // thing runs under the store lock so the pending/sends files can't be raced.
    fn auth_request(r: request) -> response {
        with_store_lock(|| {
            let v: serde_json::Value = serde_json::from_str(&r.body)
                .unwrap_or(serde_json::Value::Null);
            let phone = normalise_phone(v["phone"].as_str().unwrap_or("").to_string());
            let name = find_user(phone.clone());
            if !name.is_empty() && sms_count_last_hour(phone.clone()) < 5 {
                let pin = make_pin();
                save_pending(phone.clone(), pin.clone());
                if name.starts_with("_") {
                    println!("auth: test user {} pin {}", name, pin);
                } else {
                    let err = send_sms(phone.clone(), format!("miso login code: {}", pin));
                    if err.is_empty() {
                        record_sms(phone.clone());
                        println!("auth: sms sent {}", tag(phone.clone()));
                    } else {
                        clear_pending(phone.clone());
                        println!("auth: sms send failed: {}", err);
                    }
                }
            } else {
                println!("auth: request {} -> no code (unknown or throttled)", tag(phone));
            }
            json_response(200, "{\"ok\":true}".to_string())
        })
    }

    // verify under the lock: the base read the attempt counter, checked, then
    // wrote counter+1 as three steps, so two concurrent guesses both saw 0 and
    // the 3-strike limit never bit. Serialised, load-check-increment is atomic.
    fn auth_verify(r: request) -> response {
        with_store_lock(|| existing.auth_verify(r))
    }
}
