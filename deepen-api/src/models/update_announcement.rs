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
pub struct UpdateAnnouncement {
    /// おしらせを読んだかどうか
    #[serde(rename = "is_read")]
    pub is_read: bool,
}

impl UpdateAnnouncement {
    pub fn new(is_read: bool) -> UpdateAnnouncement {
        UpdateAnnouncement { is_read }
    }
}
