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
pub struct CalendarDate {
    /// 日
    #[serde(rename = "day")]
    pub day: i64,
    /// 月 アニメーションの種類が日替わりか誕生日の時は0が入ります
    #[serde(rename = "month")]
    pub month: i64,
    /// 年 アニメーションの種類が特別でrepeat_typeが繰り返しなしの場合以外は0が入ります
    #[serde(rename = "year")]
    pub year: i64,
}

impl CalendarDate {
    pub fn new(day: i64, month: i64, year: i64) -> CalendarDate {
        CalendarDate { day, month, year }
    }
}
