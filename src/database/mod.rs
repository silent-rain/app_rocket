use diesel::MysqlConnection;

pub mod http_logs;
pub mod token_api_auth;
pub mod user;
pub mod user_token;

#[database("mysql_pool")]
pub struct DbConn(MysqlConnection);
