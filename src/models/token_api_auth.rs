/*! Token URI
 *
 */
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::schema::token_api_auth;

// 用户 Token URI 结构
#[derive(Debug, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "token_api_auth"]
pub struct TokenApiAuth {
    pub id: i32,
    pub user_token_id: i32,
    pub uri: String,
    pub expire: i32,
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
    #[serde(default)]
    pub expire: i32,
    #[serde(default)]
    pub status: bool,
}

// 用于查询 URI 权限信息
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct TokenApiAuthQuery {
    pub user_id: String,
    pub token: String,
    pub user_token_id: i32,
    pub uri: String,
    pub expire: i32,
}

// 用于查询 URI 权限信息
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct TokenUri {
    pub token: String,
    pub uri: String,
}
