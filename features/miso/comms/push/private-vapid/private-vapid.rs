struct feature_PrivateVapid;
impl feature_PrivateVapid {
    // the VAPID signing key must be owner-only, the same rule /harden applies to
    // the session secret. Whoever reads it can send push notifications as the
    // campaign; the base wrote it 0644. Tightened on every read, so a fresh key
    // is born private and an old loose one is repaired.
    fn vapid_secret() -> Vec<u8> {
        let s = existing.vapid_secret();
        fm_own_only(&format!("{}/vapid-secret", auth_dir()));
        s
    }
}
