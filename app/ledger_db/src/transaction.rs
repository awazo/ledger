mod transaction_type;
mod insert;
mod select;

use chrono::NaiveDate;

use crate::account::AccountType;

pub use transaction_type::*;

#[derive(Debug)]
pub struct TransactionDetail {
    pub account_name: String,
    pub account_type: AccountType,
    pub debit_amount: f32,
    pub credit_amount: f32,
}

#[derive(Debug)]
pub struct Transaction {
    pub transaction_id: i32,
    pub transaction_date: NaiveDate,
    pub transaction_type: TransactionType,
    pub description: String,
    pub details: Vec<TransactionDetail>,
}

