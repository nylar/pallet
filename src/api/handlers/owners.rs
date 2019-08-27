use std::sync::Arc;

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
    let conn = app.pool.get().unwrap();

    let krate = Krate::by_name(&conn, &crate_id)
        .map_err(custom)?
        .ok_or_else(not_found)?;

    super::has_crate_permission(&conn, krate.id, owner.id)?;

    // TODO: Make this one query
    let ids = modify_user
        .users
        .iter()
        .map(|u| {
            let owner = Owner::by_login(&conn, &u).unwrap();
            owner.id
        })
        .collect::<Vec<i32>>();

    // TODO: Bulk insert
    for id in ids {
        let krate_owner = NewKrateOwner {
            krate_id: krate.id,
            owner_id: id,
        };

        krate_owner.save(&conn).map_err(custom)?;
    }

    Ok(warp::reply::json(&super::OK::new()))
}

pub fn remove(
    owner: Owner,
    crate_id: String,
    modify_user: ModifyOwner,
    app: Arc<Application>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = app.pool.get().unwrap();

    let krate = Krate::by_name(&conn, &crate_id)
        .map_err(custom)?
        .ok_or_else(not_found)?;

    super::has_crate_permission(&conn, krate.id, owner.id)?;

    // TODO: Make this one query
    let ids = modify_user
        .users
        .iter()
        .map(|u| {
            let owner = Owner::by_login(&conn, &u).unwrap();
            owner.id
        })
        .collect::<Vec<i32>>();

    // TODO: Bulk delete
    for id in ids {
        KrateOwner::remove_owner(&conn, krate.id, id).map_err(custom)?;
    }
    Ok(warp::reply::json(&super::OK::new()))
}
