//! Gotham web framework router and handlers for sudoku solver 
#![feature(conservative_impl_trait)]
extern crate futures;
extern crate gotham;
extern crate hyper;
extern crate mime;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use gotham::router::Router;
use gotham::router::builder::*;

mod handlers;
use handlers::*;

mod sudoku;

fn router() -> Router {
    build_simple_router(|route| {
        route.scope("/api", |route| {
            route.post("/solve").to(api::solve);
            route.post("/display").to(api::display);
        });
    })
}

/// Start a server and use a `Router` to dispatch requests
pub fn main() {
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
}

#[cfg(test)]
mod tests {
    use super::*;
    use gotham::test::TestServer;
    use hyper::StatusCode;

    // A small subset of possible tests

    #[test]
    fn index_get() {
        let test_server = TestServer::new(router()).unwrap();
        let response = test_server
            .client()
            .get("http://localhost")
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::Ok);

        let body = response.read_body().unwrap();
        assert_eq!(&body[..], b"index");
    }

    #[test]
    fn checkout_address_patch() {
        let test_server = TestServer::new(router()).unwrap();
        let response = test_server
            .client()
            .patch(
                "http://localhost/checkout/address",
                "data",
                mime::TEXT_PLAIN,
            )
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::Ok);

        let body = response.read_body().unwrap();
        assert_eq!(&body[..], b"update");
    }
}
