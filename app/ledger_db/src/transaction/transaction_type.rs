use std::convert::From;
use std::str::FromStr;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    strum_macros::EnumString,
    strum_macros::Display,
)]
pub enum TransactionType {
    FromPrev,  // 前期繰越
    InTerm,  // 期中仕訳
    Kessan,  // 決算仕訳
    Soneki,  // 損益計算
    ToNext,  // 次期繰越
}

impl TransactionType {

    pub fn into_japanese(
        &self
    ) -> String {
        match self {
            TransactionType::FromPrev => "前期繰越".to_string(),
            TransactionType::InTerm => "期中仕訳".to_string(),
            TransactionType::Kessan => "決算仕訳".to_string(),
            TransactionType::Soneki => "損益計算".to_string(),
            TransactionType::ToNext => "次期繰越".to_string(),
        }
    }

    pub fn from_japanese(
        ja: impl Into<String>
    ) -> Option<Self> {
        match &ja.into() as &str {
            "前期繰越" | "前期" => Some(TransactionType::FromPrev),
            "期中仕訳" | "期中" | "仕訳" => Some(TransactionType::InTerm),
            "決算仕訳" | "決算" => Some(TransactionType::Kessan),
            "損益計算" | "損益" => Some(TransactionType::Soneki),
            "次期繰越" | "次期" => Some(TransactionType::ToNext),
            _ => None,
        }
    }

}

impl From<&String> for TransactionType {

    fn from(
        value: &String,
    ) -> Self {
        TransactionType::from_str(value)
        .unwrap_or_else(|_| {
            TransactionType::from_japanese(value)
            .unwrap_or(TransactionType::InTerm)
        })
    }

}

