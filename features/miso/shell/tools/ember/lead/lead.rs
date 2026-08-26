struct feature_Lead;
impl feature_Lead {
    // the launcher's default order: the things a campaign does first —
    // projects, posts, people — lead; everything else follows in the order
    // it registered. A person's own order (/reorder), being newer, sits
    // outside this and wins.
    fn tools_list(state: String) -> String {
        let prev = existing.tools_list(state);
        let list: serde_json::Value = serde_json::from_str(&prev)
            .unwrap_or(serde_json::json!([]));
        let empty: Vec<serde_json::Value> = Vec::new();
        let all = list.as_array().unwrap_or(&empty);
        let lead = ["projects", "posts", "account"];
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
        serde_json::Value::Array(out).to_string()
    }
}
