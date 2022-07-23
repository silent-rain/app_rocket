/*! Token URI
 *
 */
use chrono::{Local, NaiveDateTime, TimeZone};
use serde::{Deserialize, Deserializer, Serialize};

use crate::config::DATE_FORMAT;
use crate::schema::token_api_auth;

// 用户 Token URI 结构
#[derive(Debug, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "token_api_auth"]
pub struct TokenApiAuth {
    pub id: i32,
    pub user_token_id: i32,
    pub uri: String,
    pub expire: NaiveDateTime,
    pub status: bool,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

// 用户 Token URI 数据 结构
#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "token_api_auth"]
pub struct TokenApiAuthData {
    #[serde(default)]
    pub id: i32,
    #[serde(default)]
    pub user_token_id: i32,
    #[serde(default)]
    pub uri: String,
    #[serde(
        rename = "expire",
        deserialize_with = "str_to_naive_date_time",
        default = "default_expire"
    )]
    pub expire: NaiveDateTime,
    #[serde(default)]
    pub status: bool,
}

// 默认有效期为当前时间
fn default_expire() -> NaiveDateTime {
    Local::now().naive_local()
}

fn str_to_naive_date_time<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let time_str = String::deserialize(deserializer)?;
    let time_obj = Local.datetime_from_str(&time_str, DATE_FORMAT).unwrap();
    Ok(time_obj.naive_local())
}

// 用于查询 URI 权限信息
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct TokenApiAuthQuery {
    pub user_id: String,
    pub token: String,
    pub user_token_id: i32,
    pub uri: String,
    pub expire: NaiveDateTime,
}

// 用于查询 URI 权限信息
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct TokenUri {
    pub token: String,
    pub uri: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_parse_time_str() {
        let t = Local.datetime_from_str("2022-07-23 16:11:06.000", "%Y-%m-%d %H:%M:%S.%3f");
        println!("{:?}", t.unwrap().naive_local());
        assert!(t.is_ok())
    }
}
