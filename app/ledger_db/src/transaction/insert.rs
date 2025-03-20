use rust_decimal::Decimal;

use crate::{
    Db,
    Error,
    account::Account,
};

use super::Transaction;

#[derive(Debug, sqlx::FromRow)]
struct TransactionInsertResult {
    transaction_id: i32,
}

impl Transaction {

    pub async fn insert(
        &self,
        db: &Db,
    ) -> Result<i32, Error> {
        let mut tx = db.conn.begin().await?;

        let transaction_id = sqlx::query_as::<_, TransactionInsertResult>(
            r#"
            INSERT INTO transactions
                (transaction_date, transaction_type, description)
            VALUES ($1, $2, $3)
            RETURNING
                transaction_id
            "#
        )
        .bind(self.transaction_date)
        .bind(self.transaction_type.to_string())
        .bind(&self.description)
        .fetch_one(&mut *tx)
        .await?.transaction_id;

        for d in &self.details {
            let acc = match Account::by_name(db, &d.account_name).await {
                Ok(acc) => acc,
                Err(Error::RowNotFound)
                    => return Err(Error::AccountNotFound),
                Err(err) => return Err(err),
            };

            let debit = match Decimal::from_f32_retain(d.debit_amount) {
                Some(a) => a,
                None => return Err(Error::DecimalConvError(d.debit_amount)),
            };
            let credit = match Decimal::from_f32_retain(d.credit_amount) {
                Some(a) => a,
                None => return Err(Error::DecimalConvError(d.credit_amount)),
            };
            sqlx::query(
                r#"
                INSERT INTO transaction_details
                    (transaction_id, account_id, debit_amount, credit_amount)
                VALUES ($1, $2, $3, $4)
                "#
            )
            .bind(transaction_id)
            .bind(acc.account_id)
            .bind(debit)
            .bind(credit)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        Ok(transaction_id)
    }

}

