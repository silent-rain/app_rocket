/*! 用户与token
 *
 */
use log;
use rocket::serde::json::Json;
use serde_json::json;

use crate::database::DbConn;
use crate::models::response::APIResponse;
use crate::models::user_token::{MakeUserToken, UserToken, UserTokenInsertOrQuery};

// 获取所有 Token 列表
#[get("/user_token/all")]
pub async fn get_all_token(db: DbConn) -> APIResponse {
    let result = db.run(move |conn| UserToken::get_all(conn)).await;
    if let Err(err) = result {
        log::error!("获取Token列表信息失败, err: {}", err);
        return APIResponse::build().code(0).msg("获取Token列表信息失败");
    }
    APIResponse::build().code(200).data(json!(result.unwrap()))
}

// 获取Token信息
#[get("/user_token/info/<user_id>")]
pub async fn get_token_info(db: DbConn, user_id: String) -> APIResponse {
    let result = db
        .run(move |conn| UserToken::get_token_by_id(user_id, conn))
        .await;

    if let Err(err) = result {
        log::error!("查询Token信息失败, err: {}", err);
        return APIResponse::build().code(0).msg("查询Token信息失败");
    }
    APIResponse::build().code(200).data(json!(result.unwrap()))
}

// 添加Token
#[post("/user_token/add", data = "<user_token_>")]
pub async fn add_token(db: DbConn, mut user_token_: Json<UserTokenInsertOrQuery>) -> APIResponse {
    let api_token = MakeUserToken::new();
    user_token_.token = api_token;
    user_token_.status = true;
    let result = db
        .run(move |conn| UserToken::insert(&user_token_.into_inner(), conn))
        .await;

    if let Err(err) = result {
        log::error!("Token信息添加失败, err: {}", err);
        return APIResponse::build().code(0).msg("Token信息添加失败");
    }
    APIResponse::build().code(200).msg("Token信息添加成功")
}

// 更新Token状态
#[put("/user_token/update", data = "<user_token_>")]
pub async fn update_token(db: DbConn, user_token_: Json<UserTokenInsertOrQuery>) -> APIResponse {
    let _result = db
        .run(move |conn| UserToken::update(user_token_.into_inner(), conn))
        .await;
    if let Err(err) = _result {
        log::error!("更新Token状态失败, err: {}", err);
        return APIResponse::build().code(0).msg("更新Token状态失败");
    }
    APIResponse::build().code(200).msg("更新成功!")
}

// 删除Token
#[delete("/user_token/delete/<user_id>")]
pub async fn delete_token(db: DbConn, user_id: String) -> APIResponse {
    let _result = db
        .run(move |conn| UserToken::delete_by_id(user_id, conn))
        .await;
    if let Err(err) = _result {
        log::error!("删除Token失败, err: {}", err);
        return APIResponse::build().code(0).msg("删除Token失败");
    }
    APIResponse::build().code(200).msg("删除成功")
}
