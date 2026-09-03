struct feature_NameOnly;
impl feature_NameOnly {
    // no number typed: the claim goes on with a placeholder number the list
    // cannot mistake for a phone, and /scan-is-proof logs the device in. A
    // typed number is left alone.
    fn qr_claim(r: request) -> response {
        let v: serde_json::Value = serde_json::from_str(&r.body)
            .unwrap_or(serde_json::Value::Null);
        let name = v["name"].as_str().unwrap_or("").trim().to_string();
        let phone = normalise_phone(v["phone"].as_str().unwrap_or("").to_string());
        if !phone.is_empty() || name.is_empty() || !v.is_object() {
            return existing.qr_claim(r);
        }
        let mut body = v.clone();
        body["phone"] = serde_json::json!(name_only_number());
        let mut sent = r;
        sent.body = body.to_string();
        println!("qr: a name-only claim — placeholder number minted");
        existing.qr_claim(sent)
    }

    // /instant's scheme: +9 and sixteen digits — seventeen, past E.164's
    // fifteen, so never a real phone; the last four kept clear of every
    // entry's last four.
    fn name_only_number() -> String {
        let list = invite_list();
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut taken: Vec<String> = Vec::new();
        for u in list.as_array().unwrap_or(&empty) {
            let p = normalise_phone(u["phone"].as_str().unwrap_or("").to_string());
            if p.len() >= 4 {
                taken.push(p[p.len() - 4..].to_string());
            }
        }
        let mut tries = 0;
        loop {
            let bytes = random_bytes(17);
            let mut digits = String::from("9");
            for b in bytes.iter().skip(1) {
                digits = format!("{}{}", digits, (b % 10).to_string());
            }
            let last4 = digits[digits.len() - 4..].to_string();
            tries = tries + 1;
            if !taken.contains(&last4) || tries > 50 {
                return format!("+{}", digits);
            }
        }
    }
}
