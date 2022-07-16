// 用于引入 diesel 中的宏
#[macro_use]
extern crate diesel;

// 用于引入数据库中的宏
#[macro_use]
extern crate rocket_sync_db_pools;

#[macro_use]
extern crate rocket;

use rocket::{
    serde::json::{json, Value},
    Build, Rocket,
};

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
    if let Err(err) = config::load_config("./app.yaml") {
        panic!("全局配置初始化失败! err: {}", err);
    }
    // 获取全局配置
    let conf = config::global_config();
    println!("conf: {:#?}", conf);

    // 数据库初始化
    // let database_url = conf.mysql.dsn();
    // let pool = database::init_pool(&database_url);
    let pool = conf.mysql.database_figment();

    // rocket 配置
    let figment = config::rocket_config(&conf).merge(&pool);
    rocket::custom(figment)
        .mount(
            "/api/v1",
            routes![
                routes::user::register_user,
                routes::user::login,
                routes::user::get_all,
                routes::user::delete_user,
                routes::user::update_first_name,
                routes::user::updateall,
                routes::user::find_user,
                routes::asset::index,
                routes::asset::serve_embedded_file,
            ],
        )
        .mount(
            "/",
            routes![routes::asset::index, routes::asset::serve_embedded_file],
        )
        .attach(database::DbConn::fairing())
        // .attach(cors_fairing())
        .attach(config::AppState::manage())
        .register("/", catchers![not_found])
}
