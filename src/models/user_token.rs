/*! 用户与token
 *
 */
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::schema::user_token;

// 用户 Token 结构
#[derive(Debug, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "user_token"]
pub struct UserToken {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub user_id: String,
    pub token: String,
    pub status: bool,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

// 用户 Token 插入 DB 结构
#[derive(Debug, Default, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "user_token"]
pub struct UserTokenInsertOrQuery {
    pub user_id: String,
    #[serde(default)]
    pub token: String,
    #[serde(default)]
    pub status: bool,
}

// 用户信息编译为Token 结构
#[derive(Debug, Serialize)]
pub struct MakeUserToken {}
