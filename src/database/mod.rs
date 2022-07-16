use diesel::MysqlConnection;

pub mod user;

#[database("mysql_pool")]
pub struct DbConn(MysqlConnection);
