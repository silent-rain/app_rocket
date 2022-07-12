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
mod models;
mod routes;
mod schema;

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

    // 数据库初始化
    let database_url = conf.mysql.dsn();
    println!("=============={}", database_url);
    let pool = database::init_pool(&database_url);

    // rocket 配置
    let figment = config::rocket_config(&conf);
    rocket::custom(figment)
        .mount(
            "/api/v1",
            routes![
                routes::user::get_all,
                routes::user::delete_user,
                routes::user::update_first_name,
                routes::user::updateall,
                routes::user::new_user,
                routes::user::find_user,
            ],
        )
        .manage(pool)
        // .attach(cors_fairing())
        .attach(config::AppState::manage())
        .register("/", catchers![not_found])
}
