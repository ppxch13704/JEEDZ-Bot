use poise::serenity_prelude as serenity;
use std::env;
use crate::Error;

pub async fn handle(ctx: &serenity::Context, event: &serenity::FullEvent) -> Result<(), Error> {
    match event {
        serenity::FullEvent::GuildMemberAddition { new_member } => {
            let channel_id = get_greeting_channel();
            let welcome_message = format!(
                "Welcome to the server, <@{}>! We're glad to have you here.",
                new_member.user.id
            );
            let _ = channel_id.say(&ctx.http, welcome_message).await;
        }
        serenity::FullEvent::GuildMemberRemoval { user, .. } => {
            let channel_id = get_greeting_channel();
            let goodbye_message = format!("Goodbye, **{}**. We'll miss you!", user.name);
            let _ = channel_id.say(&ctx.http, goodbye_message).await;
        }
        _ => {}
    }
    Ok(())
}

fn get_greeting_channel() -> serenity::ChannelId {
    let id: u64 = env::var("GREETING_CHANNEL_ID").unwrap().parse().unwrap();
    serenity::ChannelId::new(id)
}
