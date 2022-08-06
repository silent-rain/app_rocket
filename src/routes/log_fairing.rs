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
pub use crate::models::http_logs::HttpLogger;

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
        let path = request
            .uri()
            .path()
            .url_decode()
            .map_or(String::new(), |v| v.to_string());

        let query = request
            .uri()
            .query()
            .map_or(String::new(), |v| v.to_string());

        let body = String::from_utf8(data.peek(4096).await.to_vec()).map_or(String::new(), |v| v);

        let remote_addr = request.remote().map_or(String::new(), |v| v.to_string());

        let user_id =
            extract_auth_from_request(&request).map_or(String::new(), |auth| auth.id.to_string());

        let created = Local::now().naive_local();

        let log = HttpLogger {
            id: 0,
            user_id,
            method: request.method().to_string(),
            path,
            query,
            body,
            remote_addr,
            log_type: "req",
            created,
        };
        log::trace!("{:?}", log);
        // 数据入库
        if let Outcome::Success(conn) = request.guard::<DbConn>().await {
            let _ = conn.run(move |conn| HttpLogger::insert(log, conn)).await;
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        let path = request
            .uri()
            .path()
            .url_decode()
            .map_or(String::new(), |v| v.to_string());

        let query = request
            .uri()
            .query()
            .map_or(String::new(), |v| v.to_string());

        let remote_addr = request.remote().map_or(String::new(), |v| v.to_string());

        let created = Local::now().naive_local();

        let user_id =
            extract_auth_from_request(&request).map_or(String::new(), |auth| auth.id.to_string());

        let body = response
            .body_mut()
            .to_string()
            .await
            .map_or(String::new(), |v| v.to_string());
        response.set_sized_body(body.len(), Cursor::new(body.clone()));

        let log = HttpLogger {
            id: 0,
            user_id,
            method: request.method().to_string(),
            path,
            query,
            body,
            remote_addr,
            log_type: "rsp",
            created,
        };
        log::trace!("{:?}", log);
        // 数据入库
        if let Outcome::Success(conn) = request.guard::<DbConn>().await {
            let _ = conn.run(move |conn| HttpLogger::insert(log, conn)).await;
        }
    }
}
