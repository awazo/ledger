use sqlx::postgres::{
    PgPool,
    PgPoolOptions,
};

use crate::Error;

#[derive(Debug)]
pub struct Db {
    pub conn: PgPool,
}

impl Db {

    pub async fn connect(
        conn_str: &str,
        max_conn: u32,
    ) -> Result<Self, Error> {
        let pool = PgPoolOptions::new()
            .max_connections(max_conn)
            .connect(conn_str).await?;
        Ok(Db { conn: pool })
    }

    pub async fn connect_default() -> Result<Self, Error> {
        Self::connect(
            "postgres://postgres:postgres@db:5432/ledger",
            5,
        ).await
    }

}

