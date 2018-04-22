
pub mod api {

    use gotham::http::response::create_response;
    use gotham::state::{State, FromState};
    use gotham::handler::HandlerFuture;
    use hyper::{Body, Chunk, Error, Response, StatusCode};
    use mime;
    use futures::{Future, future, Stream};
    use serde_json;
    use sudoku::{Sudoku, PuzzleError};
    

    #[derive(Deserialize)]
    struct SudokuRequest {
        puzzle: String
    }
    
    #[derive(Serialize)]
    struct SudokuResponse<T> {
        status: String,
        data: T
    }    

    #[derive(Serialize)]
    struct DisplayData {
        display: Vec<String>
    }

    #[derive(Serialize)]
    struct SolutionData {
        solution: String
    }

    fn solve_sudoku(body: Result<Chunk, Error>) -> impl Future<Item=String, Error=PuzzleError> {
        match body {
            Ok(valid_body) => {
                let body_content = String::from_utf8(valid_body.to_vec()).unwrap();
                println!("Body: {}", body_content);
                let sudoku_puzzle: Result<SudokuRequest, _> = serde_json::from_str(&body_content);
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

    pub fn solve(mut state: State) -> Box<HandlerFuture> {
        let fut = 
            Body::take_from(&mut state)
            .concat2()
            .then(solve_sudoku)
            .then(|solve_result| { 
                let sudoku_response = 
                    match solve_result {
                        Ok(solution) => SudokuResponse {status: "success".into(), data: SolutionData {solution: solution}},
                        Err(e) => SudokuResponse {status: "fail".into(), data: SolutionData {solution: format!("{}", e)}},
                    };
                let resp = create_response(
                    &state,
                    StatusCode::Ok,
                    Some((serde_json::to_string(&sudoku_response).unwrap().into_bytes(), mime::APPLICATION_JSON))
                );
                future::ok((state, resp))
            }
        );

        Box::new(fut)
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