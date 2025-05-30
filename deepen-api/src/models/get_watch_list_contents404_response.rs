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
pub struct GetWatchListContents404Response {
    /// - [CONTENT_IMAGE_NOT_FOUND] コンテンツに紐づく画像が存在しない. 基本起こり得ない. マスターデータが足りない
    #[serde(rename = "error_code", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// - RECOVERABLE   - ユーザが操作しなおす、最新のデータを取得しなおすことで直る可能性のあるエラー     - 入力された値が不正       - 必須項目が未入力       - 最大文字数を超えている       - など     - コンフリクト       - すでに存在するデータを新規に作成しようとしている       - バージョンの古いデータを更新しようとしている       - など - FATAL   - コードの修正が必要かもしれないエラー - TEMPORARY   - 一時的なエラー - UNAUTHORIZED   - リクエストは正常に受け付けたが、認証できない場合のエラー   - クライアントがこのエラーを受け取った場合、直ちに現在のセッションを破棄しなければならない
    #[serde(rename = "error_type", skip_serializing_if = "Option::is_none")]
    pub error_type: Option<String>,
}

impl GetWatchListContents404Response {
    pub fn new() -> GetWatchListContents404Response {
        GetWatchListContents404Response {
            error_code: None,
            error_type: None,
        }
    }
}
