use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    routing::post,
    Json,
    Router,
};
use chrono::NaiveDate;
use serde::Deserialize;

use ledger_db::Db;

use crate::{
    ApiResponse,
    ApiResponseWithoutBody,
    AppState,
    Error,
};

use super::{
    AccountAmount,
    JournalInput,
};

pub(crate) fn build_router() -> Router<Arc<AppState>> {
    Router::new()
    .route("/income", post(soneki_income))
    .route("/expense", post(soneki_expense))
}

#[derive(Debug, Deserialize)]
struct SonekiInput {
    date: NaiveDate,
    account: String,
    total: f32,
    desc: String,
}

impl SonekiInput {

    fn into_journal_income(&self) -> JournalInput {
        let mut debit = Vec::new();
        debit.push(AccountAmount {
            account: self.account.clone(),
            amount: self.total,
        });
        let mut credit = Vec::new();
        credit.push(AccountAmount {
            account: "損益".to_string(),
            amount: self.total,
        });
        JournalInput {
            transaction_type: "Soneki".to_string(),
            date: self.date,
            debit,
            credit,
            desc: self.desc.clone(),
        }
    }

    fn into_journal_expense(&self) -> JournalInput {
        let mut debit = Vec::new();
        debit.push(AccountAmount {
            account: "損益".to_string(),
            amount: self.total,
        });
        let mut credit = Vec::new();
        credit.push(AccountAmount {
            account: self.account.clone(),
            amount: self.total,
        });
        JournalInput {
            transaction_type: "Soneki".to_string(),
            date: self.date,
            debit,
            credit,
            desc: self.desc.clone(),
        }
    }

    async fn insert_income(&self, db: &Db) -> Result<i32, Error> {
        Ok(self.into_journal_income().into_transaction(db).await?
        .insert(db).await?)
    }

    async fn insert_expense(&self, db: &Db) -> Result<i32, Error> {
        Ok(self.into_journal_expense().into_transaction(db).await?
        .insert(db).await?)
    }

}

async fn soneki_income(
    State(state): State<Arc<AppState>>,
    Json(input): Json<SonekiInput>,
) -> (StatusCode, Json<ApiResponseWithoutBody>) {
    match input.insert_income(&state.db).await {
        Ok(_) => (StatusCode::CREATED, Json(ApiResponse::ok_only())),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(Error::from(e).into_api_response()),
        ),
    }
}

async fn soneki_expense(
    State(state): State<Arc<AppState>>,
    Json(input): Json<SonekiInput>,
) -> (StatusCode, Json<ApiResponseWithoutBody>) {
    match input.insert_expense(&state.db).await {
        Ok(_) => (StatusCode::CREATED, Json(ApiResponse::ok_only())),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(Error::from(e).into_api_response()),
        ),
    }
}

