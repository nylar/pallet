use std::sync::Arc;

use crate::error::Error;
use crate::models::{owner::Owner, token::Token};
use crate::Application;

use serde::Serialize;
use warp::http::StatusCode;
use warp::{filters::BoxedFilter, reject::custom, Filter, Rejection, Reply};

pub(crate) fn auth(app: Arc<Application>) -> BoxedFilter<(Owner,)> {
    let app = warp::any().map(move || app.clone());

    warp::any()
        .and(app)
        .and(warp::header::<String>("authorization"))
        .and_then(|app: Arc<Application>, token: String| {
            let conn = app.pool.get().unwrap();

            let token = match Token::by_token(&conn, &token) {
                Ok(Some(token)) => token,
                Ok(None) => return Err(custom(Error::Unauthorized)),
                Err(err) => return Err(custom(err)),
            };

            match token.owner(&conn) {
                Ok(Some(owner)) => Ok(owner),
                Ok(None) => Err(custom(Error::Unauthorized)),
                Err(err) => Err(custom(err)),
            }
        })
        .boxed()
}

pub(crate) fn error_handler(err: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(ref err) = err.find_cause::<Error>() {
        match err {
            // Unauthorized errors should return a 403
            Error::Unauthorized => Ok(warp::reply::with_status(
                error(*err),
                StatusCode::UNAUTHORIZED,
            )),
            _ => Ok(warp::reply::with_status(error(*err), StatusCode::OK)),
        }
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
