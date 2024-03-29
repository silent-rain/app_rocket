/*!
 * 用户信息
 */
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::Component;

use crate::schema::users;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,                 // 用户ID
    pub name: String,            // 用户名
    pub gender: bool,            // 性别
    pub age: i32,                // 年龄
    pub birth: Option<String>,   // 出生日期
    pub phone: String,           // 手机号码
    pub email: Option<String>,   // 邮箱
    pub password: String,        // 密码
    pub address: Option<String>, // 居住地址
    pub avatar: Option<String>,  // 头像
    pub status: bool,            // 用户状态
    pub created: NaiveDateTime,  // 创建时间
    pub updated: NaiveDateTime,  // 更新时间
}

// 注册用户 结构
#[derive(Debug, Serialize, Deserialize, Insertable, Component)]
#[table_name = "users"]
pub struct RegisterUser {
    #[serde(skip_deserializing)]
    id: i32,
    pub name: String,
    pub gender: bool,
    pub age: i32,
    pub phone: String,
    pub password: String,
    pub status: bool,
}

// 用户登录 结构
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Login {
    #[validate(length(min = 11))]
    pub phone: Option<String>,
    #[validate(length(min = 6))]
    pub password: Option<String>,
}
