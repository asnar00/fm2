struct feature_OnlyWhenMissing;
impl feature_OnlyWhenMissing {
    // the row is an offer to fill a gap, so it is drawn only when there is
    // one. `/add-number`'s slot state answers with the ALIAS the account has
    // recorded — empty for everybody who never added one, including everyone
    // whose real number is on the guest list already — and its own `me_under`
    // draws the field whenever that answer is `ok`. So the offer stood on the
    // card of every person the campaign can already text.
    //
    // The seam is the answer itself: `me_under` draws nothing when the slot
    // says `ok: false`, so this link says exactly that for an account whose
    // number we hold. No client code, and unticking it puts the row back.
    fn addnum_state(r: request) -> response {
        let cookie = r.cookie.clone();
        let base = existing.addnum_state(r);
        if base.status != 200 {
            return base;   // not logged in: the base has already refused
        }
        let text = String::from_utf8(base.body.clone()).unwrap_or_default();
        let v: serde_json::Value = serde_json::from_str(&text)
            .unwrap_or(serde_json::Value::Null);
        if !v["ok"].as_bool().unwrap_or(false) {
            return base;
        }
        if !v["number"].as_str().unwrap_or("").is_empty() {
            // a number they added: the row is showing it back to them, which
            // is not an offer and stays
            return base;
        }
        if onlywhen_no_number(addnum_who(cookie)) {
            return base;
        }
        json_response(200, serde_json::json!({
            "ok": false,
            "why": "your number is already on the list"
        }).to_string())
    }

    // do we not have this person's number? Their world key IS their number on
    // the guest list, and a name-only scan-in was given a placeholder instead
    // of one: `/name-only` mints `+9` and sixteen digits — seventeen, two past
    // E.164's cap of fifteen — so length alone answers it. That is
    // `/instant`'s `instant_is_synthetic` rule, restated here because this
    // product does not compose `/instant` (unticked), which is the same reason
    // `/name-only` restates the mint.
    fn onlywhen_no_number(key: String) -> bool {
        let digits: String = key.chars().filter(|c| c.is_ascii_digit()).collect();
        digits.len() > 15
    }
}
