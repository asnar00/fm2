struct feature_Map;
impl feature_Map {
    fn tools_list(state: String) -> String {
        let prev = existing.tools_list(state);
        let mut list: serde_json::Value = serde_json::from_str(&prev)
            .unwrap_or(serde_json::json!([]));
        if let Some(arr) = list.as_array_mut() {
            arr.push(serde_json::json!({ "id": "map", "label": "map", "icon": "🗺" }));
        }
        list.to_string()
    }

    // intent lives in state; the page half watches open_tool and drives the
    // sensor, reporting readings back as events (the /dictate pattern)
    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        let t = e["type"].as_str().unwrap_or("").to_string();
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        if t == "Located" {
            s["map_fix"] = e["data"].clone();
            if let Some(o) = s.as_object_mut() {
                o.remove("map_error");
            }
            return s.to_string();
        }
        if t == "LocateFailed" {
            s["map_error"] = e["data"]["err"].clone();
            return s.to_string();
        }
        if t == "click" && e["ev"].as_str().unwrap_or("") == "map_again" {
            if let Some(o) = s.as_object_mut() {
                o.remove("map_error");
                o.remove("map_fix");
            }
            return s.to_string();
        }
        state
    }

    fn render(state: String) -> String {
        let base = existing.render(state.clone());
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        if s["open_tool"].as_str().unwrap_or("") != "map" {
            return base;
        }
        format!("{}{}", base, map_view(state))
    }

    fn tool_controls(state: String) -> String {
        let prev = existing.tool_controls(state.clone());
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        if s["open_tool"].as_str().unwrap_or("") != "map" {
            return prev;
        }
        format!("{}<div class=\"tool-button ctrl\" data-ev=\"map_again\">⟳</div>", prev)
    }

    // the ground distance from the centre to the outer ring: the first step
    // that comfortably contains the fix's own uncertainty, so the picture is
    // always usefully filled and never claims more precision than we have
    fn map_span(acc: f64) -> f64 {
        let steps = [25.0, 50.0, 100.0, 250.0, 500.0, 1000.0];
        for step in steps.iter() {
            if acc * 2.0 <= *step {
                return *step;
            }
        }
        1000.0
    }

    fn map_metres(m: f64) -> String {
        if m >= 1000.0 {
            return format!("{:.0} km", m / 1000.0);
        }
        format!("{:.0} m", m)
    }

    fn map_view(state: String) -> String {
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        if let Some(err) = s["map_error"].as_str() {
            let safe = err.replace('&', "&amp;").replace('<', "&lt;");
            return format!("<div class=\"map-view\"><div class=\"map-msg\">{}</div>\
                <div class=\"map-msg dim\">a map that guesses where you are \
                would be worse than none</div></div>", safe);
        }
        let fix = s["map_fix"].clone();
        if fix.is_null() {
            return "<div class=\"map-view\"><div class=\"map-msg\">finding you…</div>\
                </div>".to_string();
        }
        let lat = fix["lat"].as_f64().unwrap_or(0.0);
        let lon = fix["lon"].as_f64().unwrap_or(0.0);
        let acc = fix["acc"].as_f64().unwrap_or(0.0);
        let span = map_span(acc);
        // the accuracy disc shares the rings' scale, so the drawing is
        // internally consistent: one number sets every radius on screen
        let mut disc = acc / span * 100.0;
        if disc > 100.0 {
            disc = 100.0;
        }
        format!("<div class=\"map-view\"><div class=\"map-field\">\
            <div class=\"map-north\">N</div>\
            <div class=\"map-ring r3\"><span>{}</span></div>\
            <div class=\"map-ring r2\"><span>{}</span></div>\
            <div class=\"map-ring r1\"><span>{}</span></div>\
            <div class=\"map-acc\" style=\"width:{:.1}%;height:{:.1}%\"></div>\
            <div class=\"map-me\"></div></div>\
            <div class=\"map-read\">{:.5}, {:.5} &middot; &plusmn;{}</div></div>",
            map_metres(span), map_metres(span * 2.0 / 3.0), map_metres(span / 3.0),
            disc, disc, lat, lon, map_metres(acc))
    }
}
