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
pub struct PostUserContentImpressions {
    /// イベントタイムスタンプ
    #[serde(rename = "event_time_stamp")]
    pub event_time_stamp: i64,
}

impl PostUserContentImpressions {
    pub fn new(event_time_stamp: i64) -> PostUserContentImpressions {
        PostUserContentImpressions { event_time_stamp }
    }
}
