/*! Token URI
 *
 */

use diesel::prelude::ExpressionMethods;
use diesel::query_dsl::{QueryDsl, RunQueryDsl};
use diesel::MysqlConnection;
use diesel::QueryResult;

use crate::models::token_api_auth::{TokenApiAuth, TokenApiAuthData, TokenApiAuthQuery};
use crate::schema::{token_api_auth, user_token};

// Token API 管理
impl TokenApiAuth {
    // 获取所有 Token URI 列表
    pub fn get_all(conn: &MysqlConnection) -> QueryResult<Vec<TokenApiAuth>> {
        token_api_auth::table
            .order(token_api_auth::id.desc())
            .load::<TokenApiAuth>(conn)
    }

    // 根据 token_id 查询 Token URI 列表
    pub fn get_uri_by_token_id(
        token_id: i32,
        conn: &MysqlConnection,
    ) -> QueryResult<Vec<TokenApiAuth>> {
        token_api_auth::table
            .filter(token_api_auth::user_token_id.eq(token_id))
            .load::<TokenApiAuth>(conn)
    }

    // 根据 token, 查询用户是否拥有权限及返回用户 ID
    pub fn get_user_id_by_token(
        token: String,
        uri: String,
        conn: &MysqlConnection,
    ) -> QueryResult<TokenApiAuthQuery> {
        token_api_auth::table
            .filter(token_api_auth::uri.eq(uri))
            .filter(token_api_auth::status.eq(true))
            .inner_join(user_token::table)
            .filter(user_token::token.eq(token))
            .filter(user_token::status.eq(true))
            .select((
                user_token::user_id,
                user_token::token,
                token_api_auth::user_token_id,
                token_api_auth::uri,
                token_api_auth::expire,
            ))
            .first::<TokenApiAuthQuery>(conn)
    }

    // 添加 Token URI
    pub fn insert(token_: &TokenApiAuthData, conn: &MysqlConnection) -> QueryResult<usize> {
        diesel::insert_into(token_api_auth::table)
            .values(token_)
            .execute(conn)
    }

    // 禁用/启用 Token URI
    pub fn update_status(
        token_uri: &TokenApiAuthData,
        conn: &MysqlConnection,
    ) -> QueryResult<usize> {
        diesel::update(token_api_auth::table.filter(token_api_auth::id.eq(token_uri.id)))
            .set(token_api_auth::status.eq(token_uri.status))
            .execute(conn)
    }

    // 更新 Token URI 有效期
    pub fn update_expire(
        token_uri: &TokenApiAuthData,
        conn: &MysqlConnection,
    ) -> QueryResult<usize> {
        diesel::update(token_api_auth::table.filter(token_api_auth::id.eq(token_uri.id)))
            .set(token_api_auth::expire.eq(token_uri.expire))
            .execute(conn)
    }

    // 删除 Token URI
    pub fn delete_by_id(id: i32, conn: &MysqlConnection) -> QueryResult<usize> {
        diesel::delete(token_api_auth::table.filter(token_api_auth::id.eq(id))).execute(conn)
    }
}

#[cfg(test)]
mod tests {
    use base64;
    use chrono::Local;
    use hex;
    use uuid::Uuid;

    #[test]
    fn test_uuid() {
        let id_ = Uuid::new_v4();
        println!("id: {}", id_);
        assert_ne!(&id_.to_string(), "")
    }

    #[test]
    fn test_hex() {
        let id_ = Uuid::new_v4();
        let id_ = hex::encode(id_);
        println!("id: {}", id_);
        assert_ne!(&id_, "")
    }

    #[test]
    fn test_base64() {
        let id_ = Uuid::new_v4();
        let id_ = base64::encode(id_);
        println!("id: {}", id_);
        assert_ne!(&id_, "")
    }

    #[test]
    fn test_timestamp() {
        let time_ = Local::now().naive_local();
        let t = time_.timestamp_nanos();
        println!("time: {}", t);
        assert_ne!(t, 0)
    }
}
