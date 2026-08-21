struct feature_WholeNumber;
impl feature_WholeNumber {
    // who a request is from. The identity is the WHOLE number, spelled exactly
    // as rung 5 spells it for the context table (`phone:+44…`), so a person has
    // one key everywhere on the server. The four-digit answer this replaces
    // collided: two guests whose numbers end the same were one person to every
    // caller of this function.
    fn sender_of(cookie: String) -> String {
        let t = cookie_token(cookie);
        if !t.is_empty() && token_valid(t.clone()) {
            format!("phone:{}", token_phone(t))
        } else {
            String::new()
        }
    }

    // the relay names people by an opaque token, not by the identity. The
    // broadcast slot is a shared file that outlives the request and is read by
    // whatever else runs on the box, so it carries a per-user id that reveals
    // nothing and cannot be guessed from a phone number: HMAC under the
    // session-signing secret, truncated to 128 bits.
    fn sender_audience(who: String) -> String {
        hmac_sha256(secret(), format!("audience:{}", who))
            .chars()
            .take(32)
            .collect()
    }

    // one audience string, translated. `global` and anything else that is not a
    // person passes through untouched.
    fn opaque_audience(audience: String) -> String {
        match audience.strip_prefix("user.") {
            Some(who) if !who.is_empty() => {
                format!("user.{}", sender_audience(who.to_string()))
            }
            _ => audience,
        }
    }

    // the two ends of the relay, translated in one place: the writer addresses
    // an entry, the reader decides what it may hear. Callers keep building
    // `user.<identity>` and never learn there is a token.
    fn publish(audience: String, msg: String) {
        existing.publish(opaque_audience(audience), msg)
    }

    fn wait_filter(b: serde_json::Value, since: u64, me: String) -> String {
        if me.is_empty() {
            return existing.wait_filter(b, since, me);
        }
        existing.wait_filter(b, since, sender_audience(me))
    }
}
