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
    .route("/by_bank", post(by_bank))
    .route("/by_urikakekin", post(by_urikakekin))
    .route("/by_maeukekin", post(by_maeukekin))
}

#[derive(Debug, Deserialize)]
struct SellInput {
    date: NaiveDate,
    account: String,
    total: f32,
    tax: Option<f32>,
    desc: String,
}

impl SellInput {

    fn into_journal(&self, account_sell: String) -> JournalInput {
        let mut debit = Vec::new();
        debit.push(AccountAmount {
            account: account_sell.clone(),
            amount: self.total,
        });
        let mut credit = Vec::new();
        credit.push(AccountAmount {
            account: self.account.clone(),
            amount: self.total - self.tax.unwrap_or(0_f32),
        });
        if let Some(tax) = self.tax {
            credit.push(AccountAmount {
                account: "仮受消費税".to_string(),
                amount: tax,
            });
        }
        JournalInput {
            transaction_type: "InTerm".to_string(),
            date: self.date,
            debit,
            credit,
            desc: self.desc.clone(),
        }
    }

    async fn insert(
        &self,
        db: &Db,
        account_sell: String
    ) -> Result<i32, Error> {
        Ok(self.into_journal(account_sell).into_transaction(db).await?
        .insert(db).await?)
    }

}

async fn by_bank(
    State(state): State<Arc<AppState>>,
    Json(input): Json<SellInput>,
) -> (StatusCode, Json<ApiResponseWithoutBody>) {
    match input.insert(&state.db, "普通預金".to_string()).await {
        Ok(_) => (StatusCode::CREATED, Json(ApiResponse::ok_only())),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(Error::from(e).into_api_response()),
        ),
    }
}

async fn by_urikakekin(
    State(state): State<Arc<AppState>>,
    Json(input): Json<SellInput>,
) -> (StatusCode, Json<ApiResponseWithoutBody>) {
    match input.insert(&state.db, "売掛金".to_string()).await {
        Ok(_) => (StatusCode::CREATED, Json(ApiResponse::ok_only())),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(Error::from(e).into_api_response()),
        ),
    }
}

async fn by_maeukekin(
    State(state): State<Arc<AppState>>,
    Json(input): Json<SellInput>,
) -> (StatusCode, Json<ApiResponseWithoutBody>) {
    match input.insert(&state.db, "前受金".to_string()).await {
        Ok(_) => (StatusCode::CREATED, Json(ApiResponse::ok_only())),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(Error::from(e).into_api_response()),
        ),
    }
}

