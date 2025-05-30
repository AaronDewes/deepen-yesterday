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
pub struct FastlyTokenResponse {
    #[serde(rename = "frequency_question")]
    pub frequency_question: Box<models::FastlyToken>,
    #[serde(rename = "inquiry")]
    pub inquiry: Box<models::FastlyToken>,
    #[serde(rename = "feedback")]
    pub feedback: Box<models::FastlyToken>,
    /// Fastly 認証用に発行したトークン。[expiration]_[signature] の形式となっている。 トークンはヘッダーにつけてください。 ヘッダー名は X-Nintendo-Auth-Token です。
    #[serde(rename = "token")]
    pub token: String,
}

impl FastlyTokenResponse {
    pub fn new(
        frequency_question: models::FastlyToken,
        inquiry: models::FastlyToken,
        feedback: models::FastlyToken,
        token: String,
    ) -> FastlyTokenResponse {
        FastlyTokenResponse {
            frequency_question: Box::new(frequency_question),
            inquiry: Box::new(inquiry),
            feedback: Box::new(feedback),
            token,
        }
    }
}
