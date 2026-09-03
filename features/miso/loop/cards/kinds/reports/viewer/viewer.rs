struct feature_Viewer;
impl feature_Viewer {
    // the page /reports printed from is kept under the report's own id, so
    // the view route can serve THIS report's page and not the last one printed
    fn reports_generate(who: String, id: String) {
        existing.reports_generate(who.clone(), id.clone());
        let card = reports_card_of(who.clone(), id.clone());
        if card.is_null() {
            return;
        }
        let st = reports_state_of(&card);
        if st["status"].as_str().unwrap_or("") != "ready" {
            return;
        }
        let dir = reports_dir(who);
        let page = format!("{}/work/report.html", dir);
        let kept = format!("{}/{}.html", dir, reports_safe(id));
        let _ = std::fs::copy(page, kept);
    }

    fn route(r: request) -> response {
        if r.path == "reports/view" && r.method == "GET" {
            return viewer_route(r);
        }
        existing.route(r)
    }

    // the PDF route's own checks, then the kept page
    fn viewer_route(r: request) -> response {
        let who = reports_caller(&r);
        if !reports_allowed(who.clone()) {
            return reports_deny();
        }
        let id = reports_query_id(r.query.clone());
        if !reports_id_ok(&id) {
            return json_response(400, "{\"ok\":false,\"error\":\"bad id\"}".to_string());
        }
        if reports_card_of(who.clone(), id.clone()).is_null() {
            return json_response(404, "{\"ok\":false,\"error\":\"no such report\"}".to_string());
        }
        let file = format!("{}/{}.html", reports_dir(who), reports_safe(id));
        match std::fs::read(file) {
            Ok(bytes) => response { status: 200, ctype: "text/html; charset=utf-8".to_string(),
                                    body: bytes, set_cookie: String::new(),
                                    cache: "no-store".to_string() },
            Err(_) => json_response(404,
                "{\"ok\":false,\"error\":\"there is no page for that report yet\"}".to_string()),
        }
    }
}
