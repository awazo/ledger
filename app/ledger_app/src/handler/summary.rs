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
    Days,
    NaiveDate,
    Months,
};
use serde::Serialize;

use ledger_db::{
    AmountSide,
    Db,
    Summary as DbSummary,
};

use crate::{
    ApiResponse,
    AppState,
    Error,
};

pub(crate) fn build_router() -> Router<Arc<AppState>> {
    Router::new()
    .route("/{y}", get(show_year_in_term))
    .route("/{y}/from_prev", get(show_year_from_prev))
    .route("/{y}/kessan", get(show_year_kessan))
    .route("/{y}/soneki", get(show_year_soneki))
    .route("/{y}/to_next", get(show_year_to_next))
    .route("/{y}/{m}", get(show_month_in_term))
    .route("/{y}/{m}/from_prev", get(show_month_from_prev))
    .route("/{y}/{m}/kessan", get(show_month_kessan))
    .route("/{y}/{m}/soneki", get(show_month_soneki))
    .route("/{y}/{m}/to_next", get(show_month_to_next))
}

#[derive(Debug, Serialize)]
struct Summary {
    account_name: String,
    debit: f32,
    credit: f32,
}

impl Summary {

    fn from_db_summary(db_summary: &DbSummary) -> Summary {
        match db_summary.account_type.amount_side() {
            AmountSide::Debit => Summary {
                account_name: db_summary.account_name.clone(),
                debit: db_summary.debit - db_summary.credit,
                credit: 0_f32,
            },
            AmountSide::Credit => Summary {
                account_name: db_summary.account_name.clone(),
                debit: 0_f32,
                credit: db_summary.credit - db_summary.debit,
            },
        }
    }

}

type SummaryOutput = ApiResponse<Vec<Summary>>;

fn get_period_year(y: i32) -> Option<(NaiveDate, NaiveDate)> {
    let start = NaiveDate::from_ymd_opt(y, 1, 1);
    let end = NaiveDate::from_ymd_opt(y, 12, 31);
    match (start, end) {
        (Some(s), Some(e)) => Some((s, e)),
        _ => None,
    }
}

fn get_period_month(y: i32, m: u32) -> Option<(NaiveDate, NaiveDate)> {
    let start = match NaiveDate::from_ymd_opt(y, m, 1) {
        Some(s) => s,
        None => return None,
    };
    let end = start + Months::new(1) - Days::new(1);
    Some((start, end))
}

async fn from_db_from_prev(
    db: &Db,
    start: NaiveDate,
) -> Result<Vec<Summary>, Error> {
    let db_summary
        = match DbSummary::upto_from_prev(db, start).await {
            Ok(ds) => ds,
            Err(ledger_db::Error::RowNotFound) => Vec::new(),
            Err(e) => return Err(Error::from(e)),
        };
    Ok(db_summary.iter()
    .map(|s| Summary::from_db_summary(s)).collect::<Vec<Summary>>())
}

async fn from_db_in_term(
    db: &Db,
    start: NaiveDate,
    end: NaiveDate
) -> Result<Vec<Summary>, Error> {
    let db_summary
        = match DbSummary::upto_in_term(db, start, end).await {
            Ok(ds) => ds,
            Err(ledger_db::Error::RowNotFound) => Vec::new(),
            Err(e) => return Err(Error::from(e)),
        };
    Ok(db_summary.iter()
    .map(|s| Summary::from_db_summary(s)).collect::<Vec<Summary>>())
}

async fn from_db_kessan(
    db: &Db,
    start: NaiveDate,
    end: NaiveDate
) -> Result<Vec<Summary>, Error> {
    let db_summary
        = match DbSummary::upto_kessan(db, start, end).await {
            Ok(ds) => ds,
            Err(ledger_db::Error::RowNotFound) => Vec::new(),
            Err(e) => return Err(Error::from(e)),
        };
    Ok(db_summary.iter()
    .map(|s| Summary::from_db_summary(s)).collect::<Vec<Summary>>())
}

async fn from_db_soneki(
    db: &Db,
    start: NaiveDate,
    end: NaiveDate
) -> Result<Vec<Summary>, Error> {
    let db_summary
        = match DbSummary::upto_soneki(db, start, end).await {
            Ok(ds) => ds,
            Err(ledger_db::Error::RowNotFound) => Vec::new(),
            Err(e) => return Err(Error::from(e)),
        };
    Ok(db_summary.iter()
    .map(|s| Summary::from_db_summary(s)).collect::<Vec<Summary>>())
}

async fn from_db_to_next(
    db: &Db,
    start: NaiveDate,
    end: NaiveDate
) -> Result<Vec<Summary>, Error> {
    let db_summary
        = match DbSummary::upto_to_next(db, start, end).await {
            Ok(ds) => ds,
            Err(ledger_db::Error::RowNotFound) => Vec::new(),
            Err(e) => return Err(Error::from(e)),
        };
    Ok(db_summary.iter()
    .map(|s| Summary::from_db_summary(s)).collect::<Vec<Summary>>())
}

async fn show_year_from_prev(
    Path(y): Path<i32>,
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<SummaryOutput>) {
    let (start, _end) = match get_period_year(y) {
        Some(p) => p,
        None => return (
            StatusCode::BAD_REQUEST,
            Json(Error::DateTimeError(format!("year {}", y))
                .into_api_response()),
        ),
    };
    let summary = match from_db_from_prev(&state.db, start).await {
        Ok(s) => s,
        Err(e) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(e.into_api_response()),
        ),
    };
    (StatusCode::OK, Json(SummaryOutput::ok(summary)))
}

