//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "event_schedule")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub start_time: DateTime,
    pub end_time: DateTime,
    pub all_day: bool,
    pub name: String,
    pub icon: Option<String>,
    pub category_name: Option<String>,
    pub memo: Option<String>,
    pub is_display_home_view: bool,
    pub large_category: Option<String>,
    pub is_undefined_ended_at: bool,
    pub is_middle_day: bool,
    pub imported_at: DateTime,
    #[sea_orm(column_type = "JsonBinary")]
    pub raw_data: Json,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::content_item::Entity")]
    ContentItem,
}

impl Related<super::content_item::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ContentItem.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
