/*!请求、响应日志
 *
 */
use chrono::{Local, NaiveDateTime, TimeZone};
use serde::{Deserialize, Deserializer, Serialize};

use crate::config::DATE_FORMAT;
use crate::schema::http_logs;

// 网络请求/响应日志 结构
#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "http_logs"]
pub struct HttpLogger {
    #[serde(default)]
    pub id: i32, // 自增ID
    pub user_id: String,        // 请求用户ID
    pub method: String,         // 请求方法
    pub path: String,           // 请求地址路径
    pub query: String,          // 请求参数
    pub body: String,           // 请求体/响应体
    pub remote_addr: String,    // 请求IP
    pub log_type: &'static str, // 日志类型:req/rsp
    #[serde(
        rename = "created",
        deserialize_with = "str_to_naive_date_time",
        default = "default_created"
    )]
    pub created: NaiveDateTime, // 创建时间
}

fn str_to_naive_date_time<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let time_str = String::deserialize(deserializer)?;
    let time_obj = Local.datetime_from_str(&time_str, DATE_FORMAT).unwrap();
    Ok(time_obj.naive_local())
}

// 默认有效期为当前时间
fn default_created() -> NaiveDateTime {
    Local::now().naive_local()
}

impl HttpLogger {
    pub fn new() -> HttpLogger {
        Default::default()
    }
}

impl Default for HttpLogger {
    fn default() -> HttpLogger {
        HttpLogger {
            id: 0,
            user_id: String::new(),
            method: String::new(),
            path: String::new(),
            query: String::new(),
            body: String::new(),
            remote_addr: String::new(),
            log_type: "",
            created: default_created(),
        }
    }
}
