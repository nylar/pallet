use crate::models::{krate, version};
use crate::Application;

use diesel::pg::PgConnection;
use semver::Version;
use warp::reject::{custom, not_found};

pub fn yank(
    crate_id: String,
    version: Version,
    app: Application,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = app.pool.get().unwrap();

    do_yank(&crate_id, &version, &conn, true)
}

pub fn unyank(
    crate_id: String,
    version: Version,
    app: Application,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = app.pool.get().unwrap();

    do_yank(&crate_id, &version, &conn, false)
}

fn do_yank(
    crate_id: &str,
    version: &Version,
    conn: &PgConnection,
    yanked: bool,
) -> Result<impl warp::Reply, warp::Rejection> {
    let krate = krate::Krate::by_name(&conn, crate_id)
        .map_err(custom)?
        .ok_or_else(|| not_found())?;

    let version = version::Version::by_crate_id_and_version(&conn, krate.id, &version.to_string())
        .map_err(custom)?
        .ok_or_else(|| not_found())?;

    version.set_yanked(&conn, yanked).map_err(custom)?;

    Ok(warp::reply::json(&super::OK::new()))
}
