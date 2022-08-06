/*! 用户与token
 *
 */
use diesel::prelude::ExpressionMethods;
use diesel::query_dsl::{QueryDsl, RunQueryDsl};
use diesel::QueryResult;
use hex;
use uuid::Uuid;

use crate::database::DbConnection;
use crate::models::user_token::{MakeUserToken, UserToken, UserTokenInsertOrQuery};
use crate::schema::user_token;

// 用户 Token 管理
impl UserToken {
    // 获取所有 Token
    pub fn get_all(conn: &DbConnection) -> QueryResult<Vec<UserToken>> {
        user_token::table
            .order(user_token::id.desc())
            .load::<UserToken>(conn)
    }

    // 查询 Token
    pub fn get_token_by_id(user_id: String, conn: &DbConnection) -> QueryResult<UserToken> {
        user_token::table
            .filter(user_token::user_id.eq(user_id))
            .first::<UserToken>(conn)
    }

    // 添加 Token
    pub fn insert(user_token_: &UserTokenInsertOrQuery, conn: &DbConnection) -> QueryResult<usize> {
        diesel::insert_into(user_token::table)
            .values(user_token_)
            .execute(conn)
    }

    // 禁用/启用 Token
    pub fn update(user_token_: UserTokenInsertOrQuery, conn: &DbConnection) -> QueryResult<usize> {
        diesel::update(user_token::table.filter(user_token::user_id.eq(user_token_.user_id)))
            .set(user_token::status.eq(user_token_.status))
            .execute(conn)
    }

    // 删除 Token
    pub fn delete_by_id(user_id: String, conn: &DbConnection) -> QueryResult<usize> {
        diesel::delete(user_token::table.filter(user_token::user_id.eq(user_id))).execute(conn)
    }
}

impl MakeUserToken {
    // 生成 Token
    pub fn new() -> String {
        let token = hex::encode(Uuid::new_v4().as_bytes());
        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_token_to_bytes() {
        let token = MakeUserToken::new();
        println!("{:?}", token);
        assert_ne!(token, "".to_string())
    }
}
