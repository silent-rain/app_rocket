use diesel::prelude::ExpressionMethods;
use diesel::query_dsl::{QueryDsl, RunQueryDsl};
use diesel::MysqlConnection;
use serde::{Deserialize, Serialize};

use crate::models::user::User;
use crate::schema::user;
use crate::schema::user::dsl::user as all_user;

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "user"]
pub struct NewUser {
    pub name: String,
    pub password: String,
    pub phone: String,
}

// decode request data
#[derive(Deserialize)]
pub struct UserData {
    pub name: String,
}

impl User {
    pub fn get_all_users(conn: &MysqlConnection) -> Vec<User> {
        all_user
            .order(user::id.desc())
            .load::<User>(conn)
            .expect("error!")
    }

    pub fn insert_user(user: NewUser, conn: &MysqlConnection) -> bool {
        diesel::insert_into(user::table)
            .values(&user)
            .execute(conn)
            .is_ok()
    }

    pub fn get_user_by_username(user: UserData, conn: &MysqlConnection) -> Vec<User> {
        all_user
            .filter(user::name.eq(user.name))
            .load::<User>(conn)
            .expect("error!")
    }

    pub fn update_by_username(user: String, phone: String, conn: &MysqlConnection) -> usize {
        let updated_row = diesel::update(all_user.filter(user::name.eq(user)))
            .set(user::phone.eq(phone))
            .execute(conn)
            .unwrap();
        format!("{}", updated_row).parse().unwrap()
    }

    pub fn update_all(user: User, conn: &MysqlConnection) -> Vec<User> {
        diesel::update(all_user.filter(user::id.eq(user.id)))
            .set((
                user::name.eq(user.name),
                user::password.eq(user.password),
                user::phone.eq(user.phone),
            ))
            .execute(conn)
            .expect("update error!");

        all_user
            .filter(user::id.eq(user.id))
            .load::<User>(conn)
            .expect("find error!")
    }

    pub fn delete_by_name(user: String, conn: &MysqlConnection) -> bool {
        diesel::delete(all_user.filter(user::name.eq(user)))
            .execute(conn)
            .is_ok()
    }
}
