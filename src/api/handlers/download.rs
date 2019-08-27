use std::str::FromStr;
use std::sync::Arc;

use crate::Application;

use semver::Version;
use warp::http::Uri;
use warp::reject::custom;

pub fn download(
    crate_id: String,
    version: Version,
    app: Arc<Application>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let redirect_url: String = app
        .storage
        .get(&crate_id, &version.to_string())
        .map_err(custom)?;

    info!("Redirect URL: {}", redirect_url);

    Ok(warp::redirect::redirect(
        Uri::from_str(&redirect_url).map_err(custom)?,
    ))
}
