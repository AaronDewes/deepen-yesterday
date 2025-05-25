use nintendo_yesterday_db::entities::prelude::*;
use nintendo_yesterday_db::entities::*;
use sea_orm::ActiveValue::Set;
use sea_orm::{SelectColumns, prelude::*};

use crate::auth::Client;
use crate::storage::download_file_to_minio;

pub async fn import_tag(
    tag: deepen_api::models::Tag,
    storage: &minio::s3::client::Client,
    db: &sea_orm::DatabaseConnection,
) {
    let exists = Tag::find_by_id(&tag.id)
        .count(db)
        .await
        .expect("Failed to check if tag exists");
    if exists > 0 {
        tracing::debug!("Tag {} already exists, skipping", tag.id);
        return;
    }
    tracing::info!("Inserting tag {}", tag.id);
    let search_header_asset_object;
    if tag.search_header_image_url.is_empty() {
        search_header_asset_object = Set(None);
    } else {
        let search_header_asset_object_inner = format!("tags/{}/search_header.webp", tag.id);
        download_file_to_minio(
            &tag.search_header_image_url,
            "assets",
            &search_header_asset_object_inner,
            storage,
            Default::default(),
        )
        .await;
        search_header_asset_object = Set(Some(search_header_asset_object_inner));
    }
    let icon_asset_object;
    if tag.icon_url.is_empty() {
        icon_asset_object = Set(None);
    } else {
        let icon_asset_object_inner = format!("tags/{}/icon.webp", tag.id);
        download_file_to_minio(
            &tag.icon_url,
            "assets",
            &icon_asset_object_inner,
            storage,
            Default::default(),
        )
        .await;
        icon_asset_object = Set(Some(icon_asset_object_inner));
    }
    let raw_data = serde_json::to_value(&tag).unwrap();
    let tag_model = tag::ActiveModel {
        id: Set(tag.id),
        name: Set(tag.name),
        icon: icon_asset_object,
        platform: Set(Some(tag.platform)),
        search_header_image: search_header_asset_object,
        search_view_tag_color: Set(Some(tag.search_view_tag_color)),
        platform_text: Set(Some(tag.platform_text)),
        imported_at: Set(chrono::Utc::now().naive_utc()),
        raw_data: Set(raw_data),
    };
    Tag::insert(tag_model)
        .exec(db)
        .await
        .expect("Failed to insert tag");
}
pub async fn get_all_tags(db: &sea_orm::DatabaseConnection) -> Vec<String> {
    Tag::find()
        .select_column(nintendo_yesterday_db::entities::tag::Column::Id)
        .all(db)
        .await
        .expect("Failed to get tags")
        .into_iter()
        .map(|tag| tag.id)
        .collect()
}
