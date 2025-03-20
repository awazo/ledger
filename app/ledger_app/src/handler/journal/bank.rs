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

use ledger_db::{
    AmountSide,
    Db,
};

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
    .route("/to_owner", post(to_owner))
    .route("/from_owner", post(from_owner))
}

#[derive(Debug, Deserialize)]
struct BankInput {
    date: NaiveDate,
    total: f32,
    desc: String,
}

impl BankInput {

    fn into_journal(&self, bank_side: &AmountSide) -> JournalInput {
        let bank = AccountAmount {
            account: "普通預金".to_string(),
            amount: self.total,
        };
        let mut debit = Vec::new();
        let mut credit = Vec::new();
        match bank_side {
            AmountSide::Debit => {
                debit.push(bank);
                credit.push(AccountAmount {
                    account: "事業主借".to_string(),
                    amount: self.total,
                });
            },
            AmountSide::Credit => {
                debit.push(AccountAmount {
                    account: "事業主貸".to_string(),
                    amount: self.total,
                });
                credit.push(bank);
            },
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
        bank_side: &AmountSide,
    ) -> Result<i32, Error> {
        Ok(self.into_journal(bank_side).into_transaction(db).await?
        .insert(db).await?)
    }

}

async fn to_owner(
    State(state): State<Arc<AppState>>,
    Json(input): Json<BankInput>,
) -> (StatusCode, Json<ApiResponseWithoutBody>) {
    match input.insert(&state.db, &AmountSide::Credit).await {
        Ok(_) => (StatusCode::CREATED, Json(ApiResponse::ok_only())),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(Error::from(e).into_api_response()),
        ),
    }
}

async fn from_owner(
    State(state): State<Arc<AppState>>,
    Json(input): Json<BankInput>,
) -> (StatusCode, Json<ApiResponseWithoutBody>) {
    match input.insert(&state.db, &AmountSide::Debit).await {
        Ok(_) => (StatusCode::CREATED, Json(ApiResponse::ok_only())),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(Error::from(e).into_api_response()),
        ),
    }
}

