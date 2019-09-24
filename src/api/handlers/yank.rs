use std::sync::Arc;

use crate::models::{krate, owner::Owner, version};
use crate::types::CrateName;
use crate::Application;

use semver::Version;
use warp::reject::{custom, not_found};

pub fn yank(
    owner: Owner,
    crate_id: CrateName,
    version: Version,
    app: Arc<Application>,
) -> Result<impl warp::Reply, warp::Rejection> {
    do_yank(&owner, &crate_id, &version, &app, true)
}

pub fn unyank(
    owner: Owner,
    crate_id: CrateName,
    version: Version,
    app: Arc<Application>,
) -> Result<impl warp::Reply, warp::Rejection> {
    do_yank(&owner, &crate_id, &version, &app, false)
}

fn do_yank(
    owner: &Owner,
    crate_id: &CrateName,
    vers: &Version,
    app: &Application,
    yanked: bool,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = app.pool.get().unwrap();

    let krate = krate::Krate::by_name(&conn, crate_id)
        .map_err(custom)?
        .ok_or_else(not_found)?;

    super::has_crate_permission(&conn, krate.id, owner.id)?;

    let version = version::Version::by_crate_id_and_version(&conn, krate.id, &vers.to_string())
        .map_err(custom)?
        .ok_or_else(not_found)?;

    crate::yank_crate(&app, crate_id, vers, yanked).map_err(custom)?;

    version.set_yanked(&conn, yanked).map_err(custom)?;

    Ok(warp::reply::json(&super::OK::new()))
}
