#[macro_use]
extern crate rocket;
use rocket::{
    serde::json::{json, Value},
    Build, Rocket,
};

mod config;

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
        .mount("/api/v1", routes![])
        .attach(config::AppState::manage())
        .register("/", catchers![not_found])
}
