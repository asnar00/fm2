struct feature_Vonage;
impl feature_Vonage {
    // real SMS via the Vonage account configured on the mini
    // (~/.agent-config.json: { "vonage": { "api_key", "api_secret", "from_number" } }).
    // TLS is curl's problem, not ours. no creds -> console fallback (the base chain).
    fn send_sms(to: String, text: String) -> String {
        let cfg_raw = std::fs::read_to_string(
            format!("{}/.agent-config.json", std::env::var("HOME").unwrap_or_default()))
            .unwrap_or_default();
        let cfg: serde_json::Value = serde_json::from_str(&cfg_raw)
            .unwrap_or(serde_json::Value::Null);
        let key = cfg["vonage"]["api_key"].as_str().unwrap_or("").to_string();
        let sec = cfg["vonage"]["api_secret"].as_str().unwrap_or("").to_string();
        let from = cfg["vonage"]["from_number"].as_str().unwrap_or("noob").to_string();
        if key.is_empty() || sec.is_empty() {
            return existing.send_sms(to, text);
        }
        let out = std::process::Command::new("curl")
            .arg("-s")
            .arg("https://rest.nexmo.com/sms/json")
            .arg("--data-urlencode").arg(format!("api_key={}", key))
            .arg("--data-urlencode").arg(format!("api_secret={}", sec))
            .arg("--data-urlencode").arg(format!("from={}", from))
            .arg("--data-urlencode").arg(format!("to={}", to.replace("+", "")))
            .arg("--data-urlencode").arg(format!("text={}", text))
            .output();
        let stdout = match out {
            Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(),
            Err(e) => return format!("curl failed: {}", e),
        };
        let v: serde_json::Value = serde_json::from_str(&stdout)
            .unwrap_or(serde_json::Value::Null);
        let status = v["messages"][0]["status"].as_str().unwrap_or("");
        if status == "0" {
            String::new()
        } else {
            format!("vonage error: {}",
                    v["messages"][0]["error-text"].as_str().unwrap_or("unknown"))
        }
    }
}
