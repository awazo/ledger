pub mod journal_payload;
pub mod from_prev;
pub mod buy;
pub mod sell;
pub mod bank;
pub mod kessan;
pub mod soneki;
pub mod to_next;

use std::sync::Arc;

use axum::{
    extract::{
        Path,
        State,
    },
    http::StatusCode,
    routing::get,
    Json,
    Router,
};
use chrono::{
    Datelike,
    TimeZone,
    Utc,
};
use chrono_tz::Japan;

use ledger_db::Transaction;

use crate::{
    ApiResponse,
    ApiResponseWithoutBody,
    AppState,
    Error,
};

use journal_payload::{
    AccountAmount,
    Journal,
};

pub(crate) fn build_router() -> Router<Arc<AppState>> {
    Router::new()
    .nest("/from_prev", from_prev::build_router())
    .nest("/buy", buy::build_router())
    .nest("/sell", sell::build_router())
    .nest("/bank", bank::build_router())
    .nest("/kessan", kessan::build_router())
    .nest("/soneki", soneki::build_router())
    .nest("/to_next", to_next::build_router())
    .route("/", get(show_journal_today).post(insert_journal))
    .route("/{year}/{month}", get(show_journal))
}

type JournalInput = Journal;
type JournalOutput = ApiResponse<Vec<Journal>>;

async fn show_journal_today(
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<JournalOutput>) {
    let now = Japan.from_utc_datetime(&Utc::now().naive_utc());
    let ym = (now.year(), now.month());
    let trans
        = match Transaction::by_month(&state.db, ym.0, ym.1).await {
            Ok(t) => t,
            Err(ledger_db::Error::RowNotFound) => Vec::new(),
            Err(e) => return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Error::from(e).into_api_response()),
            ),
        };
    let journals = trans.iter()
        .map(|t| Journal::from_transaction(t))
        .collect::<Vec<Journal>>();
    (StatusCode::OK, Json(JournalOutput::ok(journals)))
}

async fn insert_journal(
    State(state): State<Arc<AppState>>,
    Json(input): Json<JournalInput>,
) -> (StatusCode, Json<ApiResponseWithoutBody>) {
    let insert_result = match input.into_transaction(&state.db).await {
        Ok(tran) => tran.insert(&state.db).await,
        Err(e) => return (
            StatusCode::BAD_REQUEST,
            Json(Error::from(e).into_api_response()),
        ),
    };
    match insert_result {
        Ok(_) => (StatusCode::CREATED, Json(ApiResponse::ok_only())),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(Error::from(e).into_api_response()),
        ),
    }
}

async fn show_journal(
    Path(ym): Path<(i32, u32)>,
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<JournalOutput>) {
    let trans
        = match Transaction::by_month(&state.db, ym.0, ym.1).await {
            Ok(t) => t,
            Err(ledger_db::Error::RowNotFound) => Vec::new(),
            Err(e) => return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Error::from(e).into_api_response()),
            ),
        };
    let journals = trans.iter()
        .map(|t| Journal::from_transaction(t))
        .collect::<Vec<Journal>>();
    (StatusCode::OK, Json(JournalOutput::ok(journals)))
}

