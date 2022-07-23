/*!网络请求/响应日志
 *- 日志打印
 *- 日志入库
 */
use std::io::Cursor;

use chrono::Local;
use log;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::request::Outcome;
use rocket::{Data, Request, Response};

use crate::database::DbConn;
use crate::models::auth::extract_auth_from_request;
use crate::models::http_logs::Logger;

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
        log::trace!("{:?}", log);
        // 数据入库
        let outcome = request.guard::<DbConn>().await;
        if let Outcome::Success(conn) = outcome {
            let _result = conn.run(move |conn| Logger::insert(log, conn)).await;
        }
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

        let mut body = String::from("");
        if let Ok(body_) = response.body_mut().to_string().await {
            body = body_.to_string();
        }
        response.set_sized_body(body.len(), Cursor::new(body.clone()));

        let log = Logger {
            id: 0,
            user_id,
            method: request.method().to_string(),
            path,
            query,
            body: Some(body),
            remote_addr,
            log_type: "rsp".to_string(),
            created,
        };
        log::trace!("{:?}", log);
        // 数据入库
        let outcome = request.guard::<DbConn>().await;
        if let Outcome::Success(conn) = outcome {
            let _result = conn.run(move |conn| Logger::insert(log, conn)).await;
        }
    }
}
