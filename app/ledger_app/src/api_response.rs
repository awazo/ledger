use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub status: String,
    pub message: String,
    pub body: Option<T>,
}

impl<T> ApiResponse<T> {

    pub fn ok_only() -> Self {
        ApiResponse::ok_only_with("".to_string())
    }

    pub fn ok_only_with(message: String) -> Self {
        ApiResponse {
            status: "OK".to_string(),
            message,
            body: None,
        }
    }

    pub fn ok(body: T) -> Self {
        ApiResponse::ok_with("".to_string(), body)
    }

    pub fn ok_with(message: String, body: T) -> Self {
        ApiResponse {
            status: "OK".to_string(),
            message,
            body: Some(body),
        }
    }

}

pub type ApiResponseWithoutBody = ApiResponse<()>;

