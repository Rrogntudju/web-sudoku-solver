use gotham::helpers::http::response::create_response;
use gotham::state::{State, FromState};
use gotham::handler::{HandlerFuture};
use hyper::{Body, Chunk, Error, StatusCode};
use futures::{Future, future, Stream};
use serde::{Deserialize, Serialize};

mod sudoku;
use sudoku::{Sudoku, PuzzleError};

#[derive(Deserialize)]
struct SudokuRequest {
    puzzle: String
}

#[derive(Serialize)]
struct SolveResponse {
    status: String,
    data: String,
    message: String
}    

#[derive(Serialize)]
struct DisplayResponse {
    status: String,
    data: Vec<String>,
    message: String
}    

fn solve_sudoku(body: Result<Chunk, Error>) -> impl Future<Item=String, Error=PuzzleError> {
    match body {
        Ok(valid_body) => {
            let body_content = String::from_utf8(valid_body.to_vec()).unwrap();
            let sudoku_puzzle: Result<SudokuRequest, _> = serde_json::from_str(&body_content);
            match sudoku_puzzle {
                Ok(sp) => {
                    let solution = Sudoku::new().solve(&sp.puzzle);
                    match solution {
                        Ok(s)   => future::ok(s),
                        Err(e)  => future::err(e)
                    }
                }
                _ => future::err(PuzzleError::InvalidGrid)
            }
        }
        _ => future::err(PuzzleError::InvalidGrid)
    }
}

fn display_sudoku(body: Result<Chunk, Error>) -> impl Future<Item=Vec<String>, Error=PuzzleError> {
    match body {
        Ok(valid_body) => {
            let body_content = String::from_utf8(valid_body.to_vec()).unwrap();
            let sudoku_puzzle: Result<SudokuRequest, _> = serde_json::from_str(&body_content);
            match sudoku_puzzle {
                Ok(sp) => {
                    let grid = Sudoku::display(&sp.puzzle);
                    match grid {
                        Ok(s)   => future::ok(s),
                        Err(e)  => future::err(e)
                    }
                }
                _ => future::err(PuzzleError::InvalidGrid)
            }
        }
        _ => future::err(PuzzleError::InvalidGrid)
    }
}

#[allow(dead_code)]
pub fn solve(mut state: State) -> Box<HandlerFuture> {
    let fut = 
        Body::take_from(&mut state)
        .concat2()
        .then(solve_sudoku)
        .then(|solve_result| { 
            let sudoku_response = 
                match solve_result {
                    Ok(solution)    => SolveResponse {status: "success".into(), data: solution, message: "".into()},
                    Err(e)          => SolveResponse {status: "fail".into(), data: "".into(), message: format!("{}", e)}
                };
            let json_response = serde_json::to_string(&sudoku_response).unwrap();
            let resp = create_response(
                &state,
                StatusCode::OK,
                mime::APPLICATION_JSON,
                json_response.into_bytes()
            );
            future::ok((state, resp))
        }
    );
    Box::new(fut)
}

// Like solve but using async/await
#[allow(dead_code)]
pub fn solve_await(mut state: State) -> Box<HandlerFuture> {
    let fut03 = async { 
        use futures03::compat::Future01CompatExt;

        let req = Body::take_from(&mut state).concat2().compat().await;
        let solve_result = solve_sudoku(req).compat().await;
        let sudoku_response = 
            match solve_result {
                Ok(solution)    => SolveResponse {status: "success".into(), data: solution, message: "".into()},
                Err(e)          => SolveResponse {status: "fail".into(), data: "".into(), message: format!("{}", e)}
            };
        let json_response = serde_json::to_string(&sudoku_response).unwrap();
        let resp = create_response(
            &state,
            StatusCode::OK,
            mime::APPLICATION_JSON,
            json_response.into_bytes()
        );
        Ok((state, resp))
    };
    use futures03::future::{FutureExt, TryFutureExt};

    Box::new(fut03.boxed().compat())
}

#[allow(dead_code)]
pub fn display(mut state: State) -> Box<HandlerFuture> {
    let fut = 
        Body::take_from(&mut state)
        .concat2()
        .then(display_sudoku)
        .then(|grid_result| { 
            let sudoku_response = 
                match grid_result {
                    Ok(grid)    => DisplayResponse {status: "success".into(), data: grid, message: "".into()},
                    Err(e)      => DisplayResponse {status: "fail".into(), data: Vec::new(), message: format!("{}", e)}
                };
            let json_response = serde_json::to_string(&sudoku_response).unwrap();
            let resp = create_response(
                &state,
                StatusCode::OK,
                mime::APPLICATION_JSON,
                json_response.into_bytes()
            );
            future::ok((state, resp))
        }
    );
    Box::new(fut)
} 

// Like display but using async/await
#[allow(dead_code)]
pub fn display_await(mut state: State) -> Box<HandlerFuture> {
    let fut03 = async { 
        use futures03::compat::Future01CompatExt;

        let req = Body::take_from(&mut state).concat2().compat().await;
        let grid_result = display_sudoku(req).compat().await;
        let sudoku_response = 
            match grid_result {
                Ok(grid)    => DisplayResponse {status: "success".into(), data: grid, message: "".into()},
                Err(e)      => DisplayResponse {status: "fail".into(), data: Vec::new(), message: format!("{}", e)}
            };
        let json_response = serde_json::to_string(&sudoku_response).unwrap();
        let resp = create_response(
            &state,
            StatusCode::OK,
            mime::APPLICATION_JSON,
            json_response.into_bytes()
        );
        Ok((state, resp))
    };
    use futures03::future::{FutureExt, TryFutureExt};

    Box::new(fut03.boxed().compat())
}
