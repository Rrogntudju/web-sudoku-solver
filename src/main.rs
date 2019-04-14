//! Gotham web framework router and handlers for sudoku solver 

use gotham::router::Router;
use gotham::router::builder::*;

mod handlers;
use handlers::{display, solve};

mod sudoku;

fn router() -> Router {
    build_simple_router(|route| {
        route.scope("/api", |route| {
            route.post("/solve").to(solve);
            route.post("/display").to(display);
        });
    })
}

/// Start a server and use a `Router` to dispatch requests
fn main() {
    let addr = "localhost:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
}

#[cfg(test)]
mod tests {
    use crate::router;
    use gotham::test::TestServer;
    use hyper::StatusCode;


    #[test]
    fn solve_ok() {
        let test_server = TestServer::new(router()).unwrap();
        let response = test_server
            .client()
            .post("http://localhost/api/solve", 
                r#"{"puzzle": "700000600060001070804020005000470000089000340000039000600050709010300020003000004"}"#, 
                mime::APPLICATION_JSON)
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.read_body().unwrap();
        assert_eq!(body.to_vec(),
            r#"{"status":"success","data":"791543682562981473834726915356478291289615347147239568628154739415397826973862154","message":""}"#
            .to_string().into_bytes()
        );
    }

    #[test]
    fn display_ok() {
        let test_server = TestServer::new(router()).unwrap();
        let response = test_server
            .client()
            .post("http://localhost/api/display", 
                r#"{"puzzle": "309800000000500000250009600480000097700000005930000061008300056000006000000007403"}"#, 
                mime::APPLICATION_JSON
            )
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.read_body().unwrap();
        assert_eq!(body.to_vec(),
            r#"{"status":"success","data":["3 0 9 |8 0 0 |0 0 0 ","0 0 0 |5 0 0 |0 0 0 ","2 5 0 |0 0 9 |6 0 0 ","------+------+------","4 8 0 |0 0 0 |0 9 7 ","7 0 0 |0 0 0 |0 0 5 ","9 3 0 |0 0 0 |0 6 1 ","------+------+------","0 0 8 |3 0 0 |0 5 6 ","0 0 0 |0 0 6 |0 0 0 ","0 0 0 |0 0 7 |4 0 3 "],"message":""}"#
            .to_string().into_bytes()
        );
    }

    #[test]
    fn solve_err_puzzle() {
        let test_server = TestServer::new(router()).unwrap();
        let response = test_server
            .client()
            .post("http://localhost/api/solve", 
                r#"{"puzzle": "X00000600060001070804020005000470000089000340000039000600050709010300020003000004"}"#, 
                mime::APPLICATION_JSON
            )
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.read_body().unwrap();
        assert_eq!(body.to_vec(), 
            r#"{"status":"fail","data":"","message":"Invalid Grid.  Provide a string of 81 digits with 0 or . for empties."}"#
                .to_string().into_bytes()
        );
    }

    #[test]
    fn solve_err_json() {
        let test_server = TestServer::new(router()).unwrap();
        let response = test_server
            .client()
            .post("http://localhost/api/solve", 
                r#"{"xuzzle": "700000600060001070804020005000470000089000340000039000600050709010300020003000004"}"#, 
                mime::APPLICATION_JSON
            )
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.read_body().unwrap();
        assert_eq!(body.to_vec(), 
            r#"{"status":"fail","data":"","message":"Invalid Grid.  Provide a string of 81 digits with 0 or . for empties."}"#
                .to_string().into_bytes()
        );
    }
}
