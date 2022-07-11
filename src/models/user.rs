use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub gender: bool,
    pub age: i32,
    pub birth: String,
    pub phone: String,
    pub email: String,
    pub password: String,
    pub address: String,
    pub avatar: String,
    pub status: bool,
    pub created: String,
    pub updated: String,
}
