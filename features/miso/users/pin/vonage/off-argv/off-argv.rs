struct feature_OffArgv;
impl feature_OffArgv {
    // curl config-file quoting for a value: backslash and doublequote only —
    // the fields here (keys, a phone number, "miso login code: NNNNNN") contain
    // nothing else that curl's -K parser treats specially.
    fn curl_escape(s: String) -> String {
        s.replace("\\", "\\\\").replace("\"", "\\\"")
    }

    // the base put the api_key, api_secret AND the login-code text on curl's
    // argv, where any local `ps` reads them. This feeds the whole request to
    // `curl -K -` on stdin instead, so nothing sensitive appears in the process
    // list or on disk. Behaviour (and the no-creds console fallback) is
    // otherwise identical.
    fn send_sms(to: String, text: String) -> String {
        use std::io::Write;
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
        let config = format!("url = \"https://rest.nexmo.com/sms/json\"\ndata-urlencode = \"api_key={}\"\ndata-urlencode = \"api_secret={}\"\ndata-urlencode = \"from={}\"\ndata-urlencode = \"to={}\"\ndata-urlencode = \"text={}\"\n",
            curl_escape(key), curl_escape(sec), curl_escape(from),
            curl_escape(to.replace("+", "")), curl_escape(text));
        let child = std::process::Command::new("curl")
            .arg("-s").arg("-K").arg("-")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn();
        let mut child = match child {
            Ok(c) => c,
            Err(e) => return format!("curl failed: {}", e),
        };
        if let Some(mut sin) = child.stdin.take() {
            let _ = sin.write_all(config.as_bytes());
        }
        let out = match child.wait_with_output() {
            Ok(o) => o,
            Err(e) => return format!("curl failed: {}", e),
        };
        let stdout = String::from_utf8_lossy(&out.stdout).to_string();
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
