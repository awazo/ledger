mod account_type;
mod insert;
mod select;

pub use account_type::*;

#[derive(Debug)]
pub struct Account {
    pub account_id: i32,
    pub account_name: String,
    pub account_type: AccountType,
}

