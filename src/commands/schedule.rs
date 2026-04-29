use crate::{ Context, Error };
use poise::serenity_prelude as serenity;
use poise::CreateReply;
use serenity::all::CreateEmbed;
use crate::google::calendar::create_meeting;

/// Schedule a team meeting
#[poise::command(slash_command)]
pub async fn meeting(
    ctx: Context<'_>,
    #[description = "Title of the meeting"] title: String,
    #[description = "Date (YYYY-MM-DD)"] date: String,
    #[description = "Time (HH:MM in 24h format)"] time: String,
    #[description = "Duration in minutes (Defaults to 60)"] duration: Option<i64>,
    #[description = "Role to notify"] role: Option<serenity::Role>,
    #[description = "Timezone (e.g., Asia/Bangkok)"] timezone: Option<String>
) -> Result<(), Error> {
    ctx.defer().await?;

    let role_mention = match &role {
        Some(r) => format!("<@&{}>", r.id),
        None => "No specific role".to_string(),
    };

    let meeting_duration = duration.unwrap_or(60);
    let target_tz = timezone.unwrap_or_else(|| "Asia/Bangkok".to_string());

    match create_meeting(&title, &date, &time, &target_tz, meeting_duration).await {
        Ok(link) => {
            // Build the professional Embed Card
            let embed = CreateEmbed::new()
                .title("🗓️ Team Meeting Scheduled")
                .url(&link)
                .color(0x4285f4)
                .description(format!("**{}**", title))
                .field("Date & Time", format!("{} at {}", date, time), true)
                .field("Duration", format!("{} mins", meeting_duration), true)
                .field("Timezone", target_tz, true)
                .field("Attendees", &role_mention, false)
                .footer(serenity::all::CreateEmbedFooter::new("Powered by Dlife AI System"));

            // Send the embed along with a normal text ping
            let builder = CreateReply::default()
                .content(format!("New meeting created! {}", role_mention))
                .embed(embed);

            ctx.send(builder).await?;
        }
        Err(e) => {
            ctx.say(format!("❌ Failed to schedule meeting: {}", e)).await?;
        }
    }
    Ok(())
}
