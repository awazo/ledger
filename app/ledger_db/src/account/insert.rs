use crate::{
    Db,
    Error,
};

use super::Account;

#[derive(Debug, sqlx::FromRow)]
struct AccountInsertResult {
    account_id: i32,
}

impl Account {

    pub async fn insert(
        &self,
        db: &Db,
    ) -> Result<i32, Error> {
        let query = sqlx::query_as::<_, AccountInsertResult>(
            r#"
            INSERT INTO accounts
                (account_name, account_type)
            VALUES ($1, $2)
            RETURNING
                account_id
            "#
        )
        .bind(&self.account_name)
        .bind(self.account_type.to_string());

        Ok(query.fetch_one(&db.conn).await?.account_id)
    }

}

