
use dotenv::dotenv;
use rocket;

use app_rocket::server;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // 设置环境变量
    dotenv().ok();

    // 启动服务
    let _ = server().launch().await?;
    println!("Rocket: deorbit.");
    Ok(())
}
