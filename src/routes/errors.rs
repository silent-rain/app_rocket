/*! 错误路由
 *
 */

use std::collections::HashMap;

use once_cell::sync::Lazy;
use rocket::http::Status;
use rocket::Request;
use serde_json::Value::Null;

use crate::routes::APIResponse;

// 业务错误码
const ERROR_CODE: Lazy<HashMap<u32, &'static str>> = Lazy::new(|| {
    let mut code = HashMap::new();
    // Token:
    code.insert(10001, "Invalid Token");
    code.insert(10002, " Expired Signature");
    code
});

#[catch(400)]
pub fn bad_request_handler() -> APIResponse {
    APIResponse::new(Status::BadRequest.code.into(), "Bad Request", Null)
}

#[catch(401)]
pub fn unauthorized_handler() -> APIResponse {
    APIResponse::new(Status::Unauthorized.code.into(), "Unauthorized", Null)
}

#[catch(403)]
pub fn forbidden_handler() -> APIResponse {
    APIResponse::new(Status::Forbidden.code.into(), "Forbidden", Null)
}

#[catch(404)]
pub fn not_found_handler() -> APIResponse {
    APIResponse::new(Status::NotFound.code.into(), "Not Found", Null)
}

#[catch(500)]
pub fn internal_server_error_handler() -> APIResponse {
    APIResponse::new(
        Status::InternalServerError.code.into(),
        "Internal Server Error",
        Null,
    )
}

#[catch(503)]
pub fn service_unavailable_handler() -> APIResponse {
    APIResponse::new(
        Status::ServiceUnavailable.code.into(),
        "Service Unavailable",
        Null,
    )
}

#[catch(default)]
pub fn default_catcher(status: Status, _req: &Request) -> APIResponse {
    match ERROR_CODE.get(&(status.code as u32)) {
        Some(msg) => APIResponse::new(status.code as u32, msg, Null),
        None => APIResponse::new(status.code as u32, "unknown error", Null),
    }
}
