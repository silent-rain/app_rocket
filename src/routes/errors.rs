/*! 错误路由
 *
 */

use std::collections::HashMap;

use rocket::http::Status;
use rocket::Request;
use serde_json::Value::Null;

use crate::routes::APIResponse;

#[catch(400)]
pub fn bad_request_handler() -> APIResponse {
    APIResponse::new(400, "Bad Request", Null)
}

#[catch(401)]
pub fn unauthorized_handler() -> APIResponse {
    APIResponse::new(401, "Unauthorized", Null)
}

#[catch(403)]
pub fn forbidden_handler() -> APIResponse {
    APIResponse::new(403, "Forbidden", Null)
}

#[catch(404)]
pub fn not_found_handler() -> APIResponse {
    APIResponse::new(404, "Not Found", Null)
}

#[catch(500)]
pub fn internal_server_error_handler() -> APIResponse {
    APIResponse::new(500, "Internal Server Error", Null)
}

#[catch(503)]
pub fn service_unavailable_handler() -> APIResponse {
    APIResponse::new(503, "Service Unavailable", Null)
}

fn buinss_status_code() -> HashMap<u32, &'static str> {
    let mut code = HashMap::new();
    code.insert(10001, "Token: Invalid Token");
    code.insert(10002, "Token: Expired Signature");
    code
}

#[catch(default)]
pub fn default_catcher(status: Status, _req: &Request) -> APIResponse {
    let status_code = buinss_status_code();
    let v = status_code.get(&(status.code as u32));
    match v {
        Some(msg) => APIResponse::new(status.code as u32, msg, Null),
        None => APIResponse::new(status.code as u32, "unknown error", Null),
    }
}
