use dotenv::dotenv;
use log;
use log4rs;
use rocket;

use app_rocket::server;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // 设置环境变量
    dotenv().ok();

    // 初始化日志配置
    if let Err(err) = log4rs::init_file("./log4rs.yaml", Default::default()) {
        log::warn!("log init config error: {}", err);
    }

    // 启动服务
    let _ = server().launch().await?;
    println!("Rocket: deorbit.");
    Ok(())
}
