use diesel::mysql::MysqlConnection;
use r2d2::{self, PooledConnection};
use r2d2_diesel::ConnectionManager;
use rocket::http::Status;
use rocket::outcome::try_outcome;
use rocket::request::{self, FromRequest, Outcome, Request};
use rocket::State;

use std::ops::Deref;

pub mod user;

pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

// 初始化数据库
pub fn init_pool(db_url: &String) -> Pool {
    let manager = ConnectionManager::<MysqlConnection>::new(db_url);
    r2d2::Pool::new(manager).expect("db pool failure")
}

pub struct DbConn(pub PooledConnection<ConnectionManager<MysqlConnection>>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for DbConn {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<DbConn, ()> {
        let pool = try_outcome!(request.guard::<&State<Pool>>().await);
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = MysqlConnection;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
