use crate::database::user::{NewUser, UserData};
use crate::database::Conn as DbConn;
use crate::models::user::User;

use rocket::serde::json::Json;
use serde_json::{json, Value};

// 获取全部用户
#[get("/all")]
pub fn get_all(conn: DbConn) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": User::get_all_users(&conn),
    }))
}

// 添加用户
#[post("/newUser", format = "application/json", data = "<new_user>")]
pub fn new_user(conn: DbConn, new_user: Json<NewUser>) -> Json<Value> {
    Json(json!({
        "status": User::insert_user(new_user.into_inner(), &conn),
        "result": User::get_all_users(&conn).first(),
    }))
}

// 删除用户
#[get("/delete/<user>")]
pub fn delete_user(conn: DbConn, user: String) -> Json<Value> {
    let status = User::delete_by_name(user, &conn);

    Json(json!({
        "status": 200,
        "result": status,
    }))
}

// 根据user更新phone
#[get("/updateName/<user>/<phone>")]
pub fn update_first_name(conn: DbConn, user: String, phone: String) -> Json<Value> {
    let code = User::update_by_username(user, phone, &conn);
    let message;
    if code as i32 == 1 {
        message = String::from("更新成功!")
    } else {
        message = String::from("更新失败!")
    }
    Json(json!({
        "status": 200,
        "code":code,
        "message": message,
    }))
}

// 根据id更新指定字段
#[post("/updateAll", format = "application/json", data = "<update_user>")]
pub fn updateall(conn: DbConn, update_user: Json<User>) -> Json<Value> {
    Json(json!({
        "status": User::update_all(update_user.into_inner(), &conn),
        "result": "ok",
    }))
}

// 根据user获取数据
#[post("/getUser", format = "application/json", data = "<user_data>")]
pub fn find_user(conn: DbConn, user_data: Json<UserData>) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": User::get_user_by_username(user_data.into_inner(), &conn),
    }))
}
