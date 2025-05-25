use sea_orm::ActiveValue::Set;

use crate::auth::Client;
use crate::storage::download_file_to_minio;
use nintendo_yesterday_db::entities::prelude::*;
use nintendo_yesterday_db::entities::*;
use sea_orm::prelude::*;

pub async fn import_calendars(
    client: &mut Client,
    locale: &str,
    storage: &minio::s3::client::Client,
    db: &sea_orm::DatabaseConnection,
) {
    let calendars = deepen_api::apis::calendars_api::get_all_calendars(
        client.get_config(locale).await,
        locale,
        // TODO: Can we use time zone stuff to get info early?
        "UTC",
        Some("android"),
        Some("1.0.4"),
        None,
        None,
    )
    .await
    .expect("Failed to get calendars");
    for calendar in calendars.calendars {
        let exists = Calendar::find_by_id(calendar.id.clone())
            .count(db)
            .await
            .expect("Failed to check if calendar exists");
        if exists > 0 {
            tracing::debug!("Calendar {} already exists, skipping", calendar.id);
            continue;
        }
        tracing::debug!("Inserting calendar {}", calendar.id);
        download_file_to_minio(
            &calendar.thumbnail_url,
            "assets",
            &calendar.thumbnail_asset_id,
            storage,
            Default::default(),
        )
        .await;
        download_file_to_minio(
            &calendar.animation_url,
            "assets",
            &calendar.animation_asset_id,
            storage,
            Default::default(),
        )
        .await;
        let raw_data = serde_json::to_value(&calendar).unwrap();
        let calendar_model = calendar::ActiveModel {
            id: Set(calendar.id),
            skin_ip: Set(Some(calendar.skin_ip.try_into().unwrap())),
            animation_type: Set(Some(calendar.animation_type.try_into().unwrap())),
            day: Set(calendar.date.day as i32),
            month: Set(calendar.date.month as i32),
            year: Set(calendar.date.year as i32),
            repeat_type: Set(Some(calendar.repeat_type.try_into().unwrap())),
            date_color: Set(Some(calendar.date_color)),
            animation: Set(Some(calendar.animation_asset_id)),
            thumbnail: Set(Some(calendar.thumbnail_asset_id)),
            imported_at: Set(chrono::Utc::now().naive_utc()),
            raw_data: Set(raw_data),
        };
        Calendar::insert(calendar_model)
            .exec(db)
            .await
            .expect("Failed to insert calendar");
    }
}
