struct feature_Pretend;
impl feature_Pretend {
    // an admin may invite a pretend person — a `_` name whose codes go to
    // the server log — because the admin can read that log anyway. Support
    // still cannot: the hole /invite closed stays closed for them.
    fn invite_name_ok(name: String, who: String) -> String {
        if name.starts_with('_') && authority_rank(who.clone()) >= 3 {
            return String::new();
        }
        existing.invite_name_ok(name, who)
    }
}
