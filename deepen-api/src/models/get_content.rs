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
pub struct GetContent {
    #[serde(rename = "user_basic_info")]
    pub user_basic_info: Box<models::UserBasicInfo>,
    #[serde(rename = "user_content")]
    pub user_content: Box<models::UserContent>,
    #[serde(
        rename = "series_info",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub series_info: Option<Option<Box<models::GetContentSeriesInfo>>>,
    #[serde(rename = "evaluations", skip_serializing_if = "Option::is_none")]
    pub evaluations: Option<Vec<models::GetContentEvaluationsInner>>,
}

impl GetContent {
    pub fn new(
        user_basic_info: models::UserBasicInfo,
        user_content: models::UserContent,
    ) -> GetContent {
        GetContent {
            user_basic_info: Box::new(user_basic_info),
            user_content: Box::new(user_content),
            series_info: None,
            evaluations: None,
        }
    }
}
