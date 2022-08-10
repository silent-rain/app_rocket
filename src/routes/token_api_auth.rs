/*! Token URI
 *
 */
use log;
use rocket::serde::json::Json;
use serde_json::json;

use crate::database::DbConn;
use crate::models::response::APIResponse;
use crate::models::token_api_auth::{TokenApiAuth, TokenApiAuthData, TokenUri};

// 获取所有 Token URI 列表
#[get("/token_uri/all")]
pub async fn get_all_token_uri(db: DbConn) -> APIResponse {
    let result = db.run(move |conn| TokenApiAuth::get_all(conn)).await;
    if let Err(err) = result {
        log::error!("获取Token URI列表信息失败, err: {}", err);
        return APIResponse::build()
            .code(0)
            .msg("获取Token URI列表信息失败");
    }
    APIResponse::build().code(200).data(json!(result.unwrap()))
}

// 根据 token_id 查询 Token URI 列表
#[get("/token_uri/uri_list/<token_id>")]
pub async fn get_token_uri_list(db: DbConn, token_id: i32) -> APIResponse {
    let result = db
        .run(move |conn| TokenApiAuth::get_uri_by_token_id(token_id, conn))
        .await;

    if let Err(err) = result {
        log::error!("查询Token对应的URI列表失败, err: {}", err);
        return APIResponse::build()
            .code(0)
            .msg("查询Token对应的URI列表失败");
    }
    APIResponse::build().code(200).data(json!(result.unwrap()))
}

// 根据 token、uri, 查询token是否拥有权限及返回用户 ID
#[get("/token_uri/info", data = "<token_uri>")]
pub async fn get_token_uri_info(db: DbConn, token_uri: Json<TokenUri>) -> APIResponse {
    let result = db
        .run(move |conn| {
            TokenApiAuth::get_user_id_by_token(token_uri.token.clone(), token_uri.uri.clone(), conn)
        })
        .await;

    if let Err(err) = result {
        log::error!("查询Token与URI对应的权限信息失败, err: {}", err);
        return APIResponse::build()
            .code(0)
            .msg("查询Token与URI对应的权限信息失败");
    }
    APIResponse::build().code(200).data(json!(result.unwrap()))
}

// 添加 Token URI
#[post("/token_uri/add", data = "<token_uri>")]
pub async fn add_token_uri(db: DbConn, token_uri: Json<TokenApiAuthData>) -> APIResponse {
    let result = db
        .run(move |conn| TokenApiAuth::insert(&token_uri.into_inner(), conn))
        .await;

    if let Err(err) = result {
        log::error!("Token URI信息添加失败, err: {}", err);
        return APIResponse::build().code(0).msg("Token信息添加失败");
    }
    APIResponse::build().code(200).msg("Token URI信息添加成功")
}

// 更新Token URI状态
#[put("/token_uri/update_status", data = "<token_uri>")]
pub async fn update_token_uri_status(db: DbConn, token_uri: Json<TokenApiAuthData>) -> APIResponse {
    let _result = db
        .run(move |conn| TokenApiAuth::update_status(&token_uri.into_inner(), conn))
        .await;
    if let Err(err) = _result {
        log::error!("更新Token URI状态失败, err: {}", err);
        return APIResponse::build().code(0).msg("更新Token URI状态失败");
    }
    APIResponse::build().code(200).msg("更新成功!")
}

// 更新Token URI 有效期
#[put("/token_uri/update_expire", data = "<token_uri>")]
pub async fn update_token_uri_expire(db: DbConn, token_uri: Json<TokenApiAuthData>) -> APIResponse {
    let _result = db
        .run(move |conn| TokenApiAuth::update_expire(&token_uri.into_inner(), conn))
        .await;
    if let Err(err) = _result {
        log::error!("更新Token URI状态失败, err: {}", err);
        return APIResponse::build().code(0).msg("更新Token URI状态失败");
    }
    APIResponse::build().code(200).msg("更新成功!")
}

// 删除Token URI
#[delete("/token_uri/delete/<id>")]
pub async fn delete_token_uri(db: DbConn, id: i32) -> APIResponse {
    let _result = db
        .run(move |conn| TokenApiAuth::delete_by_id(id, conn))
        .await;
    if let Err(err) = _result {
        log::error!("删除Token URI失败, err: {}", err);
        return APIResponse::build().code(0).msg("删除Token URI失败");
    }
    APIResponse::build().code(200).msg("删除成功")
}
