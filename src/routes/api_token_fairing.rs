/*!API Token 鉴权
 *
 */

use log;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Data, Request};

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
    async fn on_request(&self, request: &mut Request<'_>, data: &mut Data<'_>) {
        // 判断是否为外部鉴权API
        let api_token = request.headers().get_one("X-API-Token-Id");
        if let None = api_token {
            return;
        }

        // 请求 URI
        let mut uri = String::from("");
        if let Ok(uri_) = request.uri().path().url_decode() {
            uri = uri_.to_string();
        }

        // 查询 API Token 与 URI 所属权限, 并获取 token
        uri.to_string();
        api_token.unwrap();

        // 判断 授权是否过期

        // 动态生成 token

        let token = "";
        // 添加权限至 Header
        request.add_header(Header::new("authorization", format!("Token {}", token)));
    }
}
