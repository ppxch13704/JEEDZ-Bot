use google_calendar3::api::{ Event, EventDateTime };
use chrono::{ TimeZone, NaiveDateTime, Utc };
use chrono_tz::Tz;
use std::env;
use crate::google::auth::get_hub;
use crate::Error;

pub async fn create_meeting(
    title: &str,
    date: &str,
    time: &str,
    tz_input: &str,
    duration_mins: i64
) -> Result<String, Error> {
    let hub = get_hub().await;
    let calendar_id = env::var("GOOGLE_CALENDAR_ID").expect("Missing GOOGLE_CALENDAR_ID");

    let parsed_tz: Tz = tz_input.parse().unwrap_or(chrono_tz::Asia::Bangkok);

    let datetime_str = format!("{}T{}:00", date, time);
    let naive_dt = NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%dT%H:%M:%S").map_err(
        |_| "Invalid date/time format. Use YYYY-MM-DD and HH:MM"
    )?;

    let start_dt = match parsed_tz.from_local_datetime(&naive_dt) {
        chrono::LocalResult::Single(dt) => dt,
        _ => {
            return Err("Invalid local time".into());
        }
    };

    
    let end_dt = start_dt + chrono::Duration::minutes(duration_mins);

    let req = Event {
        summary: Some(title.to_string()),
        start: Some(EventDateTime {
            date_time: Some(start_dt.with_timezone(&Utc)),
            time_zone: Some(parsed_tz.name().to_string()),
            ..Default::default()
        }),
        end: Some(EventDateTime {
            date_time: Some(end_dt.with_timezone(&Utc)),
            time_zone: Some(parsed_tz.name().to_string()),
            ..Default::default()
        }),
        ..Default::default()
    };

    let result = hub.events().insert(req, &calendar_id).doit().await?;
    let event_link = result.1.html_link.unwrap_or_else(|| "No link available".to_string());

    Ok(event_link)
}
