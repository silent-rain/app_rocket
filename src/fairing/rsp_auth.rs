/*! 整罩流 demo
 *
 */
use std::io::Cursor;

use once_cell::sync::Lazy;
use rocket::fairing::AdHoc;
use rocket::http::{ContentType, Status};
use rocket::outcome::Outcome;

use crate::database::DbConn;
use crate::models::auth::extract_auth_from_request;
use crate::models::response::APIResponse;
use crate::models::user::User;

// 路由白名单
const WHITE_LIST: Lazy<Vec<&str>> = Lazy::new(|| vec!["/user/register", "/user/login"]);

/// 请求鉴权并阻止数据返回
/// 由于 rocket fairing 的特性, on_request 无法阻止请求
/// 因此这里采用 response 的方式进行阻止数据返回
pub fn resp_auth() -> AdHoc {
    AdHoc::on_response("Request Auth", |request, response| {
        Box::pin(async move {
            let path = request
                .uri()
                .path()
                .url_decode()
                .map_or(String::new(), |v| v.to_string());

            // 白名单
            for &uri in WHITE_LIST.iter() {
                if path == uri.to_string() {
                    return;
                }
            }

            // 获取鉴权 header
            let authorization = match request.headers().get_one("authorization") {
                Some(v) => v,
                None => return,
            };

            response.set_status(Status::Forbidden);
            response.set_header(ContentType::JSON);

            // 解析鉴权信息
            let auth = match extract_auth_from_request(&request) {
                Ok(v) => v,
                Err(e) => {
                    log::error!(
                        "鉴权信息解析失败, authorization: {}, err: {}",
                        authorization.clone(),
                        e
                    );
                    let body = APIResponse::build().code(0).msg("无效鉴权").to_string();
                    response.set_sized_body(body.len(), Cursor::new(body.clone()));
                    return;
                }
            };

            // DB 验证用户
            let db_conn = match request.guard::<DbConn>().await {
                Outcome::Success(conn) => conn,
                _ => {
                    log::error!("获取DB实例失败, authorization: {}", authorization.clone());
                    let body = APIResponse::build().code(0).msg("内部异常").to_string();
                    response.set_sized_body(body.len(), Cursor::new(body.clone()));
                    return;
                }
            };
            let user_id = auth.id;
            let result = db_conn
                .run(move |conn| User::get_user_by_id(user_id, conn))
                .await;
            if let Err(e) = result {
                log::error!(
                    "无效鉴权, authorization: {}, err: {}",
                    authorization.clone(),
                    e
                );
                let body = APIResponse::build().code(0).msg("无效鉴权").to_string();
                response.set_sized_body(body.len(), Cursor::new(body.clone()));
                return;
            }

            let body = response
                .body_mut()
                .to_string()
                .await
                .map_or(String::new(), |v| v.to_string());
            response.set_sized_body(body.len(), Cursor::new(body.clone()));
        })
    })
}
