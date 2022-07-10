/*!配置文件
*/
use rocket::fairing::AdHoc;
use rocket::figment::Figment;
use rocket::Config;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::error;
use std::fs::read_to_string;

/// Debug only secret for JWT encoding & decoding.
const SECRET: &'static str = "8Xui8SN4mI+7egV/9dlfYYLGQJeEx4+DwmSQLwDVXJg=";

/// 初始化, 解析配置文件
/// # Examples
///
/// ```
/// let config = load_config("./app.yaml");
/// assert!(config.is_ok());
/// ```
pub fn load_config(path: &str) -> Result<AppConfig, Box<dyn error::Error>> {
    let content = read_to_string(&path)?;
    let config: AppConfig = serde_yaml::from_str(&content)?;
    Ok(config)
}

/// 全局配置 结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub env_name: String, // 环境名称: prod/stag/dev
    #[serde(default)]
    pub server: ServerConfig, // 服务配置
    pub mysql: Mysql,     // Mysql 数据库配置
    #[serde(default)]
    pub sqlite: Sqlite, // Sqlite3 数据库配置
    #[serde(default)]
    pub cors: AppCorsConfig, // 跨域配置
}

/// Mysql 数据库配置 结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mysql {
    pub host: String,         // 服务地址
    pub port: u32,            // 端口
    pub user: String,         // 账号
    pub password: String,     // 密码
    pub db_name: String,      // DB 数据库名称
    pub pool_min_idle: u32,   // 最小连接数
    pub pool_max_open: u32,   // 最大连接数
    pub timeout_seconds: u64, // 连接超时时间单位秒
}

/// Sqlite3 数据库配置 结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sqlite {
    pub db_url: String,       // Sqlite3 数据库地址
    pub pool_min_idle: u32,   // 最小连接数
    pub pool_max_open: u32,   // 最大连接数
    pub timeout_seconds: u64, // 连接超时时间单位秒
}

impl Default for Sqlite {
    fn default() -> Sqlite {
        Sqlite {
            db_url: "sqlite://data.sqlite3".to_string(),
            pool_min_idle: 8,
            pool_max_open: 32,
            timeout_seconds: 15,
        }
    }
}

// 跨域配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppCorsConfig {
    pub cors_allow_origin: String,  // Access-Control-Allow-Origin
    pub cors_allow_methods: String, // Access-Control-Allow-Methods
    pub cors_allow_headers: String, // Access-Control-Allow-Headers
}

impl Default for AppCorsConfig {
    fn default() -> AppCorsConfig {
        AppCorsConfig {
            cors_allow_origin: String::from("*"),
            cors_allow_methods: String::from("*"),
            cors_allow_headers: String::from("*"),
        }
    }
}

// 服务配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub address: String,
    pub port: u32,
}

impl Default for ServerConfig {
    fn default() -> ServerConfig {
        ServerConfig {
            address: String::from("0.0.0.0"),
            port: 8000,
        }
    }
}

pub struct AppState {
    pub secret: Vec<u8>,
}

impl AppState {
    pub fn manage() -> AdHoc {
        AdHoc::on_ignite("Manage config", |rocket| async move {
            rocket.manage(AppState {
                secret: SECRET.to_string().into_bytes(),
            })
        })
    }
}

// rocket 配置
pub fn rocket_config(conf: &AppConfig) -> Figment {
    let provider = match &conf.env_name as &str {
        "prod" => Config::release_default(),
        "stag" => Config::debug_default(),
        "dev" => Config::debug_default(),
        _ => panic!("Unknown environment"),
    };
    let figment = Figment::from(provider)
        .merge(("address", &conf.server.address))
        .merge(("port", conf.server.port));
    figment
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let path = "./app.yaml";
        let config = load_config(path);
        assert!(config.is_ok())
    }

    #[test]
    fn test_include_str() {
        let yaml_str = include_str!("../app.yaml");
        assert_ne!(yaml_str, "");
    }
}
