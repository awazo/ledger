use std::convert::From;

use crate::{
    Db,
    Error,
};

use super::Account;

#[derive(Debug, sqlx::FromRow)]
struct AccountSelectResult {
    account_id: i32,
    account_name: String,
    account_type: String,
}

impl Account {

    pub async fn by_name(
        db: &Db,
        account_name: &str,
    ) -> Result<Self, Error> {
        let query = sqlx::query_as::<_, AccountSelectResult>(
            r#"
            SELECT
                account_id, account_name, account_type
            FROM accounts
            WHERE account_name = $1
            "#
        )
        .bind(account_name);

        Ok((&query.fetch_one(&db.conn).await?).into())
    }

    pub async fn all(
        db: &Db,
    ) -> Result<Vec<Self>, Error> {
        let query = sqlx::query_as::<_, AccountSelectResult>(
            r#"
            SELECT
                account_id, account_name, account_type
            FROM accounts
            ORDER BY account_type ASC, account_id ASC
            "#
        );

        let mut acc = query.fetch_all(&db.conn).await?
            .iter().map(|q| q.into()).collect::<Vec<Account>>();
        acc.sort_by(|a1, a2| {
            a1.account_type.cmp(&a2.account_type)
            .then(a1.account_id.cmp(&a2.account_id))
        });
        Ok(acc)
    }

}

impl From<&AccountSelectResult> for Account {

    fn from(
        value: &AccountSelectResult,
    ) -> Self {
        let account_type = (&value.account_type).into();
        Account {
            account_id: value.account_id,
            account_name: value.account_name.clone(),
            account_type,
        }
    }

}

