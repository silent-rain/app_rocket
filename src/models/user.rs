use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub gender: bool,
    pub age: i32,
    pub birth: Option<String>,
    pub phone: String,
    pub email: Option<String>,
    pub password: String,
    pub address: Option<String>,
    pub avatar: Option<String>,
    pub status: bool,
    pub created: String,
    pub updated: String,
}
