use poise::serenity_prelude::Http;
use linemux::MuxedLines;
use regex::Regex;
use std::sync::Arc;
use std::collections::HashMap;

pub async fn start_log_monitor(http: Arc<Http>, channel_id: u64, log_path: String) {
    // Define some common threat indicators to watch for in the logs
    let threats = vec!["SQL Injection", "Failed password root", "Brute Force", "Malware"];

    let mut log_counter = HashMap::new();

    println!("🛡️ Mini SOC started monitoring: {}", log_path);

    let failed_login_re = Regex::new(
        r"Failed password for (?:invalid user )?(.*?) from (.*?) port"
    ).unwrap();
    let sqli_re = Regex::new(r"(?i)(UNION SELECT|DROP TABLE|--\s|'\s*OR\s*'1'\='1)").unwrap();

    let mut lines = MuxedLines::new().expect("Failed to create log muxer");
    lines.add_file(&log_path).await.expect("Failed to open log file");

    while let Ok(Some(line)) = lines.next_line().await {
        let text = line.line();
        let part: Vec<&str> = text.splitn(2, ' ').collect();

        if part.len() >= 2 {
            let level = part[0];
            let message = part[1];

            *log_counter.entry(level.to_string()).or_insert(0) += 1;

            for threat in &threats {
                if text.to_lowercase().contains(&threat.to_lowercase()) {
                    let alert = format!(
                        "🚨 **[Alert !] Threat Detected :** {} \n**Log:** `{}`",
                        threat,
                        text
                    );
                    send_alert(&http, channel_id, &alert).await;
                }
            }
        }
    }
}

async fn send_alert(http: &Arc<Http>, channel_id: u64, message: &str) {
    let channel = poise::serenity_prelude::ChannelId::new(channel_id);
    let _ = channel.say(http, message).await;
}
