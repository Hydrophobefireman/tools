use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Error {
    error: String,
}
pub fn err_handler<T: ToString>(err: T) -> Json<Error> {
    Json(Error {
        error: err.to_string(),
    })
}
