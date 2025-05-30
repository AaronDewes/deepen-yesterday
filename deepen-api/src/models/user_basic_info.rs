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
pub struct UserBasicInfo {
    /// サーバー時刻
    #[serde(rename = "server_time")]
    pub server_time: i64,
    /// お子さまの場合の利用制限に関するステータス 0: 利用可能 1: 制限中
    #[serde(
        rename = "child_account_status",
        skip_serializing_if = "Option::is_none"
    )]
    pub child_account_status: Option<i32>,
    /// APIのバージョン
    #[serde(rename = "api_version")]
    pub api_version: String,
}

impl UserBasicInfo {
    pub fn new(server_time: i64, api_version: String) -> UserBasicInfo {
        UserBasicInfo {
            server_time,
            child_account_status: None,
            api_version,
        }
    }
}
