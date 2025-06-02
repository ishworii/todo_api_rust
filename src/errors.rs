use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum AppError {
    SqlxError(sqlx::Error),
    NotFound,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status_code, error_message) = match self {
            AppError::SqlxError(e) => {
                eprintln!("SQLX Error : {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong".to_string(),
                )
            }
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not Found".to_string()),
        };
        (status_code, error_message).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => AppError::NotFound,
            _ => AppError::SqlxError(e),
        }
    }
}
