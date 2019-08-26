use crate::error::Error;

use serde::Serialize;
use warp::http::StatusCode;
use warp::{Rejection, Reply};

pub fn error_handler(err: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(ref err) = err.find_cause::<Error>() {
        Ok(warp::reply::with_status(error(*err), StatusCode::OK))
    } else {
        Err(err)
    }
}

fn error<E: std::error::Error>(err: &E) -> impl warp::Reply {
    warp::reply::json(&Errors {
        errors: vec![ErrorDetail {
            detail: err.to_string(),
        }],
    })
}

#[derive(Debug, Serialize)]
struct Errors {
    errors: Vec<ErrorDetail>,
}

#[derive(Debug, Serialize)]
struct ErrorDetail {
    detail: String,
}
