//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use super::sea_orm_active_enums::TitleColor;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "content_item")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub title: String,
    pub category: String,
    pub category_color: Option<String>,
    pub content_url: Option<String>,
    pub content_body_url: Option<String>,
    pub title_color: Option<TitleColor>,
    pub linked_event_schedule_id: Option<Uuid>,
    pub is_news: bool,
    pub is_premiere: bool,
    pub note: Option<String>,
    pub panel_width: i32,
    pub panel_height: i32,
    pub content_images: Option<Vec<String>>,
    pub content_movie: Option<String>,
    pub icon: Option<String>,
    pub thumbnail: Option<String>,
    pub panel: Option<String>,
    pub imported_at: DateTime,
    #[sea_orm(column_type = "JsonBinary")]
    pub raw_data: Json,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::content_item_tag::Entity")]
    ContentItemTag,
    #[sea_orm(
        belongs_to = "super::event_schedule::Entity",
        from = "Column::LinkedEventScheduleId",
        to = "super::event_schedule::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    EventSchedule,
}

impl Related<super::content_item_tag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ContentItemTag.def()
    }
}

impl Related<super::event_schedule::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EventSchedule.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
    #[sea_orm(entity = "super::content_item_tag::Entity")]
    ContentItemTag,
    #[sea_orm(entity = "super::event_schedule::Entity")]
    EventSchedule,
}
