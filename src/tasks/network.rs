use poise::serenity_prelude::Http;
use reqwest::Client;
use std::sync::Arc;
use tokio::time::{ sleep, Duration };

pub async fn start_uptime_checker(http: Arc<Http>, channel_id: u64, url: String) {
    println!("🌐 Network Monitor started for: {}", url);

    let client = Client::builder()
        .user_agent("JeedzBot/1.0 (NetworkMonitor)")
        .build()
        .expect("Failed to build HTTP client");

    let init_msg = format!("📡 **Network Monitor Online**\nNow watching: `{}`", url);
    send_alert(&http, channel_id, &init_msg).await;

    let mut was_down = false;

    loop {
        println!("🔍 [Network] Checking: {}", url);

        match client.get(&url).send().await {
            Ok(response) => {
                println!("📡 [Network] Received Status: {}", response.status());

                if response.status().is_success() {
                    if was_down {
                        let msg = format!("✅ **Network Monitor**: `{}` is back ONLINE.", url);
                        send_alert(&http, channel_id, &msg).await;
                        was_down = false;
                    }
                } else if !was_down {
                    let msg = format!(
                        "⚠️ **Network Monitor**: `{}` returned status code **{}**!",
                        url,
                        response.status()
                    );
                    send_alert(&http, channel_id, &msg).await;
                    was_down = true;
                }
            }
            Err(e) => {
                println!("🚨 [Network] Request failed: {}", e);
                if !was_down {
                    let msg = format!(
                        "❌ **Network Monitor**: `{}` is completely OFFLINE!\n**Reason:** `{}`",
                        url,
                        e
                    );
                    send_alert(&http, channel_id, &msg).await;
                    was_down = true;
                }
            }
        }
        sleep(Duration::from_secs(10)).await;
    }
}

async fn send_alert(http: &Arc<Http>, channel_id: u64, message: &str) {
    let channel = poise::serenity_prelude::ChannelId::new(channel_id);
    let _ = channel.say(http, message).await;
}
