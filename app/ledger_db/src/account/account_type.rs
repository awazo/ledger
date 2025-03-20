use std::convert::From;
use std::str::FromStr;

#[derive(Debug)]
pub enum AmountSide {
    Debit,  // 借方
    Credit,  // 貸方
}

impl AmountSide {

    pub fn into_japanese(&self) -> String {
        match self {
            AmountSide::Debit => "借方".to_string(),
            AmountSide::Credit => "貸方".to_string(),
        }
    }

    pub fn from_japanese(ja: impl Into<String>) -> Option<Self> {
        match &ja.into() as &str {
            "借方" | "左" | "左側" | "<-" => Some(AmountSide::Debit),
            "貸方" | "右" | "右側" | "->" => Some(AmountSide::Credit),
            _ => None,
        }
    }

}

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
pub enum AccountType {
    Asset,  // 資産 借方 (<-)
    Liability,  // 負債 貸方 (->)
    Equity,  // 資本 貸方 (->)
    Income,  // 収益 貸方 (->)
    Expense,  // 費用 借方 (<-)
    UtilDebit,  // 作業用借方 借方 (<-)
    UtilCredit,  // 作業用貸方 貸方 (->)
}

impl AccountType {

    pub fn into_japanese(&self) -> String {
        match self {
            AccountType::Asset => "資産".to_string(),
            AccountType::Liability => "負債".to_string(),
            AccountType::Equity => "資本".to_string(),
            AccountType::Income => "収益".to_string(),
            AccountType::Expense => "費用".to_string(),
            AccountType::UtilDebit => "作業用借方".to_string(),
            AccountType::UtilCredit => "作業用貸方".to_string(),
        }
    }

    pub fn from_japanese(ja: impl Into<String>) -> Option<Self> {
        match &ja.into() as &str {
            "資産" => Some(AccountType::Asset),
            "負債" => Some(AccountType::Liability),
            "資本" => Some(AccountType::Equity),
            "収益" => Some(AccountType::Income),
            "費用" => Some(AccountType::Expense),
            "作業用借方" | "借方" => Some(AccountType::UtilDebit),
            "作業用貸方" | "貸方" => Some(AccountType::UtilCredit),
            _ => None,
        }
    }

    pub fn amount_side(&self) -> AmountSide {
        match self {
            AccountType::Asset => AmountSide::Debit,
            AccountType::Liability => AmountSide::Credit,
            AccountType::Equity => AmountSide::Credit,
            AccountType::Income => AmountSide::Credit,
            AccountType::Expense => AmountSide::Debit,
            AccountType::UtilDebit => AmountSide::Debit,
            AccountType::UtilCredit => AmountSide::Credit,
        }
    }

}

impl From<&String> for AccountType {

    fn from(
        value: &String,
    ) -> Self {
        AccountType::from_str(value)
        .unwrap_or_else(|_| {
            AccountType::from_japanese(value)
            .unwrap_or(AccountType::UtilDebit)
        })
    }

}

