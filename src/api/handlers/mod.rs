pub mod download;
pub mod me;
pub mod owners;
pub mod publish;
pub mod search;
pub mod yank;

use crate::error::Error;
use crate::models::krateowner::KrateOwner;

use diesel::pg::PgConnection;
use serde::Serialize;
use warp::reject::custom;

#[derive(Debug, Serialize)]
pub struct OK {
    ok: bool,
}

impl OK {
    pub fn new() -> Self {
        OK { ok: true }
    }
}

pub(crate) fn has_crate_permission(
    conn: &PgConnection,
    krate_id: i32,
    owner_id: i32,
) -> Result<(), warp::Rejection> {
    if KrateOwner::crate_permission(&conn, krate_id, owner_id).map_err(custom)? {
        Ok(())
    } else {
        Err(custom(Error::Unauthorized))
    }
}
