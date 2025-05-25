use crate::entities::sea_orm_active_enums::*;

impl TryFrom<i64> for AnimationType {
    type Error = ();
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        // アニメーションのタイプ 1: 日替わりアニメーション 2: 誕生日アニメーション 3: 特別アニメーション 4: イベントアニメーション
        match value {
            1 => Ok(AnimationType::Daily),
            2 => Ok(AnimationType::Birthday),
            3 => Ok(AnimationType::Special),
            4 => Ok(AnimationType::Event),
            _ => Err(()),
        }
    }
}
impl TryFrom<i64> for RepeatType {
    type Error = ();
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        // 繰り返し設定 0: 繰り返しなし 1: 毎年
        match value {
            0 => Ok(RepeatType::None),
            1 => Ok(RepeatType::Yearly),
            _ => Err(()),
        }
    }
}

impl TryFrom<i64> for SkinIp {
    type Error = ();
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        // 1: マリオ 2: ゼルダ 3: どうぶつの森 4: ピクミン 5: スプラトゥーン
        match value {
            1 => Ok(SkinIp::Mario),
            2 => Ok(SkinIp::Zelda),
            3 => Ok(SkinIp::AnimalCrossing),
            4 => Ok(SkinIp::Pikmin),
            5 => Ok(SkinIp::Splatoon),
            _ => Err(()),
        }
    }
}

impl TryFrom<i64> for TitleColor {
    type Error = ();
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        // タイトルの色 0: 黒 1: 白
        match value {
            0 => Ok(TitleColor::Black),
            1 => Ok(TitleColor::White),
            _ => Err(()),
        }
    }
}
