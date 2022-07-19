/*!请求、响应日志
 *
 */
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::schema::req_rsp_logs;

// 网络请求/响应日志 结构
#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "req_rsp_logs"]
pub struct Logger {
    #[serde(default)]
    pub id: i32, // 自增ID
    pub user_id: Option<String>,    // 请求用户ID
    pub method: String,             // 请求方法
    pub path: String,               // 请求地址路径
    pub query: Option<String>,      // 请求参数
    pub body: Option<String>,       // 请求体/响应体
    pub remote_addr: String,        // 请求IP
    pub log_type: String,           // 日志类型:req/rsp
    pub created: NaiveDateTime,     // 创建时间
}
