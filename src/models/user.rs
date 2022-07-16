use serde::{Deserialize, Serialize};

use crate::schema::user;

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
    pub created: String,         // 创建时间
    pub updated: String,         // 更新时间
}

// 注册用户 结构
#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "user"]
pub struct RegisterUser {
    #[serde(skip_deserializing)]
    id: i32,
    pub name: String,
    pub gender: bool,
    pub age: i32,
    pub phone: String,
    pub password: String,
    pub status: bool,
    pub created: String,
    pub updated: String,
}

// 用户登录 结构
#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    pub phone: String,
    pub password: String,
}
