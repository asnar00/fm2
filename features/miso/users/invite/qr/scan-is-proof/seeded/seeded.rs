struct feature_Seeded;
impl feature_Seeded {
    // the login the scan is: seed the newcomer with the inviter's cards, the
    // act /exchange does after a texted login. The key is read off the cookie
    // — the phone in the body may be empty (/name-only) and the cookie is the
    // identity the server actually issued.
    fn qr_claim(r: request) -> response {
        let resp = existing.qr_claim(r);
        if resp.status != 200 || resp.set_cookie.is_empty() {
            return resp;
        }
        let digits = seeded_digits(resp.set_cookie.clone());
        if !digits.is_empty() {
            exchange_seed(format!("phone:+{}", digits));
        }
        resp
    }

    // "miso_auth=<digits>.<issued>.<expiry>.<hmac>; …" -> the digits
    fn seeded_digits(cookie: String) -> String {
        let rest = match cookie.strip_prefix("miso_auth=") {
            Some(s) => s.to_string(),
            None => {
                return String::new();
            }
        };
        let head: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
        head
    }
}
