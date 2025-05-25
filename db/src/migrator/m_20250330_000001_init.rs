use sea_orm::sea_query::extension::postgres::Type;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20250330_000001_init"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the Bakery table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(TagType::Type)
                    .values([
                        TagType::IP,
                        TagType::SoftwareName,
                        TagType::SoftwareSeriesName,
                        TagType::ContentClassification,
                        TagType::Platform,
                        TagType::ContentGroup,
                        TagType::NintendoSwitchOnline,
                    ])
                    .to_owned(),
            )
            .await?;
        manager
            .create_type(
                Type::create()
                    .as_enum(ContentType::Type)
                    .values([ContentType::Webview, ContentType::Video, ContentType::Image])
                    .to_owned(),
            )
            .await?;
        manager
            .create_type(
                Type::create()
                    .as_enum(TitleColor::Type)
                    .values([TitleColor::Black, TitleColor::White])
                    .to_owned(),
            )
            .await?;
        manager
            .create_type(
                Type::create()
                    .as_enum(SkinIP::Type)
                    .values([
                        SkinIP::Mario,
                        SkinIP::Zelda,
                        SkinIP::AnimalCrossing,
                        SkinIP::Pikmin,
                        SkinIP::Splatoon,
                    ])
                    .to_owned(),
            )
            .await?;
        manager
            .create_type(
                Type::create()
                    .as_enum(AnimationType::Type)
                    .values([
                        AnimationType::Daily,
                        AnimationType::Birthday,
                        AnimationType::Special,
                        AnimationType::Event,
                    ])
                    .to_owned(),
            )
            .await?;
        manager
            .create_type(
                Type::create()
                    .as_enum(RepeatType::Type)
                    .values([RepeatType::None, RepeatType::Yearly])
                    .to_owned(),
            )
            .await?;
        manager
            .create_type(
                Type::create()
                    .as_enum(DatePosition::Type)
                    .values([
                        DatePosition::TopLeft,
                        DatePosition::TopRight,
                        DatePosition::BottomLeft,
                        DatePosition::BottomRight,
                    ])
                    .to_owned(),
            )
            .await?;
        manager
            .create_type(
                Type::create()
                    .as_enum(DateDirection::Type)
                    .values([DateDirection::Horizontal, DateDirection::Vertical])
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Tag::Table)
                    .col(ColumnDef::new(Tag::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Tag::Name).string().not_null())
                    .col(ColumnDef::new(Tag::Icon).string())
                    .col(
                        ColumnDef::new(Tag::Platform).array(ColumnType::String(Default::default())),
                    )
                    .col(ColumnDef::new(Tag::SearchHeaderImage).string())
                    .col(ColumnDef::new(Tag::SearchViewTagColor).string())
                    .col(ColumnDef::new(Tag::PlatformText).string())
                    .col(ColumnDef::new(Tag::ImportedAt).timestamp().not_null())
                    .col(ColumnDef::new(Tag::RawData).json_binary().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(EventSchedule::Table)
                    .col(
                        ColumnDef::new(EventSchedule::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(EventSchedule::StartTime)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EventSchedule::EndTime)
                            .timestamp()
                            .not_null(),
                    )
                    .col(ColumnDef::new(EventSchedule::AllDay).boolean().not_null())
                    .col(ColumnDef::new(EventSchedule::Name).string().not_null())
                    .col(ColumnDef::new(EventSchedule::Icon).string())
                    .col(ColumnDef::new(EventSchedule::CategoryName).string())
                    .col(ColumnDef::new(EventSchedule::Memo).string())
                    .col(
                        ColumnDef::new(EventSchedule::IsDisplayHomeView)
                            .boolean()
                            .not_null(),
                    )
                    .col(ColumnDef::new(EventSchedule::LargeCategory).string())
                    .col(
                        ColumnDef::new(EventSchedule::IsUndefinedEndedAt)
                            .boolean()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EventSchedule::IsMiddleDay)
                            .boolean()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EventSchedule::ImportedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EventSchedule::RawData)
                            .json_binary()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(ContentItem::Table)
                    .col(
                        ColumnDef::new(ContentItem::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ContentItem::Title).string().not_null())
                    .col(ColumnDef::new(ContentItem::Category).string().not_null())
                    .col(ColumnDef::new(ContentItem::CategoryColor).string())
                    .col(ColumnDef::new(ContentItem::ContentUrl).string())
                    .col(ColumnDef::new(ContentItem::ContentBodyUrl).string())
                    .col(ColumnDef::new(ContentItem::TitleColor).custom(TitleColor::Type))
                    .col(ColumnDef::new(ContentItem::LinkedEventScheduleId).uuid())
                    .col(ColumnDef::new(ContentItem::IsNews).boolean().not_null())
                    .col(ColumnDef::new(ContentItem::IsPremiere).boolean().not_null())
                    .col(ColumnDef::new(ContentItem::Note).string())
                    .col(ColumnDef::new(ContentItem::PanelWidth).integer().not_null())
                    .col(
                        ColumnDef::new(ContentItem::PanelHeight)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ContentItem::ContentImages)
                            .array(ColumnType::String(Default::default())),
                    )
                    .col(ColumnDef::new(ContentItem::ContentMovie).string())
                    .col(ColumnDef::new(ContentItem::Icon).string())
                    .col(ColumnDef::new(ContentItem::Thumbnail).string())
                    .col(ColumnDef::new(ContentItem::Panel).string())
                    .col(
                        ColumnDef::new(ContentItem::ImportedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ContentItem::RawData)
                            .json_binary()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Calendar::Table)
                    .col(
                        ColumnDef::new(Calendar::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Calendar::SkinIP).custom(SkinIP::Type))
                    .col(ColumnDef::new(Calendar::AnimationType).custom(AnimationType::Type))
                    .col(ColumnDef::new(Calendar::Day).integer().not_null())
                    .col(ColumnDef::new(Calendar::Month).integer().not_null())
                    .col(ColumnDef::new(Calendar::Year).integer().not_null())
                    .col(ColumnDef::new(Calendar::RepeatType).custom(RepeatType::Type))
                    .col(ColumnDef::new(Calendar::DateColor).string())
                    .col(ColumnDef::new(Calendar::Animation).string())
                    .col(ColumnDef::new(Calendar::Thumbnail).string())
                    .col(ColumnDef::new(Calendar::ImportedAt).timestamp().not_null())
                    .col(ColumnDef::new(Calendar::RawData).json_binary().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(ContentItemTag::Table)
                    .col(
                        ColumnDef::new(ContentItemTag::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ContentItemTag::ContentItemId).uuid().not_null())
                    .col(ColumnDef::new(ContentItemTag::TagId).string().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_content_item_event_schedule")
                    .from(ContentItem::Table, ContentItem::LinkedEventScheduleId)
                    .to(EventSchedule::Table, EventSchedule::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_content_item_tag_content_item")
                    .from(ContentItemTag::Table, ContentItemTag::ContentItemId)
                    .to(ContentItem::Table, ContentItem::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_content_item_tag_tag")
                    .from(ContentItemTag::Table, ContentItemTag::TagId)
                    .to(Tag::Table, Tag::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    // Define how to rollback this migration: Drop the Bakery table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_content_item_event_schedule")
                    .table(ContentItem::Table)
                    .to_owned(),
            )
            .await?;
        manager.drop_foreign_key(
            ForeignKey::drop()
                .name("fk_content_item_tag_content_item")
                .table(ContentItemTag::Table)
                .to_owned(),
        )
        .await?;
        manager.drop_foreign_key(
            ForeignKey::drop()
                .name("fk_content_item_tag_tag")
                .table(ContentItemTag::Table)
                .to_owned(),
        )
        .await?;
        manager
            .drop_table(Table::drop().table(Tag::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(EventSchedule::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(ContentItem::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Calendar::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(ContentItemTag::Table).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(TagType::Type).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(ContentType::Type).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(TitleColor::Type).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(SkinIP::Type).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(AnimationType::Type).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(RepeatType::Type).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(DatePosition::Type).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(DateDirection::Type).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
pub enum TagType {
    #[iden = "tag_type"]
    Type,
    IP,
    SoftwareName,
    SoftwareSeriesName,
    ContentClassification,
    Platform,
    ContentGroup,
    NintendoSwitchOnline,
}

#[derive(Iden)]
pub enum Tag {
    #[iden = "tag"]
    Table,
    Id,
    Name,
    Icon,
    Platform,
    SearchHeaderImage,
    SearchViewTagColor,
    PlatformText,
    ImportedAt,
    RawData,
}

#[derive(Iden)]
pub enum EventSchedule {
    #[iden = "event_schedule"]
    Table,
    Id,
    StartTime,
    EndTime,
    AllDay,
    Name,
    Icon,
    CategoryName,
    Memo,
    IsDisplayHomeView,
    LargeCategory,
    IsUndefinedEndedAt,
    IsMiddleDay,
    ImportedAt,
    RawData,
}

#[derive(Iden)]
pub enum ContentType {
    #[iden = "content_type"]
    Type,
    Webview,
    Video,
    Image,
}

#[derive(Iden)]
pub enum TitleColor {
    #[iden = "title_color"]
    Type,
    Black,
    White,
}

#[derive(Iden)]
pub enum SkinIP {
    #[iden = "skin_ip"]
    Type,
    Mario,
    Zelda,
    AnimalCrossing,
    Pikmin,
    Splatoon,
}

#[derive(Iden)]
pub enum AnimationType {
    #[iden = "animation_type"]
    Type,
    Daily,
    Birthday,
    Special,
    Event,
}

#[derive(Iden)]

pub enum ContentItem {
    #[iden = "content_item"]
    Table,
    Id,
    Title,
    Category,
    CategoryColor,
    ContentUrl,
    ContentBodyUrl,
    TitleColor,
    LinkedEventScheduleId,
    IsNews,
    IsPremiere,
    Note,
    PanelWidth,
    PanelHeight,
    // We need to store these ourselves
    ContentImages,
    ContentMovie,
    Icon,
    Thumbnail,
    Panel,
    ImportedAt,
    RawData,
}
#[derive(Iden)]
pub enum RepeatType {
    #[iden = "repeat_type"]
    Type,
    None,
    Yearly,
}
#[derive(Iden)]
pub enum DatePosition {
    #[iden = "date_position"]
    Type,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}
#[derive(Iden)]
pub enum DateDirection {
    #[iden = "date_direction"]
    Type,
    Horizontal,
    Vertical,
}
#[derive(Iden)]

pub enum Calendar {
    #[iden = "calendar"]
    Table,
    Id,
    SkinIP,
    AnimationType,
    Day,
    Month,
    Year,
    RepeatType,
    DateColor,
    Animation,
    Thumbnail,
    ImportedAt,
    RawData,
}


#[derive(Iden)]
pub enum ContentItemTag {
    #[iden = "content_item_tag"]
    Table,
    Id,
    ContentItemId,
    TagId,
}