async fn show_year_in_term(
    Path(y): Path<i32>,
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<SummaryOutput>) {
    let (start, end) = match get_period_year(y) {
        Some(p) => p,
        None => return (
            StatusCode::BAD_REQUEST,
            Json(Error::DateTimeError(format!("year {}", y))
                .into_api_response()),
        ),
    };
    let summary = match from_db_in_term(&state.db, start, end).await {
        Ok(s) => s,
        Err(e) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(e.into_api_response()),
        ),
    };
    (StatusCode::OK, Json(SummaryOutput::ok(summary)))
}

async fn show_year_kessan(
    Path(y): Path<i32>,
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<SummaryOutput>) {
    let (start, end) = match get_period_year(y) {
        Some(p) => p,
        None => return (
            StatusCode::BAD_REQUEST,
            Json(Error::DateTimeError(format!("year {}", y))
                .into_api_response()),
        ),
    };
    let summary = match from_db_kessan(&state.db, start, end).await {
        Ok(s) => s,
        Err(e) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(e.into_api_response()),
        ),
    };
    (StatusCode::OK, Json(SummaryOutput::ok(summary)))
}

async fn show_year_soneki(
    Path(y): Path<i32>,
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<SummaryOutput>) {
    let (start, end) = match get_period_year(y) {
        Some(p) => p,
        None => return (
            StatusCode::BAD_REQUEST,
            Json(Error::DateTimeError(format!("year {}", y))
                .into_api_response()),
        ),
    };
    let summary = match from_db_soneki(&state.db, start, end).await {
        Ok(s) => s,
        Err(e) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(e.into_api_response()),
        ),
    };
    (StatusCode::OK, Json(SummaryOutput::ok(summary)))
}

async fn show_year_to_next(
    Path(y): Path<i32>,
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<SummaryOutput>) {
    let (start, end) = match get_period_year(y) {
        Some(p) => p,
        None => return (
            StatusCode::BAD_REQUEST,
            Json(Error::DateTimeError(format!("year {}", y))
                .into_api_response()),
        ),
    };
    let summary = match from_db_to_next(&state.db, start, end).await {
        Ok(s) => s,
        Err(e) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(e.into_api_response()),
        ),
    };
    (StatusCode::OK, Json(SummaryOutput::ok(summary)))
}

async fn show_month_from_prev(
    Path(ym): Path<(i32, u32)>,
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<SummaryOutput>) {
    let (start, _end) = match get_period_month(ym.0, ym.1) {
        Some(p) => p,
        None => return (
            StatusCode::BAD_REQUEST,
            Json(Error::DateTimeError(format!("{}-{}-1", ym.0, ym.1))
                .into_api_response()),
        ),
    };
    let summary = match from_db_from_prev(&state.db, start).await {
        Ok(s) => s,
        Err(e) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(e.into_api_response()),
        ),
    };
    (StatusCode::OK, Json(SummaryOutput::ok(summary)))
}

async fn show_month_in_term(
    Path(ym): Path<(i32, u32)>,
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<SummaryOutput>) {
    let (start, end) = match get_period_month(ym.0, ym.1) {
        Some(p) => p,
        None => return (
            StatusCode::BAD_REQUEST,
            Json(Error::DateTimeError(format!("{}-{}-1", ym.0, ym.1))
                .into_api_response()),
        ),
    };
    let summary = match from_db_in_term(&state.db, start, end).await {
        Ok(s) => s,
        Err(e) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(e.into_api_response()),
        ),
    };
    (StatusCode::OK, Json(SummaryOutput::ok(summary)))
}

async fn show_month_kessan(
    Path(ym): Path<(i32, u32)>,
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<SummaryOutput>) {
    let (start, end) = match get_period_month(ym.0, ym.1) {
        Some(p) => p,
        None => return (
            StatusCode::BAD_REQUEST,
            Json(Error::DateTimeError(format!("{}-{}-1", ym.0, ym.1))
                .into_api_response()),
        ),
    };
    let summary = match from_db_kessan(&state.db, start, end).await {
        Ok(s) => s,
        Err(e) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(e.into_api_response()),
        ),
    };
    (StatusCode::OK, Json(SummaryOutput::ok(summary)))
}

async fn show_month_soneki(
    Path(ym): Path<(i32, u32)>,
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<SummaryOutput>) {
    let (start, end) = match get_period_month(ym.0, ym.1) {
        Some(p) => p,
        None => return (
            StatusCode::BAD_REQUEST,
            Json(Error::DateTimeError(format!("{}-{}-1", ym.0, ym.1))
                .into_api_response()),
        ),
    };
    let summary = match from_db_soneki(&state.db, start, end).await {
        Ok(s) => s,
        Err(e) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(e.into_api_response()),
        ),
    };
    (StatusCode::OK, Json(SummaryOutput::ok(summary)))
}

async fn show_month_to_next(
    Path(ym): Path<(i32, u32)>,
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<SummaryOutput>) {
    let (start, end) = match get_period_month(ym.0, ym.1) {
        Some(p) => p,
        None => return (
            StatusCode::BAD_REQUEST,
            Json(Error::DateTimeError(format!("{}-{}-1", ym.0, ym.1))
                .into_api_response()),
        ),
    };
    let summary = match from_db_to_next(&state.db, start, end).await {
        Ok(s) => s,
        Err(e) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(e.into_api_response()),
        ),
    };
    (StatusCode::OK, Json(SummaryOutput::ok(summary)))
}

