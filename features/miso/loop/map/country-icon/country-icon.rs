struct feature_CountryIcon;
impl feature_CountryIcon {
    // the page half works out which country the fix falls in and reports it;
    // one code, not an outline — state carries facts, never drawings
    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "CountryFound" {
            return state;
        }
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        let cc = e["data"]["code"].as_str().unwrap_or("").to_string();
        if cc.is_empty() {
            if let Some(o) = s.as_object_mut() {
                o.remove("map_country");
            }
        } else {
            s["map_country"] = serde_json::json!(cc);
        }
        s.to_string()
    }

    // swap the map tool's emoji for a placeholder the page half fills with
    // the country's outline; unknown country keeps /map's 🗺
    fn tools_list(state: String) -> String {
        let prev = existing.tools_list(state.clone());
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        let cc = s["map_country"].as_str().unwrap_or("").to_string();
        if cc.is_empty() {
            return prev;
        }
        let mut list: serde_json::Value = serde_json::from_str(&prev)
            .unwrap_or(serde_json::json!([]));
        if let Some(arr) = list.as_array_mut() {
            for t in arr.iter_mut() {
                if t["id"].as_str() == Some("map") {
                    t["icon"] = serde_json::json!(
                        format!("<span class=\"cc\" data-cc=\"{}\"></span>", cc));
                }
            }
        }
        list.to_string()
    }
}
