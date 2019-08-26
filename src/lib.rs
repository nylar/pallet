#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

mod api;
mod commands;
mod error;
mod git_auth;
mod metadata;
mod models;
mod repository;
mod schema;
mod storage;
mod utils;

pub use commands::{Commands, Server};

use std::sync::{Arc, Mutex, MutexGuard};

use crate::error::Error;
use crate::metadata::Metadata;
use crate::repository::Repository;
use crate::storage::{Local, Storage};

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

#[derive(Clone)]
pub struct Application {
    pub pool: Pool<ConnectionManager<PgConnection>>,
    pub storage: Storage,
    index: Arc<Mutex<Repository>>,
}

impl Application {
    pub fn new(server: &Server) -> Result<Self, Error> {
        let pool = make_pool(&server.db_url)?;

        let storage = Storage::Local(Local::new(server.local_base_path.as_path()));

        let index = Arc::new(Mutex::new(Repository::open(&server.index_location)?));

        Ok(Application {
            pool,
            storage,
            index,
        })
    }

    pub fn lock_index(&self) -> Result<MutexGuard<'_, Repository>, Error> {
        let repo = self.index.lock().unwrap();
        repo.reset_head()?;
        Ok(repo)
    }
}

pub(crate) fn make_pool(db_url: &str) -> Result<Pool<ConnectionManager<PgConnection>>, Error> {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder().build(manager).map_err(Error::Pool)
}

pub fn add_crate(app: &Application, metadata: &Metadata) -> Result<(), Error> {
    use std::fs::{self, OpenOptions};
    use std::io::Write;

    let repo = app.lock_index()?;

    let dst = repo.index_file(&metadata.name);
    fs::create_dir_all(dst.parent().unwrap())?;

    let mut file = OpenOptions::new().append(true).create(true).open(&dst)?;
    serde_json::to_writer(&mut file, metadata)?;
    file.write_all(b"\n")?;

    repo.commit_and_push(
        &format!("Updating crate `{}#{}`", metadata.name, metadata.vers),
        &repo.relative_index_file(&metadata.name),
    )?;

    Ok(())
}
