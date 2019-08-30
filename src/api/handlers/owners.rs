use std::sync::Arc;

use crate::error::Error;
use crate::models::{
    krate::Krate,
    krateowner::{KrateOwner, NewKrateOwner},
    owner::Owner,
};
use crate::Application;

use serde::{Deserialize, Serialize};
use warp::reject::{custom, not_found};

#[derive(Debug, Serialize)]
pub struct List {
    users: Vec<Owner>,
}

impl List {
    pub fn new(users: Vec<Owner>) -> Self {
        List { users }
    }
}

#[derive(Debug, Deserialize)]
pub struct ModifyOwner {
    users: Vec<String>,
}

pub fn list(
    owner: Owner,
    crate_id: String,
    app: Arc<Application>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = app.pool.get().unwrap();

    let krate = Krate::by_name(&conn, &crate_id)
        .map_err(custom)?
        .ok_or_else(not_found)?;

    super::has_crate_permission(&conn, krate.id, owner.id)?;

    let owners = krate.owners(&conn).map_err(custom)?;

    Ok(warp::reply::json(&List::new(owners)))
}

pub fn add(
    owner: Owner,
    crate_id: String,
    modify_user: ModifyOwner,
    app: Arc<Application>,
) -> Result<impl warp::Reply, warp::Rejection> {
    if modify_user.users.is_empty() {
        return Err(Error::MissingOwners).map_err(custom);
    }

    let conn = app.pool.get().unwrap();

    let krate = Krate::by_name(&conn, &crate_id)
        .map_err(custom)?
        .ok_or_else(not_found)?;

    super::has_crate_permission(&conn, krate.id, owner.id)?;

    let ids = Owner::ids_by_logins(&conn, modify_user.users).map_err(custom)?;

    let new_krate_owners = ids
        .iter()
        .map(|id| NewKrateOwner {
            krate_id: krate.id,
            owner_id: *id,
        })
        .collect::<Vec<_>>();

    NewKrateOwner::save_many(&conn, new_krate_owners).map_err(custom)?;

    Ok(warp::reply::json(&super::OK::new()))
}

pub fn remove(
    owner: Owner,
    crate_id: String,
    modify_user: ModifyOwner,
    app: Arc<Application>,
) -> Result<impl warp::Reply, warp::Rejection> {
    if modify_user.users.is_empty() {
        return Err(Error::MissingOwners).map_err(custom);
    }

    let conn = app.pool.get().unwrap();

    let krate = Krate::by_name(&conn, &crate_id)
        .map_err(custom)?
        .ok_or_else(not_found)?;

    super::has_crate_permission(&conn, krate.id, owner.id)?;

    let ids = Owner::ids_by_logins(&conn, modify_user.users).map_err(custom)?;

    KrateOwner::remove_owners(&conn, krate.id, ids).map_err(custom)?;

    Ok(warp::reply::json(&super::OK::new()))
}
