/*!
 * 用户信息
 */
use diesel::prelude::ExpressionMethods;
use diesel::query_dsl::{QueryDsl, RunQueryDsl};

use diesel::QueryResult;
use serde::{Deserialize, Serialize};

use crate::config;
use crate::database::DbConnection;
use crate::models::auth;
use crate::models::user::{Login, RegisterUser, User};
use crate::result::ErrorKind;
use crate::schema::users;
use crate::schema::users::dsl::users as users_dsl;
use crate::utils::crypto_aes::encrypt;

// 注册用户
impl RegisterUser {
    // 注册用户
    pub fn register_user(mut _user: RegisterUser, conn: &DbConnection) -> QueryResult<usize> {
        let key = config::PASSWORD_SECRET;
        let password =
            base64::encode(&encrypt(key.as_bytes(), &_user.password.as_bytes()).unwrap());
        _user.password = password;
        diesel::insert_into(users::table)
            .values(&_user)
            .execute(conn)
    }
}

// 反馈用户认证信息 结构
#[derive(Serialize)]
pub struct UserProfile {
    pub id: i32,
    pub name: String,
    pub phone: String,
    pub token: String,
}

impl Login {
    // 登录
    pub fn login(
        phone: String,
        password: String,
        conn: &DbConnection,
    ) -> Result<UserProfile, ErrorKind> {
        // 获取用户信息
        let user = users::table
            .filter(users::phone.eq(phone))
            .filter(users::password.eq(password))
            .first::<User>(conn)
            .map_err(|e| ErrorKind::Other(Box::new(e)))?;

        // 获取 token
        let conf = config::global_config();
        let secret = conf.auth_token.secret.clone();
        let token: String = auth::Auth::new(user.id, user.name.clone()).make_token(&secret)?;

        Ok(UserProfile {
            id: user.id,
            name: user.name,
            phone: user.phone,
            token,
        })
    }
}

// decode request data
#[derive(Deserialize)]
pub struct UserData {
    pub name: String,
}

impl User {
    // 通过 token 获取用户信息
    pub fn token_for_user(id: i32, token: String, conn: &DbConnection) -> QueryResult<UserProfile> {
        let user = users::table.filter(users::id.eq(id)).first::<User>(conn)?;
        Ok(UserProfile {
            id,
            name: user.name,
            phone: user.phone,
            token: token,
        })
    }

    // 通过用户 ID 获取用户信息
    pub fn get_user_by_id(id: i32, conn: &DbConnection) -> QueryResult<User> {
        users::table.filter(users::id.eq(id)).first::<User>(conn)
    }

    // 获取全部用户
    pub fn get_all_users(conn: &DbConnection) -> QueryResult<Vec<User>> {
        users_dsl.order(users::id.desc()).load::<User>(conn)
    }

    // 根据user获取数据
    pub fn get_user_by_username(user_: UserData, conn: &DbConnection) -> QueryResult<Vec<User>> {
        users_dsl
            .filter(users::name.eq(user_.name))
            .load::<User>(conn)
    }

    // 根据user更新phone
    pub fn update_by_username(
        name: String,
        phone: String,
        conn: &DbConnection,
    ) -> QueryResult<usize> {
        diesel::update(users_dsl.filter(users::name.eq(name)))
            .set(users::phone.eq(phone))
            .execute(conn)
    }

    // 根据id更新指定字段
    pub fn update_all(user_: User, conn: &DbConnection) -> QueryResult<usize> {
        diesel::update(users_dsl.filter(users::id.eq(user_.id)))
            .set((
                users::name.eq(user_.name),
                users::password.eq(user_.password),
                users::phone.eq(user_.phone),
            ))
            .execute(conn)
    }

    // 删除用户
    pub fn delete_by_name(name: String, conn: &DbConnection) -> QueryResult<usize> {
        diesel::delete(users_dsl.filter(users::name.eq(name))).execute(conn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_u8_to_string() {
        let s: String = [10u8, 20u8, 30u8]
            .to_vec()
            .iter()
            .map(|x| *x as char)
            .collect();
        println!("{}", s);
        assert_ne!(s, "".to_string())
    }
    #[test]
    fn test_base64() {
        let a = b"hello world";
        let b = "aGVsbG8gd29ybGQ=";
        assert_eq!(base64::encode(a), b);
        assert_eq!(a, &base64::decode(b).unwrap()[..]);
    }

    #[test]
    fn test_login_password() {
        let key = config::PASSWORD_SECRET;
        let password = "888888";
        let password = base64::encode(&encrypt(key.as_bytes(), password.as_bytes()).unwrap());
        println!("encode password: {}", &password);
        assert_eq!(password, "aWHB8nCtflaAgO57uMsaJA==");
    }
}
