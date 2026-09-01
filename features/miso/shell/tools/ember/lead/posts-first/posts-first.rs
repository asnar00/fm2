struct feature_PostsFirst;
impl feature_PostsFirst {
    // the re-ruled default: posts, people, reports, projects lead; the rest
    // follow in registration order. A person's own arrangement still wins.
    fn tools_list(state: String) -> String {
        let prev = existing.tools_list(state);
        if tools_order_chosen() {
            return prev;
        }
        let list: serde_json::Value = serde_json::from_str(&prev)
            .unwrap_or(serde_json::json!([]));
        let empty: Vec<serde_json::Value> = Vec::new();
        let all = list.as_array().unwrap_or(&empty);
        let lead = ["posts", "account", "reports", "projects"];
        let mut out: Vec<serde_json::Value> = Vec::new();
        for id in lead.iter() {
            for t in all {
                if t["id"].as_str() == Some(id) {
                    out.push(t.clone());
                }
            }
        }
        for t in all {
            if !lead.contains(&t["id"].as_str().unwrap_or("")) {
                out.push(t.clone());
            }
        }
        serde_json::json!(out).to_string()
    }
}
