use diesel::result::Error as DieselError;
use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Value;
use std::convert::From;
use std::io::Cursor;

pub mod api_token_fairing;
pub mod demo_fairing;
pub mod log_fairing;

pub mod asset;
pub mod errors;
pub mod token_api_auth;
pub mod user;
pub mod user_token;

// 响应返回 结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct APIResponse {
    code: u32,
    msg: String,
    data: Value,
}

impl APIResponse {
    pub fn new(code: u32, msg: &str, data: Value) -> APIResponse {
        APIResponse {
            code,
            msg: msg.to_string(),
            data,
        }
    }
    pub fn build() -> APIResponse {
        APIResponse {
            code: 200,
            msg: "".to_string(),
            data: Value::Null,
        }
    }
    /// Convenience method to set `Response` to `code`.
    #[allow(dead_code)]
    pub fn code(mut self, code: u32) -> APIResponse {
        self.code = code;
        self
    }
    /// Convenience method to set `Response` to `msg`.
    #[allow(dead_code)]
    pub fn msg(mut self, msg: &str) -> APIResponse {
        self.msg = msg.to_string();
        self
    }
    /// Set the data of the `Response` to `data`.
    #[allow(dead_code)]
    pub fn data(mut self, data: Value) -> APIResponse {
        self.data = data;
        self
    }
}

impl From<DieselError> for APIResponse {
    fn from(_: DieselError) -> Self {
        internal_server_error()
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for APIResponse {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'o> {
        let body = serde_json::to_string(&self).map_err(|_e| Status::ExpectationFailed)?;
        let mut status = Status::Ok;
        if &self.code > &10000 {
            status = Status::Forbidden;
        }
        Response::build()
            .header(ContentType::JSON)
            .status(status)
            .sized_body(body.len(), Cursor::new(body.to_string()))
            .ok()
    }
}

// 服务异常
pub fn internal_server_error() -> APIResponse {
    APIResponse {
        data: Value::Null,
        code: 500,
        msg: "Internal Server Error".to_string(),
    }
}
