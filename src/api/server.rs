use std::net::SocketAddr;

use super::{handlers, middleware};
use crate::Application;

use semver::Version;
use warp::{path, Filter};

pub fn server(addr: impl Into<SocketAddr> + 'static, application: Application) {
    let app = warp::any().map(move || application.clone());

    let authorization_header = warp::header("Authorization");

    let me_endpoint = path!("me");

    let api_endpoint = path!("api" / "v1");

    let crates_endpoint = api_endpoint.and(path!("crates"));

    let publish_endpoint = crates_endpoint.and(path!("new")).and(warp::path::end());

    let crate_id = crates_endpoint.and(warp::path::param::<String>());
    let crate_version = crate_id.and(warp::path::param::<Version>());

    let download_endpoint = crate_version.and(path!("download")).and(warp::path::end());

    let yank_endpoint = crate_version.and(path!("yank")).and(warp::path::end());
    let unyank_endpoint = crate_version.and(path!("unyank")).and(warp::path::end());

    let owners_endpoint = crate_id.and(path!("owners")).and(warp::path::end());

    let search_endpoint = crates_endpoint.and(warp::path::end());

    let modify_owners = warp::body::json();

    // Publish `PUT /api/v1/crates/new`
    let crates_new = warp::put2()
        .and(authorization_header)
        .and(warp::body::content_length_limit(10485760))
        .and(warp::body::concat())
        .and(publish_endpoint)
        .and(app.clone())
        .and_then(handlers::publish::publish);

    // Download `GET /api/v1/crates/:crate_id/:version/download`
    let crates_download = warp::get2()
        .and(download_endpoint)
        .and(app.clone())
        .and_then(handlers::download::download);

    // Yank `DELETE /api/v1/crates/:crate_id/:version/yank`
    let crates_yank = warp::delete2()
        .and(yank_endpoint)
        .and(app.clone())
        .and_then(handlers::yank::yank);

    // Unyank `PUT /api/v1/crates/:crate_id/:version/unyank`
    let crates_unyank = warp::put2()
        .and(unyank_endpoint)
        .and(app.clone())
        .and_then(handlers::yank::unyank);

    // Owners List `GET /api/v1/crates/:crate_id/owners`
    let owners_list = warp::get2()
        .and(owners_endpoint)
        .and(app.clone())
        .and_then(handlers::owners::list);

    // Owners Add `PUT /api/v1/crates/{crate_name}/owners`
    let owners_add = warp::put2()
        .and(owners_endpoint)
        .and(modify_owners)
        .and(app.clone())
        .and_then(handlers::owners::add);

    // Owners Remove `DELETE /api/v1/crates/{crate_name}/owners`
    let owners_remove = warp::delete2()
        .and(owners_endpoint)
        .and(modify_owners)
        .and(app.clone())
        .and_then(handlers::owners::remove);

    // Search `GET /api/v1/crates`
    let search = warp::get2()
        .and(search_endpoint)
        .and(warp::query::<handlers::search::SearchOptions>())
        .and_then(handlers::search::search);

    // Me `GET /me`
    let me = warp::get2().and(me_endpoint).map(handlers::me::me);

    let api = crates_new
        .or(crates_download)
        .or(crates_yank)
        .or(crates_unyank)
        .or(owners_list)
        .or(owners_add)
        .or(owners_remove)
        .or(search)
        .or(me)
        .recover(middleware::error_handler);

    warp::serve(api).run(addr);
}
