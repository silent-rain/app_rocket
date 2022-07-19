/*!网络请求/响应日志
 *- 日志入库
 */
use chrono::Local;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Data, Request, Response};

use crate::models::auth::extract_auth_from_request;
use crate::models::logs::Logger;

// 网络请求/响应日志
#[derive(Default)]
pub(crate) struct HttpLogger {}

#[rocket::async_trait]
impl Fairing for HttpLogger {
    // This is a request and response fairing named "GET/POST Counter".
    fn info(&self) -> Info {
        Info {
            name: "GET/POST Logger",
            kind: Kind::Request | Kind::Response,
        }
    }

    // Increment the counter for `GET` and `POST` requests.
    async fn on_request(&self, request: &mut Request<'_>, data: &mut Data<'_>) {
        let mut path = String::from("");
        if let Ok(path_) = request.uri().path().url_decode() {
            path = path_.to_string();
        }
        let mut query = Some(String::from(""));
        if let Some(query_) = request.uri().query() {
            query = Some(query_.to_string());
        }

        let mut body = Some(String::from(""));
        if let Ok(body_) = String::from_utf8(data.peek(4096).await.to_vec()) {
            body = Some(body_);
        }

        let mut remote_addr = String::from("");
        if let Some(remote_addr_) = request.remote() {
            remote_addr = remote_addr_.to_string();
        }

        let created = Local::now().naive_local();

        let mut user_id = Some(String::from(""));
        if let Ok(auth) = extract_auth_from_request(&request) {
            user_id = Some(auth.id.to_string());
        }

        let log = Logger {
            id: 0,
            user_id,
            method: request.method().to_string(),
            path,
            query,
            body,
            remote_addr,
            log_type: "req".to_string(),
            created,
        };
        println!("req ==========={:?}", log);

        // 数据入库
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        let mut path = String::from("");
        if let Ok(path_) = request.uri().path().url_decode() {
            path = path_.to_string();
        }
        let mut query = Some(String::from(""));
        if let Some(query_) = request.uri().query() {
            query = Some(query_.to_string());
        }

        let mut remote_addr = String::from("");
        if let Some(remote_addr_) = request.remote() {
            remote_addr = remote_addr_.to_string();
        }

        let created = Local::now().naive_local();

        let mut user_id = Some(String::from(""));
        if let Ok(auth) = extract_auth_from_request(&request) {
            user_id = Some(auth.id.to_string());
        }

        let mut body = Some(String::from(""));
        if let Ok(body_) = response.body_mut().to_string().await {
            body = Some(body_.to_string());
        }

        let log = Logger {
            id: 0,
            user_id,
            method: request.method().to_string(),
            path,
            query,
            body,
            remote_addr,
            log_type: "rsp".to_string(),
            created,
        };
        println!("rsp ==========={:?}", log);

        // 数据入库
    }
}
