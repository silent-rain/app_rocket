use base64;
use chrono::{Duration, NaiveDateTime, Utc};

use diesel::prelude::ExpressionMethods;
use diesel::query_dsl::{QueryDsl, RunQueryDsl};
use diesel::MysqlConnection;
use diesel::QueryResult;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::auth;
use crate::models::user::{Login, RegisterUser, User};
use crate::schema::user;
use crate::schema::user::dsl::user as user_dsl;

// 注册用户
impl RegisterUser {
    // 注册用户
    pub fn register_user(mut _user: RegisterUser, conn: &MysqlConnection) -> QueryResult<usize> {
        diesel::insert_into(user::table)
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
        user_: Login,
        conn: &MysqlConnection,
    ) -> Result<UserProfile, Box<dyn std::error::Error>> {
        // 获取用户信息
        let user = user::table
            .filter(user::phone.eq(user_.phone))
            .filter(user::password.eq(user_.password))
            .first::<User>(conn)?;

        // 获取 token
        let token: String = auth::Auth::new(user.id, user.name.clone())?;

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
    pub fn token_for_user(
        id: i32,
        token: String,
        conn: &MysqlConnection,
    ) -> QueryResult<UserProfile> {
        let user = user::table.filter(user::id.eq(id)).first::<User>(conn)?;
        Ok(UserProfile {
            id,
            name: user.name,
            phone: user.phone,
            token: token,
        })
    }

    // 获取全部用户
    pub fn get_all_users(conn: &MysqlConnection) -> QueryResult<Vec<User>> {
        user_dsl.order(user::id.desc()).load::<User>(conn)
    }

    // 根据user获取数据
    pub fn get_user_by_username(user: UserData, conn: &MysqlConnection) -> QueryResult<Vec<User>> {
        user_dsl.filter(user::name.eq(user.name)).load::<User>(conn)
    }

    // 根据user更新phone
    pub fn update_by_username(
        user: String,
        phone: String,
        conn: &MysqlConnection,
    ) -> QueryResult<usize> {
        diesel::update(user_dsl.filter(user::name.eq(user)))
            .set(user::phone.eq(phone))
            .execute(conn)
    }

    // 根据id更新指定字段
    pub fn update_all(user: User, conn: &MysqlConnection) -> QueryResult<usize> {
        diesel::update(user_dsl.filter(user::id.eq(user.id)))
            .set((
                user::name.eq(user.name),
                user::password.eq(user.password),
                user::phone.eq(user.phone),
            ))
            .execute(conn)
    }

    // 删除用户
    pub fn delete_by_name(user: String, conn: &MysqlConnection) -> QueryResult<usize> {
        diesel::delete(user_dsl.filter(user::name.eq(user))).execute(conn)
    }

    // 根据 id 删除用户
    pub fn delete_by_id(id: i32, connection: &MysqlConnection) -> bool {
        diesel::delete(user::table.find(id))
            .execute(connection)
            .is_ok()
    }
}

// impl User {
//     /// 验证 candidate_password 是否与存储的密码匹配。
//     pub fn verify_password(&self, candidate_password: &str) -> bool {
//         let candidate_hash = argon2i_simple(candidate_password, SECRET).to_vec();
//         self.password_hash == candidate_hash
//     }

//     /// 生成身份验证令牌并将其保存到 current_auth_token 列。
//     pub fn generate_auth_token(&mut self, conn: &MysqlConnection) -> Result<String, DieselError> {
//         let rng = thread_rng();
//         let new_auth_token = rng.sample_iter(&Alphanumeric).take(32).collect::<String>();
//         self.current_auth_token = Some(new_auth_token.clone());
//         self.last_action = Some(Utc::now().naive_utc());
//         self.save_changes::<User>(conn)?;
//         Ok(new_auth_token)
//     }

//     /// 返回用户是否具有有效的身份验证令牌。
//     pub fn has_valid_auth_token(&self, auth_token_timeout: Duration) -> bool {
//         let latest_valid_date = Utc::now() - auth_token_timeout;
//         if let Some(last_action) = self.last_action {
//             if self.current_auth_token.is_some() {
//                 last_action > latest_valid_date.naive_utc()
//             } else {
//                 false
//             }
//         } else {
//             false
//         }
//     }

//     /// Get a `User` from a login token.
//     ///
//     /// A login token has this format:
//     ///     <user uuid>:<auth token>
//     pub fn get_user_from_login_token(token: &str, db: &MysqlConnection) -> Option<User> {
//         let v: Vec<&str> = token.split(':').collect();
//         let user_id = Uuid::parse_str(v.get(0).unwrap_or(&"")).unwrap_or_default();
//         let auth_token = v.get(1).unwrap_or(&"").to_string();

//         let user = user::table.find(user_id).first::<User>(&*db).optional();
//         if let Ok(Some(u)) = user {
//             if let Some(token) = u.current_auth_token.clone() {
//                 if verify_slices_are_equal(token.as_bytes(), auth_token.as_bytes()).is_ok() {
//                     return Some(u);
//                 }
//             }
//         }
//         None
//     }
// }

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
}
