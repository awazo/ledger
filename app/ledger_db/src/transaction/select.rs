use std::collections::HashMap;
use std::convert::From;

use chrono::{
    Months,
    NaiveDate,
};
use rust_decimal::{
    prelude::ToPrimitive,
    Decimal,
};

use crate::{
    Db,
    Error,
};

use super::{
    Transaction,
    TransactionDetail,
};

#[derive(Debug, sqlx::FromRow)]
struct TransactionSelectResult {
    transaction_id: i32,
    transaction_date: NaiveDate,
    transaction_type: String,
    description: String,
    account_name: String,
    account_type: String,
    debit_amount: Decimal,
    credit_amount: Decimal,
}

impl From<&TransactionSelectResult> for TransactionDetail {

    fn from(
        value: &TransactionSelectResult,
    ) -> Self {
        TransactionDetail {
            account_name: value.account_name.clone(),
            account_type: (&value.account_type).into(),
            debit_amount: value.debit_amount.to_f32().unwrap_or(0_f32),
            credit_amount: value.credit_amount.to_f32().unwrap_or(0_f32),
        }
    }

}

impl Transaction {

    fn init_from(
        tsr: &TransactionSelectResult,
    ) -> Self {
        Transaction {
            transaction_id: tsr.transaction_id,
            transaction_date: tsr.transaction_date,
            transaction_type: (&tsr.transaction_type).into(),
            description: tsr.description.clone(),
            details: Vec::new(),
        }
    }

    fn from(
        value: Vec<TransactionSelectResult>,
    ) -> Vec<Transaction> {
        if value.is_empty() { return Vec::new(); }

        let mut id_map = HashMap::new();
        for tsr in &value {
            if !id_map.contains_key(&tsr.transaction_id) {
                id_map.insert(
                    tsr.transaction_id,
                    Transaction::init_from(tsr),
                );
            }
            id_map.get_mut(&tsr.transaction_id).unwrap()
            .details.push(tsr.into());
        }

        let mut trans: Vec<Transaction> = id_map.into_values().collect();
        trans.sort_by(|t1, t2| {
            t1.transaction_date.cmp(&t2.transaction_date)
            .then(
                t1.transaction_type.cmp(&t2.transaction_type)
                .then(t1.transaction_id.cmp(&t2.transaction_id))
            )
        });
        trans
    }

    pub async fn by_month(
        db: &Db,
        year: i32,
        month: u32,
    ) -> Result<Vec<Transaction>, Error> {
        let start_date
            = NaiveDate::from_ymd_opt(year, month, 1)
                .ok_or(Error::DateTimeError)?;
        let end_date = start_date + Months::new(1_u32);
        let query = sqlx::query_as::<_, TransactionSelectResult>(
            r#"
            SELECT
                t.transaction_id,
                t.transaction_date,
                t.transaction_type,
                t.description,
                a.account_name,
                a.account_type,
                td.debit_amount,
                td.credit_amount
            FROM transactions t
                LEFT OUTER JOIN transaction_details td
                ON t.transaction_id = td.transaction_id
                LEFT OUTER JOIN accounts a
                ON td.account_id = a.account_id
            WHERE
                t.transaction_date >= $1
                AND t.transaction_date < $2
            ORDER BY
                t.transaction_date ASC,
                t.transaction_type ASC,
                t.transaction_id ASC,
                td.debit_amount DESC,
                td.credit_amount DESC,
                td.transaction_detail_id ASC
            "#
        )
        .bind(start_date)
        .bind(end_date);

        Ok(Transaction::from(query.fetch_all(&db.conn).await?))
    }

}

