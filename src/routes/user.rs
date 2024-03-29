/*!
 * 用户信息
 */
use log;
use rocket::serde::json::Json;
use serde_json::json;

use crate::config;
use crate::database::user::UserData;
use crate::database::DbConn;
use crate::models::auth::Auth;
use crate::models::response::APIResponse;
use crate::models::user::{Login, RegisterUser, User};
use crate::models::validation;

// 注册用户
#[utoipa::path(
    request_body = RegisterUser,
    responses(
    (status = 200, body = APIResponse, description = "register user")
   ),
)]
#[post("/user/register", format = "application/json", data = "<user>")]
pub async fn register_user(db: DbConn, user: Json<RegisterUser>) -> APIResponse {
    let result = db
        .run(move |conn| RegisterUser::register_user(user.into_inner(), conn))
        .await;

    if let Err(err) = result {
        log::error!("注册用户失败, err: {}", err);
        return APIResponse::build().code(0).msg("注册用户失败");
    }
    return APIResponse::build().code(200).data(json!(result.unwrap()));
}

// 用户登录
#[post("/user/login", format = "application/json", data = "<user>")]
pub async fn login(db: DbConn, user: Json<Login>) -> Result<APIResponse, APIResponse> {
    let user = user.into_inner();
    // 字段验证
    let mut extractor = validation::FieldValidator::validate(&user);
    let phone = extractor.extract("phone", user.phone);
    let password = extractor.extract("password", user.password);
    extractor.check()?;

    let result = db
        .run(|conn| Login::login(phone, password, conn).map_err(|e| e.to_string()))
        .await;

    if let Err(err) = result {
        if &err.to_string() == "NotFound" {
            log::error!("用户或密码错误, err: {:#?}", err);
            return Ok(APIResponse::build().code(0).msg("用户或密码错误"));
        }
        log::error!("用户登录失败, err: {:#?}", err);
        return Ok(APIResponse::build().code(0).msg("用户登录失败"));
    }
    return Ok(APIResponse::build().code(200).data(json!(result.unwrap())));
}

// 获取用户列表
#[get("/user/info")]
pub async fn get_user_info(auth: Auth, db: DbConn) -> APIResponse {
    let conf = config::global_config();
    // 更新 token, 用于持久登录
    let mut token = "".to_string();
    if conf.auth_token.keep_alive {
        let secret = conf.auth_token.secret.clone();
        token = Auth::new(auth.id, auth.username)
            .make_token(&secret)
            .unwrap_or("".to_string());
    }

    let result = db
        .run(move |conn| User::token_for_user(auth.id, token, conn))
        .await;
    if let Err(err) = result {
        log::error!("获取用户信息失败, err: {}", err);
        return APIResponse::build().code(0).msg("获取用户信息失败");
    }
    return APIResponse::build().code(200).data(json!(result.unwrap()));
}

// 获取用户列表
#[get("/user/all")]
pub async fn get_all(db: DbConn) -> APIResponse {
    let result = db.run(move |conn| User::get_all_users(conn)).await;

    if let Err(err) = result {
        log::error!("获取用户列表失败, err: {}", err);
        return APIResponse::build().code(0).msg("获取用户列表失败");
    }
    return APIResponse::build().code(200).data(json!(result.unwrap()));
}

// 删除用户
#[utoipa::path(
    responses(
        (status = 200, description = "delete user")
    ),
    params(
        ("user", description = "user name"),
    )
)]
#[delete("/delete/<user>")]
pub async fn delete_user(db: DbConn, user: String) -> APIResponse {
    let result = db.run(move |conn| User::delete_by_name(user, conn)).await;

    if let Err(err) = result {
        log::error!("删除用户失败, err: {}", err);
        return APIResponse::build().code(0).msg("删除用户失败");
    }
    return APIResponse::build().code(200).data(json!(result.unwrap()));
}

// 根据user更新phone
#[put("/update_name/<user>/<phone>")]
pub async fn update_first_name(db: DbConn, user: String, phone: String) -> APIResponse {
    let result = db
        .run(move |conn| User::update_by_username(user, phone, conn))
        .await;

    if let Err(err) = result {
        log::error!("更新用户手机号码失败, err: {}", err);
        return APIResponse::build().code(0).msg("更新用户手机号码失败");
    }
    return APIResponse::build().code(200).data(json!(result.unwrap()));
}

// 根据id更新指定字段
#[post(
    "/update_user_info",
    format = "application/json",
    data = "<update_user>"
)]
pub async fn updateall(db: DbConn, update_user: Json<User>) -> APIResponse {
    let result = db
        .run(move |conn| User::update_all(update_user.into_inner(), conn))
        .await;

    if let Err(err) = result {
        log::error!("更新用户信息失败, err: {}", err);
        return APIResponse::build().code(0).msg("更新用户信息失败");
    }
    return APIResponse::build().code(200).data(json!(result.unwrap()));
}

// 根据user获取数据
#[post("/get_user", format = "application/json", data = "<user_data>")]
pub async fn find_user(db: DbConn, user_data: Json<UserData>) -> APIResponse {
    let result = db
        .run(move |conn| User::get_user_by_username(user_data.into_inner(), conn))
        .await;

    if let Err(err) = result {
        log::error!("查询用户信息失败, err: {}", err);
        return APIResponse::build().code(0).msg("查询用户信息失败");
    }
    return APIResponse::build().code(200).data(json!(result.unwrap()));
}

#[cfg(test)]
mod tests {
    // 打印数据类型
    fn print_type_of<T>(_: &T) {
        println!("=============={}", std::any::type_name::<T>())
    }

    #[test]
    fn test_print_type() {
        print_type_of(&"path".to_string());
        assert!(true)
    }
}
