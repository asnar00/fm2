struct feature_Scope;
impl feature_Scope {
    // client half: generic arrival — any VarUpdate writes into state under its
    // key, so features consuming a Var never handle synchronisation at all.
    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "VarUpdate" {
            return state;
        }
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        let key = e["data"]["key"].as_str().unwrap_or("").to_string();
        if !key.is_empty() {
            s[key] = e["data"]["value"].clone();
        }
        s.to_string()
    }

    // server half: one generic handler for every scoped variable
    fn handle_msg(msg: String) -> String {
        let m: serde_json::Value = serde_json::from_str(&msg)
            .unwrap_or(serde_json::Value::Null);
        let t = m["type"].as_str().unwrap_or("").to_string();
        if t != "VarSet" && t != "VarAdd" {
            return existing.handle_msg(msg);
        }
        let scope = m["data"]["scope"].as_str().unwrap_or("").to_string();
        let key = m["data"]["key"].as_str().unwrap_or("").to_string();
        let from = m["_from"].as_str().unwrap_or("").to_string();
        if key.is_empty() {
            return "{}".to_string();
        }
        if scope == "group" {
            return "{\"ok\":false,\"error\":\"group scope awaits the membership model\"}".to_string();
        }
        let store_key = if scope == "user" {
            format!("user.{}.{}", from, key)
        } else {
            format!("global.{}", key)
        };
        let value = if t == "VarAdd" {
            let cur = var_read(store_key.clone())["v"].as_u64().unwrap_or(0);
            serde_json::json!(cur + m["data"]["value"].as_u64().unwrap_or(0))
        } else {
            m["data"]["value"].clone()
        };
        var_write(store_key, value.clone());
        // the audience string is what /messaging filters waits against
        let audience = if scope == "user" {
            format!("user.{}", from)
        } else {
            "global".to_string()
        };
        let update = serde_json::json!({
            "type": "VarUpdate",
            "data": { "scope": scope, "key": key, "value": value }
        }).to_string();
        publish(audience, update.clone());
        update
    }

    fn var_dir() -> String {
        "/tmp/miso-vars".to_string()
    }

    fn var_read(store_key: String) -> serde_json::Value {
        let raw = std::fs::read_to_string(format!("{}/{}.json", var_dir(), store_key))
            .unwrap_or_default();
        serde_json::from_str(&raw).unwrap_or(serde_json::json!({ "v": null }))
    }

    fn var_write(store_key: String, value: serde_json::Value) {
        let _ = std::fs::create_dir_all(var_dir());
        let _ = std::fs::write(format!("{}/{}.json", var_dir(), store_key),
                               serde_json::json!({ "v": value }).to_string());
    }
}
