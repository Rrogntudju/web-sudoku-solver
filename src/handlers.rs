#[allow(dead_code)]
pub mod api {

    use gotham::http::response::create_response;
    use gotham::state::{State, FromState};
    use gotham::handler::HandlerFuture;
    use hyper::{Body, Chunk, Error, StatusCode};
    use mime;
    use futures::{Future, future, Stream};
    use futures::prelude::*;
    use serde_json;
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
                    StatusCode::Ok,
                    Some((json_response.into_bytes(), mime::APPLICATION_JSON))
                );
                future::ok((state, resp))
            }
        );

        Box::new(fut)
    }

    // Like solve but using async/await
    #[async(boxed)]
    pub fn solve_await(mut state: State) -> Result<(State, hyper::Response), (State, gotham::handler::HandlerError)> {
        let req = await!(Body::take_from(&mut state).concat2().into_future());
        let solve_result = await!(solve_sudoku(req));
        let sudoku_response = 
            match solve_result {
                Ok(solution)    => SolveResponse {status: "success".into(), data: solution, message: "".into()},
                Err(e)          => SolveResponse {status: "fail".into(), data: "".into(), message: format!("{}", e)}
            };
        let json_response = serde_json::to_string(&sudoku_response).unwrap();
        let resp = create_response(
            &state,
            StatusCode::Ok,
            Some((json_response.into_bytes(), mime::APPLICATION_JSON))
        );

        Ok((state, resp))
    }

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
                    StatusCode::Ok,
                    Some((json_response.into_bytes(), mime::APPLICATION_JSON))
                );
                future::ok((state, resp))
            }
        );

        Box::new(fut)
    }

    // Like display but using async/await
    #[async(boxed)]
    pub fn display_await(mut state: State) -> Result<(State, hyper::Response), (State, gotham::handler::HandlerError)> {
        let req = await!(Body::take_from(&mut state).concat2().into_future());
        let grid_result = await!(display_sudoku(req));
        let sudoku_response = 
            match grid_result {
                Ok(grid)    => DisplayResponse {status: "success".into(), data: grid, message: "".into()},
                Err(e)      => DisplayResponse {status: "fail".into(), data: Vec::new(), message: format!("{}", e)}
            };
        let json_response = serde_json::to_string(&sudoku_response).unwrap();
        let resp = create_response(
            &state,
            StatusCode::Ok,
            Some((json_response.into_bytes(), mime::APPLICATION_JSON))
        );

        Ok((state, resp))
    }

}