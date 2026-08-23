struct feature_Harden;
impl feature_Harden {
    // the epoch below which every session is dead. `~/.miso-auth/revoked-before`
    // holds a millisecond timestamp; bump it (`date +%s000`) to log everyone out
    // at once without discarding the signing key. Absent/blank = 0 = nothing
    // revoked.
    fn revoked_before() -> u64 {
        std::fs::read_to_string(format!("{}/revoked-before", auth_dir()))
            .unwrap_or_default().trim().parse().unwrap_or(0)
    }

    // token gains an issued-at: "<digits>.<issued>.<expiry>.<hmac>". The issued
    // stamp is what the revocation epoch compares against; the year-long expiry
    // is unchanged.
    fn make_token(phone: String) -> String {
        let issued = now_ms();
        let exp = issued + 31536000000u64;
        let payload = format!("{}.{}.{}", phone.replace("+", ""), issued, exp);
        format!("{}.{}", payload, hmac_sha256(secret(), payload.clone()))
    }

    // validity is now HMAC + not-expired + issued-after-the-epoch + STILL A
    // GUEST. The last check is the point: dropping someone from users.json cuts
    // their live sessions the same request, where before a stolen or ex-member
    // cookie stayed good for a year.
    fn token_valid(token: String) -> bool {
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 4 {
            return false;
        }
        let issued: u64 = parts[1].parse().unwrap_or(0);
        let exp: u64 = parts[2].parse().unwrap_or(0);
        if exp < now_ms() {
            return false;
        }
        if issued < revoked_before() {
            return false;
        }
        let payload = format!("{}.{}.{}", parts[0], parts[1], parts[2]);
        if !constant_eq(hmac_sha256(secret(), payload), parts[3].to_string()) {
            return false;
        }
        !find_user(format!("+{}", parts[0])).is_empty()
    }

    fn token_phone(token: String) -> String {
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() == 4 {
            format!("+{}", parts[0])
        } else {
            String::new()
        }
    }

    // the signing key must be owner-only. The base writes it 0644; this tightens
    // it to 0600 on every read, so a fresh key is born private and an old loose
    // one is repaired in place.
    fn secret() -> Vec<u8> {
        let s = existing.secret();
        fm_own_only(&format!("{}/secret", auth_dir()));
        s
    }

    // a short or failed urandom read must be an error, never silent zeros: the
    // base ignored the result, so a failure would have handed out an
    // all-zero PIN, secret, or challenge.
    fn random_bytes(n: usize) -> Vec<u8> {
        use std::io::Read;
        let mut buf = vec![0u8; n];
        let mut f = std::fs::File::open("/dev/urandom").expect("miso: no /dev/urandom");
        f.read_exact(&mut buf).expect("miso: urandom short read");
        buf
    }
}
