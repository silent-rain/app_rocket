use std::env;

// 用于引入 validator 中的宏
#[macro_use]
extern crate validator_derive;

// 用于引入 diesel 中的宏
#[macro_use]
extern crate diesel;

// 用于引入数据库中的宏
#[macro_use]
extern crate rocket_sync_db_pools;

#[macro_use]
extern crate rocket;
use rocket::{Build, Rocket};
use utoipa_swagger_ui::SwaggerUi;

// 用于引入 utoipa 中的宏
use utoipa::OpenApi;

mod config;
mod database;
mod fairing;
mod models;
mod result;
mod routes;
mod schema;
mod utils;

// 服务
pub fn server() -> Rocket<Build> {
    // 获取当前环境
    let env_name = env::var("ENV_NAME").unwrap_or_else(|_| "prod".to_string());
    // 加载配置
    if let Err(err) = config::load_config(&format!("./app-{}.yaml", env_name)) {
        panic!("全局配置初始化失败! err: {}", err);
    }
    // 获取全局配置
    let conf = config::global_config();

    // mysql 数据库初始化
    let mysql_pool = conf.mysql.database_figment();
    // sqlite 数据库初始化
    let sqlite_pool = conf.sqlite.database_figment();

    // rocket 配置
    let figment = config::rocket_config(&conf)
        .merge(&mysql_pool)
        .merge(&sqlite_pool);
    rocket::custom(figment)
        .attach(fairing::api_token::ApiAuthToken::default()) // API Token 鉴权
        .attach(fairing::log::HttpLogger::new()) // 日志
        .attach(fairing::rsp_auth::resp_auth()) // 鉴权验证
        .attach(database::DbConn::fairing()) // mysql 数据库
        // .attach(cors_fairing())
        .attach(config::AppState::manage()) // App 内部状态
        .mount(
            // http://0.0.0.0:8000/swagger-ui/
            "/",
            SwaggerUi::new("/swagger-ui/<_..>").url(
                "/api-doc/openapi.json",
                models::open_api_doc::ApiDoc::openapi(),
            ),
        )
        .mount(
            "/api/v1",
            routes![
                // 注册、登录
                routes::user::register_user,
                routes::user::login,
                // token 管理
                routes::user_token::get_all_token,
                routes::user_token::get_token_info,
                routes::user_token::add_token,
                routes::user_token::update_token,
                routes::user_token::delete_token,
                // token API 管理
                routes::token_api_auth::get_all_token_uri,
                routes::token_api_auth::get_token_uri_list,
                routes::token_api_auth::get_token_uri_info,
                routes::token_api_auth::add_token_uri,
                routes::token_api_auth::update_token_uri_status,
                routes::token_api_auth::update_token_uri_expire,
                routes::token_api_auth::delete_token_uri,
                // 用户管理
                routes::user::get_user_info,
                routes::user::get_all,
                routes::user::delete_user,
                routes::user::update_first_name,
                routes::user::updateall,
                routes::user::find_user,
                // web 页面
                routes::asset::index,
                // 静态资源
                routes::asset::serve_embedded_file,
            ],
        )
        .mount(
            "/",
            routes![routes::asset::index, routes::asset::serve_embedded_file],
        )
        .register(
            "/",
            catchers![
                routes::errors::bad_request_handler,
                routes::errors::unauthorized_handler,
                routes::errors::forbidden_handler,
                routes::errors::not_found_handler,
                routes::errors::internal_server_error_handler,
                routes::errors::service_unavailable_handler,
                routes::errors::default_catcher,
            ],
        )
}
