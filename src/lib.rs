// 用于引入 diesel 中的宏 
#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket;
use rocket::{
    serde::json::{json, Value},
    Build, Rocket,
};

mod auth;
mod config;
mod database;
mod errors;
mod schema;
mod models;
mod routes;

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

// 服务
pub fn server() -> Rocket<Build> {
    // 加载配置
    let conf = config::load_config("./app.yaml")
        .unwrap_or_else(|err| panic!("配置初始化失败! err:{:?}", err));

    // rocket 配置
    let figment = config::rocket_config(&conf);
    rocket::custom(figment)
        .mount(
            "/api/v1",
            routes![
                // routes::users::post_users_login,
            ],
        )
        // .attach(database::Db::fairing())
        // .attach(cors_fairing())
        .attach(config::AppState::manage())
        .register("/", catchers![not_found])
}
