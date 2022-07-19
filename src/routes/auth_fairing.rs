/*!全局鉴权
 *
 */

use rocket::data::ToByteUnit;
use rocket::fairing::AdHoc;
use rocket::tokio;

const AUTH_WHITE_LIST: [&str; 2] = ["", ""];

// 全局请求 demo
pub fn req_demo() -> AdHoc {
    AdHoc::on_request("Put Rewriter", |req, data| {
        Box::pin(async move {
            // req.set_method(Method::Put);
            // println!("{:#?}", req);
            println!("{:?}", req.headers().get_one("authorization"));
            println!("{:?}", req.uri().path().url_decode());
            println!("{:?}", req.limits().get("json").unwrap());
            println!("{:?}", req.limits().get("form").unwrap());
            // println!("{:?}", data.open(req.limits().get("json").unwrap()));
            // println!("{:?}", data.open(2_i32.mebibytes()));
            println!("{:?}", data.peek_complete());
        })
    })
}

// FromData

use std::io::Cursor;
use std::sync::atomic::{AtomicUsize, Ordering};

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{ContentType, Method, Status};
use rocket::{Data, Request, Response};

#[derive(Default)]
pub(crate) struct Counter {
    get: AtomicUsize,
    post: AtomicUsize,
}

#[rocket::async_trait]
impl Fairing for Counter {
    // This is a request and response fairing named "GET/POST Counter".
    fn info(&self) -> Info {
        Info {
            name: "GET/POST Counter",
            kind: Kind::Request | Kind::Response,
        }
    }

    // Increment the counter for `GET` and `POST` requests.
    async fn on_request(&self, request: &mut Request<'_>, data: &mut Data<'_>) {
        let text = String::from_utf8(data.peek(4096).await.to_vec());
        println!("明文: {:?}", text);
        // println!("==========={:?}", data.peek(4096).await);
        match request.method() {
            Method::Get => self.get.fetch_add(1, Ordering::Relaxed),
            Method::Post => self.post.fetch_add(1, Ordering::Relaxed),
            _ => return,
        };
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        // Don't change a successful user's response, ever.
        if response.status() != Status::NotFound {
            return;
        }

        // Rewrite the response to return the current counts.
        if request.method() == Method::Get && request.uri().path() == "/counts" {
            let get_count = self.get.load(Ordering::Relaxed);
            let post_count = self.post.load(Ordering::Relaxed);
            let body = format!("Get: {}\nPost: {}", get_count, post_count);

            response.set_status(Status::Ok);
            response.set_header(ContentType::Plain);
            response.set_sized_body(body.len(), Cursor::new(body));
        }
    }
}
