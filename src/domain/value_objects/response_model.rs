use axum::{http::{header, StatusCode}, response::IntoResponse, Json};
use serde::Serialize;
use serde_json::json;

pub struct ResponseObject <T: Serialize> {
    pub status_code: StatusCode,
    pub success: bool,
    pub message: String,
    pub result: Option<T>
}

impl <T: Serialize> IntoResponse for ResponseObject <T> {

    fn into_response(self) -> axum::response::Response {
        (self.status_code, [(header::CONTENT_TYPE, "application/json")], Json(json!({
            "success" : self.success,
            "message" : self.message,
            "result" : self.result
        }))).into_response()
    }

}