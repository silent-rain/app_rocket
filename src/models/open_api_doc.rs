/*! Auto generated OpenAPI documentation
 *
 */
use utoipa::Component;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

use crate::models::response::APIResponse;
use crate::models::user::RegisterUser;
use crate::routes;

// 注册 serde_json::Value
// 让 open api 显示为对象
#[derive(Debug, Clone, Component)]
struct Value {}

#[derive(OpenApi)]
#[openapi(
    handlers(
        // 注册、登录
        routes::user::register_user,
        // routes::user::login,
        // // token 管理
        // routes::user_token::get_all_token,
        // routes::user_token::get_token_info,
        // routes::user_token::add_token,
        // routes::user_token::update_token,
        // routes::user_token::delete_token,
        // // token API 管理
        // routes::token_api_auth::get_all_token_uri,
        // routes::token_api_auth::get_token_uri_list,
        routes::token_api_auth::get_token_uri_info,
        // routes::token_api_auth::add_token_uri,
        // routes::token_api_auth::update_token_uri_status,
        // routes::token_api_auth::update_token_uri_expire,
        // routes::token_api_auth::delete_token_uri,
        // // 用户管理
        // routes::user::get_user_info,
        // routes::user::get_all,
        routes::user::delete_user,
        // routes::user::update_first_name,
        // routes::user::updateall,
        // routes::user::find_user,
    ),
    components(
        Value,
        APIResponse,
        RegisterUser,
    ),
    security(
        (),
        ("my_auth" = ["read:items", "edit:items"]),
        ("token_jwt" = [])
    ),
    tags(
        (name = "rocket::api", description = "All about rocket",
            external_docs(url = "https://github.com/juhaku/utoipa", description = "Find out more"))
    ),
    external_docs(url = "https://github.com/juhaku/utoipa", description = "More about our APIs")
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
        components.add_security_scheme(
            "api_key",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("todo_apikey"))),
        )
    }
}
