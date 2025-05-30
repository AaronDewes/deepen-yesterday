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
pub struct UpdateUser {
    /// 言語
    #[serde(rename = "locale", deserialize_with = "Option::deserialize")]
    pub locale: Option<String>,
}

impl UpdateUser {
    pub fn new(locale: Option<String>) -> UpdateUser {
        UpdateUser { locale }
    }
}
