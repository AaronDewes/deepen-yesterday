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
pub struct PostAuthLoginResponse {
    /// Deepen UserのID
    #[serde(rename = "user_id")]
    pub user_id: String,
    /// Deepen ServerのAPI呼び出しに利用するJWT
    #[serde(rename = "access_token")]
    pub access_token: String,
    /// リフレッシュトークン
    #[serde(rename = "refresh_token")]
    pub refresh_token: String,
    /// ユーザーを新規登録したかどうか
    #[serde(rename = "is_new_user")]
    pub is_new_user: bool,
}

impl PostAuthLoginResponse {
    pub fn new(
        user_id: String,
        access_token: String,
        refresh_token: String,
        is_new_user: bool,
    ) -> PostAuthLoginResponse {
        PostAuthLoginResponse {
            user_id,
            access_token,
            refresh_token,
            is_new_user,
        }
    }
}
