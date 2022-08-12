use diesel::MysqlConnection;

pub mod http_logs;
pub mod token_api_auth;
pub mod user;
pub mod user_token;

// DB 别名
type DbConnection = MysqlConnection;

/// mysql_pool: diesel_mysql_pool db 连接配置参数
/// # 初始化db
/// ```
/// let db_pool = conf.mysql.database_figment();
/// assert!(config.is_ok());
/// let figment = config::rocket_config(&conf).merge(&db_pool);
/// ```
///
#[database("mysql_pool")]
pub struct DbConn(DbConnection);
