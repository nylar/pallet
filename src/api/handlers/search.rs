use semver::VersionReq;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SearchOptions {
    q: Option<String>,
    per_page: Option<u8>,
}

#[derive(Debug, Serialize)]
pub struct Results {
    crates: Vec<Crate>,
    meta: Meta,
}

#[derive(Debug, Serialize)]
pub struct Crate {
    name: String,
    max_version: VersionReq,
    description: String,
}

#[derive(Debug, Serialize)]
pub struct Meta {
    total: u64,
}

pub fn search(_search_options: SearchOptions) -> Result<impl warp::Reply, warp::Rejection> {
    // TODO: Populate from DB using search options.
    let results = Results {
        crates: Vec::new(),
        meta: Meta { total: 0 },
    };

    Ok(warp::reply::json(&results))
}
