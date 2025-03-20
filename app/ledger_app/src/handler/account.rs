use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    routing::{
        get,
        post,
    },
    Json,
    Router,
};
use serde::{
    Deserialize,
    Serialize,
};

use ledger_db::Db;

use crate::{
    ApiResponse,
    ApiResponseWithoutBody,
    AppState,
    Error,
};

pub(crate) fn build_router() -> Router<Arc<AppState>> {
    Router::new()
    .route("/", get(show_account).post(insert_account))
    .route("/asset", post(insert_asset))
    .route("/liability", post(insert_liability))
    .route("/equity", post(insert_equity))
    .route("/income", post(insert_income))
    .route("/expense", post(insert_expense))
    .route("/util_debit", post(insert_util_debit))
    .route("/util_credit", post(insert_util_credit))
}

#[derive(Debug, Serialize, Deserialize)]
struct Account {
    account_name: String,
    account_type: String,
    amount_side: Option<String>,
}

impl Account {

    fn into_db_account(&self) -> ledger_db::Account {
        ledger_db::Account {
            account_id: 0,
            account_name: self.account_name.clone(),
            account_type: (&self.account_type).into(),
        }
    }

    fn from_db_account(account: &ledger_db::Account) -> Self {
        let amount_side = Some(
            account.account_type.amount_side().into_japanese()
        );
        Account {
            account_name: account.account_name.clone(),
            account_type: account.account_type.into_japanese(),
            amount_side,
        }
    }

}

type AccountInput = Account;
type AccountOutput = ApiResponse<Vec<Account>>;

async fn show_account(
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<AccountOutput>) {
    let db_acc = match ledger_db::Account::all(&state.db).await {
        Ok(a) => a,
        Err(ledger_db::Error::RowNotFound) => Vec::new(),
        Err(e) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Error::from(e).into_api_response()),
        ),
    };
    let acc = db_acc.iter()
        .map(|a| Account::from_db_account(a))
        .collect::<Vec<Account>>();
    (StatusCode::OK, Json(AccountOutput::ok(acc)))
}

async fn insert_account(
    State(state): State<Arc<AppState>>,
    Json(input): Json<AccountInput>,
) -> (StatusCode, Json<ApiResponseWithoutBody>) {
    match input.into_db_account().insert(&state.db).await {
        Ok(_) => (StatusCode::CREATED, Json(ApiResponse::ok_only())),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(Error::from(e).into_api_response()),
        ),
    }
}

#[derive(Debug, Deserialize)]
struct AccountNameInput {
    name: String,
}

async fn insert_account_name(
    db: &Db,
    account_name: &str,
    account_type: &str,
) -> (StatusCode, Json<ApiResponseWithoutBody>) {
    let acc = Account {
        account_name: account_name.to_string(),
        account_type: account_type.to_string(),
        amount_side: None,
    };
    match acc.into_db_account().insert(db).await {
        Ok(_) => (StatusCode::CREATED, Json(ApiResponse::ok_only())),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(Error::from(e).into_api_response()),
        ),
    }
}

async fn insert_asset(
    State(state): State<Arc<AppState>>,
    Json(input): Json<AccountNameInput>,
) -> (StatusCode, Json<ApiResponseWithoutBody>) {
    insert_account_name(&state.db, &input.name, "Asset").await
}

async fn insert_liability(
    State(state): State<Arc<AppState>>,
    Json(input): Json<AccountNameInput>,
) -> (StatusCode, Json<ApiResponseWithoutBody>) {
    insert_account_name(&state.db, &input.name, "Liability").await
}

async fn insert_equity(
    State(state): State<Arc<AppState>>,
    Json(input): Json<AccountNameInput>,
) -> (StatusCode, Json<ApiResponseWithoutBody>) {
    insert_account_name(&state.db, &input.name, "Equity").await
}

async fn insert_income(
    State(state): State<Arc<AppState>>,
    Json(input): Json<AccountNameInput>,
) -> (StatusCode, Json<ApiResponseWithoutBody>) {
    insert_account_name(&state.db, &input.name, "Income").await
}

async fn insert_expense(
    State(state): State<Arc<AppState>>,
    Json(input): Json<AccountNameInput>,
) -> (StatusCode, Json<ApiResponseWithoutBody>) {
    insert_account_name(&state.db, &input.name, "Expense").await
}

async fn insert_util_debit(
    State(state): State<Arc<AppState>>,
    Json(input): Json<AccountNameInput>,
) -> (StatusCode, Json<ApiResponseWithoutBody>) {
    insert_account_name(&state.db, &input.name, "UtilDebit").await
}

async fn insert_util_credit(
    State(state): State<Arc<AppState>>,
    Json(input): Json<AccountNameInput>,
) -> (StatusCode, Json<ApiResponseWithoutBody>) {
    insert_account_name(&state.db, &input.name, "UtilCredit").await
}

