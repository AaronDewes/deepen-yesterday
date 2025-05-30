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
pub struct GetSettingCalendarResponse {
    #[serde(rename = "official_event_setting")]
    pub official_event_setting: Box<models::GetSettingCalendarResponseOfficialEventSetting>,
}

impl GetSettingCalendarResponse {
    pub fn new(
        official_event_setting: models::GetSettingCalendarResponseOfficialEventSetting,
    ) -> GetSettingCalendarResponse {
        GetSettingCalendarResponse {
            official_event_setting: Box::new(official_event_setting),
        }
    }
}
