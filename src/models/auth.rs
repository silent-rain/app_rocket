/*! token 认证模块
 *
 */
use chrono::{Duration, Utc};
use jsonwebtoken as jwt;
use jwt::{DecodingKey, EncodingKey};
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};
use serde::{Deserialize, Serialize};

use crate::config;

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth {
    /// timestamp
    pub exp: i64,
    /// user id
    pub id: i32,
    pub username: String,
}

impl Auth {
    // 生成 token
    pub fn new(id: i32, username: String) -> Result<String, Box<dyn std::error::Error>> {
        let conf = config::global_config();
        let expire = conf.token.expire;
        let secret = &conf.token.secret;

        let exp = Utc::now() + Duration::seconds(expire);
        let auth = Auth {
            id,
            username,
            exp: exp.timestamp(),
        };
        let token: String = auth.make_token(&secret)?;
        Ok(token)
    }

    // user info 编码 token
    fn make_token(&self, secret: &str) -> Result<String, Box<dyn std::error::Error>> {
        let encoding_key = EncodingKey::from_base64_secret(secret)?;
        let token: String = jwt::encode(&jwt::Header::default(), self, &encoding_key)?;
        Ok(token)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
    type Error = ();

    /// 从 Authorization 头中提取 Auth 令牌。
    /// 从请求中解析 Auth 成功继续调用，失败则返回 530 错误。
    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Auth, Self::Error> {
        if let Some(auth) = extract_auth_from_request(req) {
            Outcome::Success(auth)
        } else {
            Outcome::Failure((Status::Forbidden, ()))
        }
    }
}

/// 从请求的 headers 中获取 Auth 令牌
fn extract_auth_from_request(req: &Request) -> Option<Auth> {
    req.headers()
        .get_one("authorization")
        .and_then(extract_token_from_header)
        .and_then(|token| decode_token(token))
}

// 检查 headers 中是否存在 TOKEN_PREFIX
fn extract_token_from_header<'a>(header: &'a str) -> Option<&'a str> {
    let conf = config::global_config();
    let prefix = &conf.token.prefix;

    if header.starts_with(prefix) {
        Some(&header[prefix.len()..])
    } else {
        None
    }
}

/// 将令牌解码为 Auth 令牌
/// 如果遇到任何错误，返回 None。
fn decode_token(token: &str) -> Option<Auth> {
    let conf = config::global_config();

    use jwt::{Algorithm, Validation};
    let decoding_key = DecodingKey::from_base64_secret(&conf.token.secret);
    jwt::decode(
        token,
        &decoding_key.unwrap(),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|err| {
        eprintln!("Auth decode error: {:?}", err);
    })
    .ok()
    .map(|token_data| token_data.claims)
}
