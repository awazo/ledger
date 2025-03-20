mod db;
mod account;
mod transaction;
mod summary;

use std::convert::From;
use thiserror::Error;

pub use db::*;
pub use account::*;
pub use transaction::*;
pub use summary::*;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    SqlError(sqlx::Error),
    #[error("row not found")]
    RowNotFound,
    #[error("account not found")]
    AccountNotFound,
    #[error("illegal datetime")]
    DateTimeError,
}

impl From<sqlx::Error> for Error {

    fn from(
        value: sqlx::Error,
    ) -> Self {
        match value {
            sqlx::Error::RowNotFound => Self::RowNotFound,
            _ => Self::SqlError(value),
        }
    }

}

