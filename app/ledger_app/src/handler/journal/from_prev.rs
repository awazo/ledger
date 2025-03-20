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
    .route("/debit", post(from_prev_debit))
    .route("/credit", post(from_prev_credit))
}

#[derive(Debug, Deserialize)]
struct FromPrevInput {
    date: NaiveDate,
    account: String,
    total: f32,
    desc: String,
}

impl FromPrevInput {

    fn into_journal(&self, side: &AmountSide) -> JournalInput {
        let mut debit = Vec::new();
        let mut credit = Vec::new();
        match side {
            AmountSide::Debit => {
                debit.push(AccountAmount {
                    account: self.account.clone(),
                    amount: self.total,
                });
                credit.push(AccountAmount {
                    account: "(前期繰越(借方勘定用))".to_string(),
                    amount: self.total,
                });
            },
            AmountSide::Credit => {
                debit.push(AccountAmount {
                    account: "(前期繰越(貸方勘定用))".to_string(),
                    amount: self.total,
                });
                credit.push(AccountAmount {
                    account: self.account.clone(),
                    amount: self.total,
                });
            },
        }
        JournalInput {
            transaction_type: "FromPrev".to_string(),
            date: self.date,
            debit,
            credit,
            desc: self.desc.clone(),
        }
    }

    async fn insert(
        &self,
        db: &Db,
        side: &AmountSide,
    ) -> Result<i32, Error> {
        Ok(self.into_journal(side).into_transaction(db).await?
        .insert(db).await?)
    }

}

async fn from_prev_debit(
    State(state): State<Arc<AppState>>,
    Json(input): Json<FromPrevInput>,
) -> (StatusCode, Json<ApiResponseWithoutBody>) {
    match input.insert(&state.db, &AmountSide::Debit).await {
        Ok(_) => (StatusCode::CREATED, Json(ApiResponse::ok_only())),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(Error::from(e).into_api_response()),
        ),
    }
}

async fn from_prev_credit(
    State(state): State<Arc<AppState>>,
    Json(input): Json<FromPrevInput>,
) -> (StatusCode, Json<ApiResponseWithoutBody>) {
    match input.insert(&state.db, &AmountSide::Credit).await {
        Ok(_) => (StatusCode::CREATED, Json(ApiResponse::ok_only())),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(Error::from(e).into_api_response()),
        ),
    }
}

