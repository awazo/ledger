use std::convert::From;

use chrono::NaiveDate;
use rust_decimal::{
    prelude::ToPrimitive,
    Decimal,
};

use crate::{
    Db,
    Error,
    account::AccountType,
    transaction::TransactionType,
};

#[derive(Debug, sqlx::FromRow)]
struct SummaryQueryResult {
    account_id: i32,
    account_name: String,
    account_type: String,
    debit: Decimal,
    credit: Decimal,
}

#[derive(Debug)]
pub struct Summary {
    pub account_id: i32,
    pub account_name: String,
    pub account_type: AccountType,
    pub debit: f32,
    pub credit: f32,
}

impl Summary {

    pub async fn upto_from_prev(
        db: &Db,
        start_date: NaiveDate,
    ) -> Result<Vec<Summary>, Error> {
        let query = sqlx::query_as::<_, SummaryQueryResult>(
            r#"
            SELECT
                a.account_id,
                a.account_name,
                a.account_type,
                SUM(td.debit_amount) AS debit,
                SUM(td.credit_amount) AS credit
            FROM transactions t
                LEFT OUTER JOIN transaction_details td
                ON t.transaction_id = td.transaction_id
                LEFT OUTER JOIN accounts a
                ON td.account_id = a.account_id
            WHERE
                (t.transaction_date = $1
                AND t.transaction_type = $2)
            GROUP BY
                a.account_id, a.account_name, a.account_type
            ORDER BY
                a.account_type ASC, a.account_id ASC
            "#
        )
        .bind(start_date)
        .bind(TransactionType::FromPrev.to_string());

        let mut summary = query.fetch_all(&db.conn).await?
            .iter().map(|lqr| Summary::from(lqr))
            .collect::<Vec<Summary>>();
        summary.sort_by(|s1, s2| {
            s1.account_type.cmp(&s2.account_type)
            .then(s1.account_id.cmp(&s2.account_id))
        });
        Ok(summary)
    }

