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
    .route("/by_owner", post(by_owner))
    .route("/by_bank", post(by_bank))
    .route("/by_kaikakekin", post(by_kaikakekin))
    .route("/by_maebaraikin", post(by_maebaraikin))
}

#[derive(Debug, Deserialize)]
struct BuyInput {
    date: NaiveDate,
    account: String,
    total: f32,
    tax: Option<f32>,
    desc: String,
}

impl BuyInput {

    fn into_journal(&self, account_buy: String) -> JournalInput {
        let mut debit = Vec::new();
        debit.push(AccountAmount {
            account: self.account.clone(),
            amount: self.total - self.tax.unwrap_or(0_f32),
        });
        if let Some(tax) = self.tax {
            debit.push(AccountAmount {
                account: "仮払消費税".to_string(),
                amount: tax,
            });
        }
        let mut credit = Vec::new();
        credit.push(AccountAmount {
            account: account_buy.clone(),
            amount: self.total,
        });
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
        account_buy: String,
    ) -> Result<i32, Error> {
        Ok(self.into_journal(account_buy).into_transaction(db).await?
        .insert(db).await?)
    }

}

async fn by_owner(
    State(state): State<Arc<AppState>>,
    Json(input): Json<BuyInput>,
) -> (StatusCode, Json<ApiResponseWithoutBody>) {
    match input.insert(&state.db, "事業主借".to_string()).await {
        Ok(_) => (StatusCode::CREATED, Json(ApiResponse::ok_only())),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(Error::from(e).into_api_response()),
        ),
    }
}

async fn by_bank(
    State(state): State<Arc<AppState>>,
    Json(input): Json<BuyInput>,
) -> (StatusCode, Json<ApiResponseWithoutBody>) {
    match input.insert(&state.db, "普通預金".to_string()).await {
        Ok(_) => (StatusCode::CREATED, Json(ApiResponse::ok_only())),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(Error::from(e).into_api_response()),
        ),
    }
}

async fn by_kaikakekin(
    State(state): State<Arc<AppState>>,
    Json(input): Json<BuyInput>,
) -> (StatusCode, Json<ApiResponseWithoutBody>) {
    match input.insert(&state.db, "買掛金".to_string()).await {
        Ok(_) => (StatusCode::CREATED, Json(ApiResponse::ok_only())),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(Error::from(e).into_api_response()),
        ),
    }
}

async fn by_maebaraikin(
    State(state): State<Arc<AppState>>,
    Json(input): Json<BuyInput>,
) -> (StatusCode, Json<ApiResponseWithoutBody>) {
    match input.insert(&state.db, "前払金".to_string()).await {
        Ok(_) => (StatusCode::CREATED, Json(ApiResponse::ok_only())),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(Error::from(e).into_api_response()),
        ),
    }
}

