use std::sync::Arc;

use crate::models::{
    owner::Owner,
    token::{NewToken, Token},
};
use crate::Application;

use serde::{Deserialize, Serialize};
use warp::reject::custom;

#[derive(Deserialize)]
pub struct TokenForm {
    name: String,
}

#[derive(Serialize)]
pub struct TokenResponse {
    token: String,
    name: String,
}

pub fn add(
    owner: Owner,
    form: TokenForm,
    app: Arc<Application>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = app.pool.get().unwrap();

    let token = crate::utils::generate_token();

    let new_token = NewToken {
        owner_id: owner.id,
        name: &form.name,
        api_token: &token,
        created_at: chrono::Utc::now().naive_utc(),
    };

    let api_token = new_token.save(&conn).map_err(custom)?;

    Ok(warp::reply::with_status(
        warp::reply::json(&TokenResponse {
            token: token.to_owned(),
            name: api_token.name.to_owned(),
        }),
        warp::http::StatusCode::CREATED,
    ))
}

pub fn remove(
    owner: Owner,
    form: TokenForm,
    app: Arc<Application>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = app.pool.get().unwrap();

    Token::delete(&conn, &form.name, owner.id).map_err(custom)?;

    Ok(warp::reply::json(&super::OK::new()))
}