    pub async fn upto_in_term(
        db: &Db,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<Summary>, Error> {
        let query = sqlx::query_as::<_, SummaryQueryResult>(
            r#"
            SELECT
                a.account_id,
                a.account_name,
                a.account_type,
                SUM(td.debit_amount) AS debit,
                SUM(td.credit_amount) AS credit
            FROM transactions t
                LEFT OUTER JOIN transaction_details td
                ON t.transaction_id = td.transaction_id
                LEFT OUTER JOIN accounts a
                ON td.account_id = a.account_id
            WHERE
                (t.transaction_date = $1
                AND t.transaction_type = $3)
                OR
                (t.transaction_date >= $1
                AND t.transaction_date <= $2
                AND t.transaction_type = $4)
            GROUP BY
                a.account_id, a.account_name, a.account_type
            ORDER BY
                a.account_type ASC, a.account_id ASC
            "#
        )
        .bind(start_date)
        .bind(end_date)
        .bind(TransactionType::FromPrev.to_string())
        .bind(TransactionType::InTerm.to_string());

        let mut summary = query.fetch_all(&db.conn).await?
            .iter().map(|lqr| Summary::from(lqr))
            .collect::<Vec<Summary>>();
        summary.sort_by(|s1, s2| {
            s1.account_type.cmp(&s2.account_type)
            .then(s1.account_id.cmp(&s2.account_id))
        });
        Ok(summary)
    }

    pub async fn upto_kessan(
        db: &Db,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<Summary>, Error> {
        let query = sqlx::query_as::<_, SummaryQueryResult>(
            r#"
            SELECT
                a.account_id,
                a.account_name,
                a.account_type,
                SUM(td.debit_amount) AS debit,
                SUM(td.credit_amount) AS credit
            FROM transactions t
                LEFT OUTER JOIN transaction_details td
                ON t.transaction_id = td.transaction_id
                LEFT OUTER JOIN accounts a
                ON td.account_id = a.account_id
            WHERE
                (t.transaction_date = $1
                AND t.transaction_type = $3)
                OR
                (t.transaction_date >= $1
                AND t.transaction_date <= $2
                AND t.transaction_type = $4)
                OR
                (t.transaction_date = $2
                AND t.transaction_type IN ($5))
            GROUP BY
                a.account_id, a.account_name, a.account_type
            ORDER BY
                a.account_type ASC, a.account_id ASC
            "#
        )
        .bind(start_date)
        .bind(end_date)
        .bind(TransactionType::FromPrev.to_string())
        .bind(TransactionType::InTerm.to_string())
        .bind(TransactionType::Kessan.to_string());

        let mut summary = query.fetch_all(&db.conn).await?
            .iter().map(|lqr| Summary::from(lqr))
            .collect::<Vec<Summary>>();
        summary.sort_by(|s1, s2| {
            s1.account_type.cmp(&s2.account_type)
            .then(s1.account_id.cmp(&s2.account_id))
        });
        Ok(summary)
    }

    pub async fn upto_soneki(
        db: &Db,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<Summary>, Error> {
        let query = sqlx::query_as::<_, SummaryQueryResult>(
            r#"
            SELECT
                a.account_id,
                a.account_name,
                a.account_type,
                SUM(td.debit_amount) AS debit,
                SUM(td.credit_amount) AS credit
            FROM transactions t
                LEFT OUTER JOIN transaction_details td
                ON t.transaction_id = td.transaction_id
                LEFT OUTER JOIN accounts a
                ON td.account_id = a.account_id
            WHERE
                (t.transaction_date = $1
                AND t.transaction_type = $3)
                OR
                (t.transaction_date >= $1
                AND t.transaction_date <= $2
                AND t.transaction_type = $4)
                OR
                (t.transaction_date = $2
                AND t.transaction_type IN ($5, $6))
            GROUP BY
                a.account_id, a.account_name, a.account_type
            ORDER BY
                a.account_type ASC, a.account_id ASC
            "#
        )
        .bind(start_date)
        .bind(end_date)
        .bind(TransactionType::FromPrev.to_string())
        .bind(TransactionType::InTerm.to_string())
        .bind(TransactionType::Kessan.to_string())
        .bind(TransactionType::Soneki.to_string());

        let mut summary = query.fetch_all(&db.conn).await?
            .iter().map(|lqr| Summary::from(lqr))
            .collect::<Vec<Summary>>();
        summary.sort_by(|s1, s2| {
            s1.account_type.cmp(&s2.account_type)
            .then(s1.account_id.cmp(&s2.account_id))
        });
        Ok(summary)
    }

    pub async fn upto_to_next(
        db: &Db,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<Summary>, Error> {
        let query = sqlx::query_as::<_, SummaryQueryResult>(
            r#"
            SELECT
                a.account_id,
                a.account_name,
                a.account_type,
                SUM(td.debit_amount) AS debit,
                SUM(td.credit_amount) AS credit
            FROM transactions t
                LEFT OUTER JOIN transaction_details td
                ON t.transaction_id = td.transaction_id
                LEFT OUTER JOIN accounts a
                ON td.account_id = a.account_id
            WHERE
                (t.transaction_date = $1
                AND t.transaction_type = $3)
                OR
                (t.transaction_date >= $1
                AND t.transaction_date <= $2
                AND t.transaction_type = $4)
                OR
                (t.transaction_date = $2
                AND t.transaction_type IN ($5, $6, $7))
            GROUP BY
                a.account_id, a.account_name, a.account_type
            ORDER BY
                a.account_type ASC, a.account_id ASC
            "#
        )
        .bind(start_date)
        .bind(end_date)
        .bind(TransactionType::FromPrev.to_string())
        .bind(TransactionType::InTerm.to_string())
        .bind(TransactionType::Kessan.to_string())
        .bind(TransactionType::Soneki.to_string())
        .bind(TransactionType::ToNext.to_string());

        let mut summary = query.fetch_all(&db.conn).await?
            .iter().map(|lqr| Summary::from(lqr))
            .collect::<Vec<Summary>>();
        summary.sort_by(|s1, s2| {
            s1.account_type.cmp(&s2.account_type)
            .then(s1.account_id.cmp(&s2.account_id))
        });
        Ok(summary)
    }

}

impl From<&SummaryQueryResult> for Summary {

    fn from(
        value: &SummaryQueryResult,
    ) -> Self {
        let acc_type = (&value.account_type).into();
        Summary {
            account_id: value.account_id,
            account_name: value.account_name.clone(),
            account_type: acc_type,
            debit: value.debit.to_f32().unwrap_or(0_f32),
            credit: value.credit.to_f32().unwrap_or(0_f32),
        }
    }

}

