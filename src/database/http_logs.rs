/*!网络请求
 *- 日志入库
 */

use diesel::query_dsl::RunQueryDsl;
use diesel::{MysqlConnection, QueryResult};

use crate::models::http_logs::Logger;
use crate::schema::http_logs;

impl Logger {
    // 插入日志
    pub fn insert(logger_: Logger, conn: &MysqlConnection) -> QueryResult<usize> {
        diesel::insert_into(http_logs::table)
            .values(&logger_)
            .execute(conn)
    }
}
