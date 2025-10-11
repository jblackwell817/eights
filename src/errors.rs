use axum::response::IntoResponse;

/// Custom error type for API responses
pub enum ApiError {
    BadRequest(String),
    InternalServerError,
}

impl From<serde_json::Error> for ApiError {
    fn from(_: serde_json::Error) -> Self {
        ApiError::InternalServerError
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiError::BadRequest(s) => (axum::http::StatusCode::BAD_REQUEST, s).into_response(),
            ApiError::InternalServerError => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
            )
                .into_response(),
        }
    }
}
