
pub mod api {

    use gotham::http::response::create_response;
    use gotham::state::{State, FromState};
    use gotham::handler::{HandlerFuture, HandlerError};
    use hyper::{Body, Chunk, Error, Response, StatusCode};
    use mime;
    use futures::{Future, future, Stream};
    use serde_json;
    use sudoku::{Sudoku, PuzzleError};
    

    #[derive(Serialize, Deserialize)]
    struct SudokuPuzzle<T> {
        puzzle: T
    }
    
    #[derive(Serialize)]
    struct SudokuMessage {
        message: String
    }    

    type SolutionFuture = Future<Item=String, Error=PuzzleError>;

    fn solve_sudoku2<F>(fut: F) -> impl Future<Item=String, Error=PuzzleError> 
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

    fn solve_sudoku(body: Result<Chunk, Error>) -> impl Future<Item=String, Error=PuzzleError> {
        match body {
            Ok(valid_body) => {
                let body_content = String::from_utf8(valid_body.to_vec()).unwrap();
                println!("Body: {}", body_content);
                let sudoku_puzzle: Result<SudokuPuzzle<String>, _> = serde_json::from_str(&body_content);
                match sudoku_puzzle {
                    Ok(sp) => {
                        let solution = Sudoku::new().solve(&sp.puzzle);
                        match solution {
                            Ok(s) => future::ok(s),
                            Err(e) => future::err(e)
                        }
                    }
                    _ => future::err(PuzzleError::InvalidGrid)
                }
            }
            _ => future::err(PuzzleError::InvalidGrid)
        }
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

    pub fn solve(mut state: State) -> impl Future<Item = (State, Response), Error = (State, HandlerError)> {
        
        let fut = 
            Body::take_from(&mut state)
            .concat2()
            .then(solve_sudoku);
/*             .then(|full_body| {
                let future_solution: SolutionFuture = {
                    let x =
                    match full_body {
                        Ok(valid_body) => {
                            let body_content = String::from_utf8(valid_body.to_vec()).unwrap();
                            println!("Body: {}", body_content);
                            let sudoku_puzzle: Result<SudokuPuzzle<String>, _> = serde_json::from_str(&body_content);
                            match sudoku_puzzle {
                                Ok(sp) => {
                                    let solution = Sudoku::new().solve(&sp.puzzle);
                                    match solution {
                                        Ok(s) => future::ok(s),
                                        Err(e) => future::err(e)
                                    }
                                }
                                _ => future::err(PuzzleError::InvalidGrid)
                            }
                        }
                        _ => future::err(PuzzleError::InvalidGrid)
                    };
 //                   Box::new(x)
                };
                future_solution    
            });
        //     .then( */ //|solve_result| { match solve_result {
        //         Ok(solution) => {
        //             let sudoku_solution = SudokuPuzzle {puzzle: solution};
        //             let resp = create_response(
        //                 &state,
        //                 StatusCode::Ok,
        //                 Some((Vec<u8>::From(serde_json::to_string(&sudoku_solution).unwrap()), mime::APPLICATION_JSON))
        //             );
        //             future::ok((state, resp))
        //         }
        //         Err(e) => 
        //     }
        // });

    future::ok()

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