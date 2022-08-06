/*!API Token 鉴权
 *
 */

use chrono::{Local, NaiveDateTime};
use log;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::request::Outcome;
use rocket::{Data, Request};

use crate::config;
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
        // 获取 header 判断是否为外部 API 鉴权
        // 否, 则退出
        let api_token_header = request.headers().get_one("X-API-Token-Id");
        let api_token = match api_token_header {
            Some(header) => header.to_string(),
            None => return,
        };

        // 获取请求 URI
        let uri = match request.uri().path().url_decode() {
            Ok(uri) => uri.to_string(),
            Err(e) => {
                log::error!("URI 解析失败, api_token: {}, err: {}", api_token.clone(), e);
                return;
            }
        };

        let (user_id, expire) = match query_user_id_and_expire(request, &api_token, &uri).await {
            Some(v) => v,
            None => return,
        };

        // 动态生成 jwt token
        let conf = config::global_config();
        let secret = conf.auth_token.secret.clone();
        let jwt_token = match Auth::new(user_id, "".to_string())
            .with_exp(expire.timestamp())
            .make_token(&secret)
        {
            Ok(v) => v,
            Err(e) => {
                log::error!(
                    "Token 生成鉴权信息失败, api_token: {}, err: {}",
                    api_token,
                    e
                );
                return;
            }
        };

        // 添加权限至 Header
        request.add_header(Header::new("authorization", format!("Token {}", jwt_token)));
    }
}

// 查询用户 ID 与 expire
async fn query_user_id_and_expire(
    request: &mut Request<'_>,
    api_token: &String,
    uri: &String,
) -> Option<(i32, NaiveDateTime)> {
    // 从 db 查询 API Token 与 URI 所属权限
    let db_conn = match request.guard::<DbConn>().await {
        Outcome::Success(conn) => conn,
        _ => {
            log::error!("获取DB实例失败, api_token: {}", api_token.clone());
            return None;
        }
    };
    let api_token_ = api_token.clone();
    let uri_ = uri.clone();
    let result = db_conn
        .run(move |conn| TokenApiAuth::get_user_id_by_token(api_token_, uri_, conn))
        .await;

    let api_token_auth = match result {
        Ok(v) => v,
        Err(e) => {
            if e.to_string() == "NotFound" {
                log::error!(
                    "该token未查询到此URI的鉴权信息, api_token: {}, uri: {}, err: {}",
                    api_token.clone(),
                    uri.clone(),
                    e
                );
                return None;
            }
            log::error!(
                "获取URI权限信息失败, api_token: {}, uri: {}, err: {}",
                api_token.clone(),
                uri.clone(),
                e
            );
            return None;
        }
    };

    // 判断 授权是否过期
    let time_ = Local::now().naive_local();
    if time_ >= api_token_auth.expire {
        log::error!("Token 授权已过期, api_token: {}", api_token_auth.token);
        return None;
    }

    let user_id = match api_token_auth.user_id.parse::<i32>() {
        Ok(v) => v,
        Err(e) => {
            log::error!(
                "用户信息解析失败, api_token: {}, err: {}",
                api_token_auth.token,
                e
            );
            return None;
        }
    };
    return Some((user_id, api_token_auth.expire));
}
