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
pub struct Calendar {
    /// カレンダーID
    #[serde(rename = "id")]
    pub id: String,
    /// 1: マリオ 2: ゼルダ 3: どうぶつの森 4: ピクミン 5: スプラトゥーン
    #[serde(rename = "skin_ip")]
    pub skin_ip: i64,
    /// アニメーションのタイプ 1: 日替わりアニメーション 2: 誕生日アニメーション 3: 特別アニメーション 4: イベントアニメーション
    #[serde(rename = "animation_type")]
    pub animation_type: i64,
    #[serde(rename = "date")]
    pub date: Box<models::CalendarDate>,
    /// 繰り返し設定 0: 繰り返しなし 1: 毎年
    #[serde(rename = "repeat_type")]
    pub repeat_type: i64,
    /// 年月の表示位置 1: 左上 2: 右上 3: 左下 4: 右下
    #[serde(rename = "date_position")]
    pub date_position: i64,
    /// 年月の向き 1: 横 2: 縦
    #[serde(rename = "date_direction")]
    pub date_direction: i64,
    /// 年月のカラーコード
    #[serde(rename = "date_color")]
    pub date_color: String,
    /// アニメーションのURL. アニメーションがない場合は空文字になる
    #[serde(rename = "animation_url")]
    pub animation_url: String,
    /// アニメーションのアセットID
    #[serde(rename = "animation_asset_id")]
    pub animation_asset_id: String,
    /// サムネイルのURL
    #[serde(rename = "thumbnail_url")]
    pub thumbnail_url: String,
    /// サムネイルのアセットID
    #[serde(rename = "thumbnail_asset_id")]
    pub thumbnail_asset_id: String,
}

impl Calendar {
    pub fn new(
        id: String,
        skin_ip: i64,
        animation_type: i64,
        date: models::CalendarDate,
        repeat_type: i64,
        date_position: i64,
        date_direction: i64,
        date_color: String,
        animation_url: String,
        animation_asset_id: String,
        thumbnail_url: String,
        thumbnail_asset_id: String,
    ) -> Calendar {
        Calendar {
            id,
            skin_ip,
            animation_type,
            date: Box::new(date),
            repeat_type,
            date_position,
            date_direction,
            date_color,
            animation_url,
            animation_asset_id,
            thumbnail_url,
            thumbnail_asset_id,
        }
    }
}
