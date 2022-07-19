use diesel::MysqlConnection;

pub mod http_logs;
pub mod user;

#[database("mysql_pool")]
pub struct DbConn(MysqlConnection);
