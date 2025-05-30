/*
 * Deepen API
 *
 * Deepen API
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetSettingCalendarResponseOfficialEventSettingInGameEventSettingRecentPlayGameTitlesInner
{
    #[serde(rename = "game_title")]
    pub game_title: Box<models::GameTitle>,
    /// 表示設定が有効かどうか
    #[serde(rename = "enabled")]
    pub enabled: bool,
}

impl GetSettingCalendarResponseOfficialEventSettingInGameEventSettingRecentPlayGameTitlesInner {
    pub fn new(
        game_title: models::GameTitle,
        enabled: bool,
    ) -> GetSettingCalendarResponseOfficialEventSettingInGameEventSettingRecentPlayGameTitlesInner
    {
        GetSettingCalendarResponseOfficialEventSettingInGameEventSettingRecentPlayGameTitlesInner {
            game_title: Box::new(game_title),
            enabled,
        }
    }
}
