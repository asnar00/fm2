struct feature_ByKey;
impl feature_ByKey {
    fn context_user_of(token: String, tunnel: bool, query: String) -> String {
        let prev = existing.context_user_of(token.clone(), tunnel, query.clone());
        if !prev.is_empty() || tunnel {
            return prev;
        }
        let name = query_param(query, "user".to_string());
        if !name.contains(':') {
            return prev;
        }
        // the raw-key alphabet: what /remember's filename encoding accepts,
        // plus the two chars real keys carry (`:` and `+`)
        let ok = !name.is_empty() && name.len() <= 64
            && name.chars().all(|c| c.is_ascii_alphanumeric()
                                || c == '-' || c == '.' || c == '_'
                                || c == ':' || c == '+');
        if ok { name } else { prev }
    }
}
