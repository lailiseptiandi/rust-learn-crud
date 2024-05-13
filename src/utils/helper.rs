use actix_web::{http::StatusCode, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub status: String,
    pub code: u16,
    pub message: String,
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn new(status: &str, code: u16, message: &str, data: Option<T>) -> Self {
        ApiResponse {
            status: status.to_string(),
            code,
            message: message.to_string(),
            data,
        }
    }
}

pub fn json_response<T: Serialize>(
    status: &str,
    code: u16,
    message: &str,
    data: Option<T>,
) -> HttpResponse {
    let api_response = ApiResponse::new(status, code, message, data);
    let json = serde_json::to_string(&api_response).unwrap();
    HttpResponse::build(StatusCode::from_u16(code).unwrap())
        .content_type("application/json")
        .body(json)
}
