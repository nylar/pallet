// // use crate::db::DB;
// use std::sync::Arc;

// use crate::db::owner_by_token;
// use crate::models::Owner;

// use diesel::pg::PgConnection;
// use warp::{filters::BoxedFilter, Filter, Rejection, Reply};

// pub fn access_token(
//     token: String,
//     conn: &PgConnection,
// ) -> impl Filter<Extract = (Owner,), Error = Rejection> + Clone {
//     warp::any().and(move || Arc::new(owner_by_token(conn, &token)))
// }

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
