use crate::models::{krate, version};
use crate::Application;

use semver::Version;
use warp::reject::{custom, not_found};

pub fn yank(
    crate_id: String,
    version: Version,
    app: Application,
) -> Result<impl warp::Reply, warp::Rejection> {
    do_yank(&crate_id, &version, &app, true)
}

pub fn unyank(
    crate_id: String,
    version: Version,
    app: Application,
) -> Result<impl warp::Reply, warp::Rejection> {
    do_yank(&crate_id, &version, &app, false)
}

fn do_yank(
    crate_id: &str,
    vers: &Version,
    app: &Application,
    yanked: bool,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = app.pool.get().unwrap();

    let krate = krate::Krate::by_name(&conn, crate_id)
        .map_err(custom)?
        .ok_or_else(not_found)?;

    let version = version::Version::by_crate_id_and_version(&conn, krate.id, &vers.to_string())
        .map_err(custom)?
        .ok_or_else(not_found)?;

    crate::yank_crate(&app, crate_id, vers, yanked).map_err(custom)?;

    version.set_yanked(&conn, yanked).map_err(custom)?;

    Ok(warp::reply::json(&super::OK::new()))
}
