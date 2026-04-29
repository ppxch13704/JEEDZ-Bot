use poise::serenity_prelude as serenity;
use std::env;
use std::sync::Arc;

mod commands;
mod events;
mod google;
mod tasks;

pub struct Data {}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data
) -> Result<(), Error> {
    events::greetings::handle(ctx, event).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    // Initialize the Rustls crypto provider to prevent the panic!
    let _ = rustls::crypto::ring::default_provider().install_default();

    // Load environment variables
    dotenvy::dotenv().expect("Failed to load .env file");
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Configure gateway intents
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::GUILD_MEMBERS;

    let framework = poise::Framework
        ::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::schedule::meeting()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                // Start background tasks
                let http_client = Arc::clone(&ctx.http);
                let alert_channel_id: u64 = env::var("ALERTS_CHANNEL_ID").unwrap().parse().unwrap();

                // Mini SOC
                let soc_http = Arc::clone(&http_client);
                tokio::spawn(async move {
                    let log_file = "src/log.txt".to_string();
                    tasks::soc::start_log_monitor(soc_http, alert_channel_id, log_file).await;
                });

                // Network Monitor (Currently pointing to a failure test)
                let net_http = Arc::clone(&http_client);
                tokio::spawn(async move {
                    let target_url = "https://httpbin.org/status/500".to_string();
                    tasks::network::start_uptime_checker(
                        net_http,
                        alert_channel_id,
                        target_url
                    ).await;
                });

                Ok(Data {})
            })
        })
        .build();

    // Create the client and start the bot
    let mut client = serenity::ClientBuilder
        ::new(token, intents)
        .framework(framework).await
        .expect("Error creating client");

    println!("Jeedz-bot is starting...");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
