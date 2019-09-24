use std::collections::HashSet;
use std::sync::Arc;

use crate::error::Error;
use crate::models::{
    krate::Krate,
    krateowner::{KrateOwner, NewKrateOwner},
    owner::{NewOwner, Owner},
};
use crate::types::CrateName;
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
    crate_id: CrateName,
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
    crate_id: CrateName,
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

    let ids = Owner::ids_by_logins(&conn, &modify_user.users).map_err(custom)?;

    let new_krate_owners = ids
        .iter()
        .map(|id| NewKrateOwner {
            krate_id: krate.id,
            owner_id: *id,
        })
        .collect::<Vec<_>>();

    NewKrateOwner::save_many(&conn, new_krate_owners).map_err(custom)?;

    let msg = format!("Added [{}] as owners", &modify_user.users.join(", "));

    Ok(warp::reply::json(&super::OkMessage::new(msg)))
}

pub fn remove(
    owner: Owner,
    crate_id: CrateName,
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

    let all_owners = krate
        .owners(&conn)
        .map_err(custom)?
        .iter()
        .map(|o| o.id)
        .collect::<HashSet<_>>();

    let ids = Owner::ids_by_logins(&conn, &modify_user.users).map_err(custom)?;

    // Don't allow all owners to be removed from a krate.
    if all_owners == ids.iter().map(|o| *o).collect::<HashSet<_>>() {
        return Err(Error::UnableToOrphanCrate).map_err(custom);
    }

    KrateOwner::remove_owners(&conn, krate.id, ids).map_err(custom)?;

    let msg = format!("Removed [{}] as owners", &modify_user.users.join(", "));

    Ok(warp::reply::json(&super::OkMessage::new(msg)))
}

#[derive(Deserialize)]
pub struct OwnerForm {
    login: String,
    name: Option<String>,
}

pub fn new(form: OwnerForm, app: Arc<Application>) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = app.pool.get().unwrap();

    let new_owner = NewOwner {
        login: &form.login,
        name: form.name.as_ref().map(|x| &**x),
    };

    new_owner.save(&conn).map_err(custom)?;

    Ok(warp::reply::with_status(
        warp::reply::json(&super::OK::new()),
        warp::http::StatusCode::CREATED,
    ))
}
