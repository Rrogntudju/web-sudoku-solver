
pub mod api {

    use gotham::http::response::create_response;
    use gotham::state::State;
    use gotham::handler::HandlerFuture;
    use hyper::{Response, StatusCode};
    use mime;
    use futures::{Future, future};
    use serde_json;
    use serde_derive;
    use sudoku::{Sudoku, PuzzleError};
    

    #[derive(Serialize, Deserialize)]
    struct SudokuPuzzle<T> {
        puzzle: T
    }
    
    #[derive(Serialize)]
    struct SudokuMessage {
        message: String
    }    

   fn solve_sudoku<F>(fut: F) -> impl Future<Item=String, Error=PuzzleError> 
        where F: Future<Item=String, Error=PuzzleError> {
            fut.then(move |result| { 
                    let solver_result = result.and_then(|puzzle| {
                        let sdk = Sudoku::new();
                        sdk.solve(&puzzle)
                    });
                    match solver_result {
                        Ok(solution) => future::ok(solution),
                        Err(e) => future::err(e)
                    }
            })
    }
/* 
    fn solve_sudoku<F: 'static>(fut: F) -> Box<Future<Item=String, Error=PuzzleError>> 
        where F: Future<Item=String, Error=PuzzleError> {
            let f =
                fut.then(move |result| { 
                    let solver_result = result.and_then(|puzzle| {
                        let sdk = Sudoku::new();
                        sdk.solve(&puzzle)
                    });
                    match solver_result {
                        Ok(solution) => future::ok(solution),
                        Err(e) => future::err(e)
                    }
                });
            Box::new(f)
    } */

    pub fn solve(mut state: State) -> Box<HandlerFuture> {
        
        let fut = solve_sudoku(future::ok("123456789.".into()));
        let res = create_response(
            &state,
            StatusCode::Ok,
            Some((String::from(stringify!($t)).into_bytes(), mime::TEXT_PLAIN)),
        );

    (state, res)
    }

    pub fn display(state: State) -> (State, Response) {
        let res = create_response(
            &state,
            StatusCode::Ok,
            Some((String::from(stringify!($t)).into_bytes(), mime::TEXT_PLAIN)),
        );

    (state, res)
    }
}