/*!API Token 鉴权
 *
 */

use chrono::Local;
use log;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::request::Outcome;
use rocket::{Data, Request};

use crate::database::DbConn;
use crate::models::auth::Auth;
use crate::models::token_api_auth::TokenApiAuth;

#[derive(Default)]
pub(crate) struct ApiAuthToken {}

#[rocket::async_trait]
impl Fairing for ApiAuthToken {
    // This is a request fairing named "GET/POST ApiAuthToken".
    fn info(&self) -> Info {
        Info {
            name: "GET/POST ApiAuthToken",
            kind: Kind::Request,
        }
    }

    // Increment the counter for `GET` and `POST` requests.
    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        // 判断是否为外部鉴权API
        let api_token = request.headers().get_one("X-API-Token-Id");
        if let None = api_token {
            return;
        }
        let api_token: String = api_token.unwrap().to_string();

        // 请求 URI
        let mut uri: String = String::from("");
        if let Ok(uri_) = request.uri().path().url_decode() {
            uri = uri_.to_string();
        } else {
            log::error!(
                "URI 解析失败, api_token: {}, uri: {}",
                api_token.clone(),
                uri.clone(),
            );
            return;
        }

        // 查询 API Token 与 URI 所属权限
        let outcome = request.guard::<DbConn>().await;
        if let Outcome::Success(conn) = outcome {
            let api_token_ = api_token.clone();
            let uri_ = uri.clone();
            let result = conn
                .run(move |conn| TokenApiAuth::get_user_id_by_token(api_token_, uri_, conn))
                .await;
            if let Err(e) = result {
                if e.to_string() == "NotFound" {
                    log::error!(
                        "该token未查询到此URI的鉴权信息, api_token: {}, uri: {}, err: {}",
                        api_token.clone(),
                        uri.clone(),
                        e
                    );
                    return;
                }
                log::error!(
                    "获取URI权限信息失败, api_token: {}, uri: {}, err: {}",
                    api_token.clone(),
                    uri.clone(),
                    e
                );
                return;
            }
            let api_token_auth = result.unwrap();

            // 判断 授权是否过期
            let time_ = Local::now().naive_local();
            if time_.timestamp_nanos() >= api_token_auth.expire as i64 {
                log::error!("Token 授权已过期, api_token: {}", api_token_auth.token);
                return;
            }

            // 动态生成 token
            let user_id = api_token_auth.user_id.parse::<i32>();
            if let Err(e) = user_id {
                log::error!(
                    "用户信息解析失败, api_token: {}, err: {}",
                    api_token_auth.token,
                    e
                );
                return;
            }
            let token = Auth::new(user_id.unwrap(), "".to_string());
            if let Err(e) = token {
                log::error!(
                    "Token 生成鉴权信息失败, api_token: {}, err: {}",
                    api_token_auth.token,
                    e
                );
                return;
            }

            // 添加权限至 Header
            request.add_header(Header::new(
                "authorization",
                format!("Token {}", token.unwrap()),
            ));
        }
    }
}
