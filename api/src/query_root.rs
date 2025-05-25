use crate::entities::*;
use async_graphql::dynamic::*;
use sea_orm::DatabaseConnection;
use seaography::{async_graphql, lazy_static, Builder, BuilderContext};

lazy_static::lazy_static! { static ref CONTEXT : BuilderContext = BuilderContext :: default () ; }

pub fn schema(
    database: DatabaseConnection,
    depth: Option<usize>,
    complexity: Option<usize>,
) -> Result<Schema, SchemaError> {
    let mut builder = Builder::new(&CONTEXT, database.clone());
    seaography::register_entities!(
        builder,
        [
            calendar,
            content_item,
            content_item_tag,
            event_schedule,
            tag,
        ]
    );
    builder.register_enumeration::<crate::entities::sea_orm_active_enums::AnimationType>();
    builder.register_enumeration::<crate::entities::sea_orm_active_enums::RepeatType>();
    builder.register_enumeration::<crate::entities::sea_orm_active_enums::SkinIp>();
    builder.register_enumeration::<crate::entities::sea_orm_active_enums::TitleColor>();
    builder
        .set_depth_limit(depth)
        .set_complexity_limit(complexity)
        .schema_builder()
        .data(database)
        .finish()
}
