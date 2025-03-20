mod api_response;
mod handler;

use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use thiserror::Error;

use ledger_db::Db;

use api_response::{
    ApiResponse,
    ApiResponseWithoutBody,
};

#[derive(Debug, Error)]
enum Error {
    #[error(transparent)]
    DataBaseError(#[from] ledger_db::Error),
    #[error("account '{0}' not found")]
    AccountNotFound(String),
    #[error("'{0}' can not convert to datetime")]
    DateTimeError(String),
}

impl Error {

    fn into_api_response<T>(&self) -> ApiResponse<T> {
        ApiResponse {
            status: self.to_string(),
            message: format!("{}", self),
            body: None,
        }
    }

}

#[derive(Debug)]
struct AppState {
    db: Db,
}

#[tokio::main]
async fn main() {
    let app_state = Arc::new(AppState {
        db: Db::connect_default().await.unwrap(),
    });

    let router = Router::new()
        .route("/", get(root))
        .nest("/account", handler::account::build_router())
        .nest("/journal", handler::journal::build_router())
        .nest("/summary", handler::summary::build_router())
        .with_state(app_state);

    let listener
        = tokio::net::TcpListener::bind("0.0.0.0:2480").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

async fn root() -> impl IntoResponse {
    (StatusCode::OK, "ledger version 0.1".to_string())
}

