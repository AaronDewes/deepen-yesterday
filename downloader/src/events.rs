use chrono::{TimeZone, Utc};
use deepen_api::models::UserEventSchedule;
use sea_orm::ActiveValue::Set;

use crate::auth::Client;
use crate::storage::download_file_to_minio;
use nintendo_yesterday_db::entities::prelude::*;
use nintendo_yesterday_db::entities::*;
use sea_orm::prelude::*;

pub async fn import_event_schedules(
    events: Vec<UserEventSchedule>,
    _locale: &str,
    storage: &minio::s3::client::Client,
    db: &sea_orm::DatabaseConnection,
) {
    for event in events {
        let event_schedule = event.event_schedule;
        let exists = EventSchedule::find_by_id(Uuid::parse_str(&event_schedule.id).unwrap())
            .count(db)
            .await
            .expect("Failed to check if event schedule exists");
        if exists > 0 {
            tracing::debug!(
                "Event schedule {} already exists, skipping",
                event_schedule.id
            );
            continue;
        }
        tracing::debug!("Inserting event schedule {}", event_schedule.id);
        let icon_asset_object;
        if event_schedule.icon_url.is_empty() {
            icon_asset_object = Set(None);
        } else {
            let icon_asset_object_inner =
                format!("event_schedules/{}/icon.webp", event_schedule.id);
            download_file_to_minio(
                &event_schedule.icon_url,
                "assets",
                &icon_asset_object_inner,
                storage,
                Default::default(),
            )
            .await;
            icon_asset_object = Set(Some(icon_asset_object_inner));
        }
        let raw_data = serde_json::to_value(&event_schedule).unwrap();
        let event_schedule_model = event_schedule::ActiveModel {
            id: Set(Uuid::parse_str(&event_schedule.id).unwrap()),
            start_time: Set(Utc.timestamp(event_schedule.started_at, 0).naive_utc()),
            end_time: Set(Utc.timestamp(event_schedule.ended_at, 0).naive_utc()),
            all_day: Set(event_schedule.all_day),
            name: Set(event_schedule.name),
            icon: icon_asset_object,
            category_name: Set(event_schedule.category_name),
            memo: Set(if event_schedule.memo.is_empty() {
                None
            } else {
                Some(event_schedule.memo)
            }),
            is_display_home_view: Set(event_schedule.is_display_home_view),
            large_category: Set(if event_schedule.large_category.is_empty() {
                None
            } else {
                Some(event_schedule.large_category)
            }),
            is_undefined_ended_at: Set(event_schedule.is_undefined_ended_at),
            is_middle_day: Set(event_schedule.is_middle_day),
            imported_at: Set(chrono::Utc::now().naive_utc()),
            raw_data: Set(raw_data),
        };
        EventSchedule::insert(event_schedule_model)
            .exec(db)
            .await
            .expect("Failed to insert event schedule");
    }
}

pub async fn import_hidden_event_schedules(
    client: &mut Client,
    locale: &str,
    storage: &minio::s3::client::Client,
    db: &sea_orm::DatabaseConnection,
) {
    let event_schedule = deepen_api::apis::event_schedules_api::get_hidden_event_schedules(
        client.get_config(locale).await,
        locale,
        "UTC",
        Some("android"),
        Some("1.0.4"),
        None,
        None,
        None,
        None,
    )
    .await
    .expect("Failed to get event schedules");
    let schedules = event_schedule.event_schedules.unwrap_or_default();
    if event_schedule.exist_next_page.is_some_and(|x| x) {
        todo!();
    }
    import_event_schedules(schedules, locale, storage, db).await;
}

pub async fn import_public_event_schedules(
    client: &mut Client,
    locale: &str,
    start_time: &str,
    end_time: &str,
    storage: &minio::s3::client::Client,
    db: &sea_orm::DatabaseConnection,
) {
    let event_schedule = deepen_api::apis::event_schedules_api::get_event_schedules(
        client.get_config(locale).await,
        locale,
        "UTC",
        start_time.to_string(),
        end_time.to_string(),
        Some("android"),
        Some("1.0.4"),
        None,
        None,
    )
    .await
    .expect(&format!(
        "Failed to get event schedules between {} and {}",
        start_time, end_time
    ));
    println!(
        "Got {} calendars between {} and {}",
        event_schedule.calendars.len(),
        start_time,
        end_time
    );
    let mut all_events = event_schedule
        .calendars
        .iter()
        .flat_map(|item| {
            item.event_schedules.iter().map(|item| {
                (
                    item.event_schedule.id.clone(),
                    item.event_schedule.started_at,
                    item.event_schedule.ended_at,
                )
            })
        })
        .collect::<Vec<_>>();
    all_events.sort_by(|(a_id, _, _), (b_id, _, _)| a_id.cmp(b_id));
    all_events.dedup();
    println!("Found {} events", all_events.len());
    if let Some((_, start_time, end_time)) = all_events.first() {
        let start_time = Utc.timestamp(*start_time, 0);
        let end_time = Utc.timestamp(*end_time, 0);
        println!(
            "First event starts at {} and ends at {}",
            start_time, end_time
        );
    }
    if let Some((_, start_time, end_time)) = all_events.last() {
        let start_time = Utc.timestamp(*start_time, 0);
        let end_time = Utc.timestamp(*end_time, 0);
        println!(
            "Last event starts at {} and ends at {}",
            start_time, end_time
        );
    }
    for calendar in event_schedule.calendars {
        import_event_schedules(calendar.event_schedules, locale, storage, db).await;
    }
}
