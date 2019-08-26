pub mod download;
pub mod me;
pub mod owners;
pub mod publish;
pub mod search;
pub mod yank;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct OK {
    ok: bool,
}

impl OK {
    pub fn new() -> Self {
        OK { ok: true }
    }
}
