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
pub struct PutSettingNotificationRequestNotificationSetting {
    /// 通知が有効かどうか
    #[serde(
        rename = "enabled",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub enabled: Option<Option<bool>>,
    /// ピックアップニュースを受信するかどうか
    #[serde(
        rename = "pickup_notification_enabled",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pickup_notification_enabled: Option<Option<bool>>,
    /// 毎日のお知らせを受信するかどうか
    #[serde(
        rename = "daily_notification_enabled",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub daily_notification_enabled: Option<Option<bool>>,
    /// 毎日のお知らせを受信する時刻(0 ~ 23)
    #[serde(
        rename = "daily_notification_time",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub daily_notification_time: Option<Option<i64>>,
    /// カレンダー通知を受信するかどうか
    #[serde(
        rename = "calendar_notification_enabled",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub calendar_notification_enabled: Option<Option<bool>>,
    /// あなたへのお知らせを受信するかどうか
    #[serde(
        rename = "personalized_notification_enabled",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub personalized_notification_enabled: Option<Option<bool>>,
}

impl PutSettingNotificationRequestNotificationSetting {
    pub fn new() -> PutSettingNotificationRequestNotificationSetting {
        PutSettingNotificationRequestNotificationSetting {
            enabled: None,
            pickup_notification_enabled: None,
            daily_notification_enabled: None,
            daily_notification_time: None,
            calendar_notification_enabled: None,
            personalized_notification_enabled: None,
        }
    }
}
