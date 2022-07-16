/*!配置文件
*/
use once_cell::sync::OnceCell;
use rocket::fairing::AdHoc;
use rocket::figment::Figment;
use rocket::Config;
use serde::{Deserialize, Serialize};
use serde_yaml;

use std::collections::HashMap;
use std::error;
use std::fs::read_to_string;
use std::sync::Arc;

/// js toISOString() in test suit can't handle chrono's default precision
pub const DATE_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.3fZ";

// 全局配置对象
static GLOBAL_CONFIG: OnceCell<Arc<AppConfig>> = OnceCell::new();

/// 初始化, 解析配置文件
/// # Examples
///
/// ```
/// let config = load_config("./app.yaml");
/// assert!(config.is_ok());
/// ```
pub fn load_config(path: &str) -> Result<(), Box<dyn error::Error>> {
    let content = read_to_string(&path)?;
    let config: AppConfig = serde_yaml::from_str(&content)?;
    GLOBAL_CONFIG.get_or_init(|| Arc::new(config));
    Ok(())
}

/// 获取全局配置
/// # Examples
/// ```
/// config = global_config()
/// assert!(config.is_ok());
/// ```
pub fn global_config() -> Arc<AppConfig> {
    let config = GLOBAL_CONFIG.get();
    match config {
        Some(config) => Arc::clone(config),
        None => {
            log::error!("configuration not initialized!");
            panic!("configuration not initialized!")
        }
    }
}

/// 全局配置 结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub env_name: String, // 环境名称: prod/stag/dev
    #[serde(default)]
    pub token: Token, // 令牌配置
    #[serde(default)]
    pub server: ServerConfig, // 服务配置
    pub mysql: Mysql,     // Mysql 数据库配置
    #[serde(default)]
    pub sqlite: Sqlite, // Sqlite3 数据库配置
    #[serde(default)]
    pub cors: AppCorsConfig, // 跨域配置
}

// 令牌配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Token {
    pub expire: i64,    // token 有效期，单位秒
    pub secret: String, // JWT编码和解码的唯一调试秘密。
    pub prefix: String, // 令牌前缀
}

impl Default for Token {
    fn default() -> Token {
        Token {
            expire: 60 * 60 * 24,
            secret: "8Xui8SN4mI+7egV/9dlfYYLGQJeEx4+DwmSQLwDVXJg=".to_string(),
            prefix: "Token ".to_string(),
        }
    }
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

impl Mysql {
    // 获取数据库 url
    pub fn dsn(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.db_name
        )
    }
    // diesel_mysql_pool db 连接配置
    pub fn database_figment(&self) -> (String, HashMap<String, HashMap<String, String>>) {
        let mut database_config = HashMap::new();
        let database_url = self.dsn();
        database_config.insert("url".to_string(), database_url);

        let mut databases = HashMap::new();
        databases.insert("mysql_pool".to_string(), database_config);
        ("databases".to_string(), databases)
    }
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
                secret: "SECRET".to_string().into_bytes(),
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
        .merge(("port", &conf.server.port));
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
