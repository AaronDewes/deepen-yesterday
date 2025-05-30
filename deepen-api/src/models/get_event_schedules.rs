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
pub struct GetEventSchedules {
    #[serde(rename = "user_basic_info")]
    pub user_basic_info: Box<models::UserBasicInfo>,
    #[serde(rename = "calendars")]
    pub calendars: Vec<models::GetEventSchedulesCalendarsInner>,
}

impl GetEventSchedules {
    pub fn new(
        user_basic_info: models::UserBasicInfo,
        calendars: Vec<models::GetEventSchedulesCalendarsInner>,
    ) -> GetEventSchedules {
        GetEventSchedules {
            user_basic_info: Box::new(user_basic_info),
            calendars,
        }
    }
}
