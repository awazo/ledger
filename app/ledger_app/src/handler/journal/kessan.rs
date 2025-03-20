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
    .route("/to_misyuukin", post(to_misyuukin))
    .route("/to_mibaraikin", post(to_mibaraikin))
    .route("/sousai_syouhizei", post(sousai_syouhizei))
    .route("/sousai_owner", post(sousai_owner))
}

#[derive(Debug, Deserialize)]
struct KessanInput {
    date: NaiveDate,
    account: String,
    total: f32,
    desc: String,
}

impl KessanInput {

    fn into_journal_debit(&self, account: String) -> JournalInput {
        let mut debit = Vec::new();
        debit.push(AccountAmount {
            account,
            amount: self.total,
        });
        let mut credit = Vec::new();
        credit.push(AccountAmount {
            account: self.account.clone(),
            amount: self.total,
        });
        JournalInput {
            transaction_type: "Kessan".to_string(),
            date: self.date,
            debit,
            credit,
            desc: self.desc.clone(),
        }
    }

    fn into_journal_credit(&self, account: String) -> JournalInput {
        let mut debit = Vec::new();
        debit.push(AccountAmount {
            account: self.account.clone(),
            amount: self.total,
        });
        let mut credit = Vec::new();
        credit.push(AccountAmount {
            account,
            amount: self.total,
        });
        JournalInput {
            transaction_type: "Kessan".to_string(),
            date: self.date,
            debit,
            credit,
            desc: self.desc.clone(),
        }
    }

    async fn insert_debit(
        &self,
        db: &Db,
        account: String,
    ) -> Result<i32, Error> {
        Ok(self.into_journal_debit(account).into_transaction(db).await?
        .insert(db).await?)
    }

    async fn insert_credit(
        &self,
        db: &Db,
        account: String,
    ) -> Result<i32, Error> {
        Ok(self.into_journal_credit(account).into_transaction(db).await?
        .insert(db).await?)
    }

}

#[derive(Debug, Deserialize)]
struct KessanSousaiInput {
    date: NaiveDate,
    total: f32,
    desc: String,
}

impl KessanSousaiInput {

    fn into_journal(
        &self,
        account_debit: String,
        account_credit: String
    ) -> JournalInput {
        let mut debit = Vec::new();
        debit.push(AccountAmount {
            account: account_debit.clone(),
            amount: self.total,
        });
        let mut credit = Vec::new();
        credit.push(AccountAmount {
            account: account_credit.clone(),
            amount: self.total,
        });
        JournalInput {
            transaction_type: "Kessan".to_string(),
            date: self.date,
            debit,
            credit,
            desc: self.desc.clone(),
        }
    }

    async fn insert(
        &self,
        db: &Db,
        account_debit: String,
        account_credit: String,
    ) -> Result<i32, Error> {
        Ok(self.into_journal(account_debit, account_credit)
        .into_transaction(db).await?
        .insert(db).await?)
    }

}

async fn to_misyuukin(
    State(state): State<Arc<AppState>>,
    Json(input): Json<KessanInput>,
) -> (StatusCode, Json<ApiResponseWithoutBody>) {
    match input.insert_debit(&state.db, "未収金".to_string()).await {
        Ok(_) => (StatusCode::CREATED, Json(ApiResponse::ok_only())),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(Error::from(e).into_api_response()),
        ),
    }
}

async fn to_mibaraikin(
    State(state): State<Arc<AppState>>,
    Json(input): Json<KessanInput>,
) -> (StatusCode, Json<ApiResponseWithoutBody>) {
    match input.insert_credit(&state.db, "未払金".to_string()).await {
        Ok(_) => (StatusCode::CREATED, Json(ApiResponse::ok_only())),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(Error::from(e).into_api_response()),
        ),
    }
}

async fn sousai_syouhizei(
    State(state): State<Arc<AppState>>,
    Json(input): Json<KessanSousaiInput>,
) -> (StatusCode, Json<ApiResponseWithoutBody>) {
    match input.insert(
        &state.db,
        "仮受消費税".to_string(),
        "仮払消費税".to_string(),
    ).await {
        Ok(_) => (StatusCode::CREATED, Json(ApiResponse::ok_only())),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(Error::from(e).into_api_response()),
        ),
    }
}

async fn sousai_owner(
    State(state): State<Arc<AppState>>,
    Json(input): Json<KessanSousaiInput>,
) -> (StatusCode, Json<ApiResponseWithoutBody>) {
    match input.insert(
        &state.db,
        "事業主借".to_string(),
        "事業主貸".to_string(),
    ).await {
        Ok(_) => (StatusCode::CREATED, Json(ApiResponse::ok_only())),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(Error::from(e).into_api_response()),
        ),
    }
}


