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
    .route("/to_shihonkin_plus", post(to_shihonkin_plus))
    .route("/to_shihonkin_minus", post(to_shihonkin_minus))
    .route("/debit", post(debit))
    .route("/credit", post(credit))
}

#[derive(Debug, Deserialize)]
struct ToNextInput {
    date: NaiveDate,
    account: String,
    total: f32,
    desc: String,
}

impl ToNextInput {

    fn into_journal_shihonkin(&self, side: &AmountSide) -> JournalInput {
        let mut debit = Vec::new();
        let mut credit = Vec::new();
        match side {
            AmountSide::Debit => {
                debit.push(AccountAmount {
                    account: "資本金".to_string(),
                    amount: self.total,
                });
                credit.push(AccountAmount {
                    account: self.account.clone(),
                    amount: self.total,
                });
            },
            AmountSide::Credit => {
                debit.push(AccountAmount {
                    account: self.account.clone(),
                    amount: self.total,
                });
                credit.push(AccountAmount {
                    account: "資本金".to_string(),
                    amount: self.total,
                });
            },
        }
        JournalInput {
            transaction_type: "ToNext".to_string(),
            date: self.date,
            debit,
            credit,
            desc: self.desc.clone(),
        }
    }

    fn into_journal(&self, side: &AmountSide) -> JournalInput {
        let mut debit = Vec::new();
        let mut credit = Vec::new();
        match side {
            AmountSide::Debit => {
                debit.push(AccountAmount {
                    account: "(次期繰越(借方勘定用))".to_string(),
                    amount: self.total,
                });
                credit.push(AccountAmount {
                    account: self.account.clone(),
                    amount: self.total,
                });
            },
            AmountSide::Credit => {
                debit.push(AccountAmount {
                    account: self.account.clone(),
                    amount: self.total,
                });
                credit.push(AccountAmount {
                    account: "(次期繰越(貸方勘定用))".to_string(),
                    amount: self.total,
                });
            },
        }
        JournalInput {
            transaction_type: "ToNext".to_string(),
            date: self.date,
            debit,
            credit,
            desc: self.desc.clone(),
        }
    }

    async fn insert_shihonkin(
        &self,
        db: &Db,
        side: &AmountSide,
    ) -> Result<i32, Error> {
        Ok(self.into_journal_shihonkin(side).into_transaction(db).await?
        .insert(db).await?)
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

async fn to_shihonkin_plus(
    State(state): State<Arc<AppState>>,
    Json(input): Json<ToNextInput>,
) -> (StatusCode, Json<ApiResponseWithoutBody>) {
    match input.insert_shihonkin(&state.db, &AmountSide::Credit).await {
        Ok(_) => (StatusCode::CREATED, Json(ApiResponse::ok_only())),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(Error::from(e).into_api_response()),
        ),
    }
}

async fn to_shihonkin_minus(
    State(state): State<Arc<AppState>>,
    Json(input): Json<ToNextInput>,
) -> (StatusCode, Json<ApiResponseWithoutBody>) {
    match input.insert_shihonkin(&state.db, &AmountSide::Debit).await {
        Ok(_) => (StatusCode::CREATED, Json(ApiResponse::ok_only())),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(Error::from(e).into_api_response()),
        ),
    }
}

async fn debit(
    State(state): State<Arc<AppState>>,
    Json(input): Json<ToNextInput>,
) -> (StatusCode, Json<ApiResponseWithoutBody>) {
    match input.insert(&state.db, &AmountSide::Debit).await {
        Ok(_) => (StatusCode::CREATED, Json(ApiResponse::ok_only())),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(Error::from(e).into_api_response()),
        ),
    }
}

async fn credit(
    State(state): State<Arc<AppState>>,
    Json(input): Json<ToNextInput>,
) -> (StatusCode, Json<ApiResponseWithoutBody>) {
    match input.insert(&state.db, &AmountSide::Credit).await {
        Ok(_) => (StatusCode::CREATED, Json(ApiResponse::ok_only())),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(Error::from(e).into_api_response()),
        ),
    }
}

