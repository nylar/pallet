#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

mod api;
mod commands;
mod config;
mod error;
mod git_auth;
mod metadata;
mod models;
mod repository;
mod schema;
mod storage;
mod types;
mod utils;

pub use commands::{Commands, Server};

use std::fs::File;
use std::sync::{Arc, Mutex, MutexGuard};

use crate::config::Config;
use crate::error::Error;
use crate::metadata::{Dependency, Metadata};
use crate::repository::Repository;
use crate::storage::Storage;
use crate::types::CrateName;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use semver::Version;

#[derive(Clone)]
pub struct Application {
    pub pool: Pool<ConnectionManager<PgConnection>>,
    pub storage: Storage,
    index: Arc<Mutex<Repository>>,
    pub max_upload_size: u64,
    config: Config,
}

impl Application {
    pub fn new(server: &Server) -> Result<Self, Error> {
        let pool = make_pool(&server.db_url)?;

        let storage = Storage::new(&server);

        let checkout_path = match server.checkout_path {
            Some(ref checkout_path) => checkout_path.to_owned(),
            None => tempfile::TempDir::new()?.into_path(),
        };

        let index = Arc::new(Mutex::new(Repository::open(
            &server.index_location,
            &checkout_path,
        )?));

        let config_file = File::open(checkout_path.join("config.json"))?;

        let config = Config::open(config_file, &server.index_location)?;

        Ok(Application {
            pool,
            storage,
            index,
            max_upload_size: server.max_upload_size,
            config,
        })
    }

    pub fn lock_index(&self) -> Result<MutexGuard<'_, Repository>, Error> {
        let repo = self.index.lock().unwrap();
        repo.reset_head()?;
        Ok(repo)
    }

    pub fn dependency_registry_allowed(&self, dependencies: &[Dependency]) -> Result<(), Error> {
        for dependency in dependencies {
            if let Some(ref registry) = dependency.registry {
                if !self.config.registry_allowed(&registry) {
                    return Err(Error::DisallowedRegistry(
                        dependency.name.to_owned(),
                        registry.to_owned(),
                    ));
                }
            }
        }
        Ok(())
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

    let dst = repo.index_file(&*metadata.name);
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

pub fn yank_crate(
    app: &Application,
    name: &CrateName,
    version: &Version,
    yanked: bool,
) -> Result<(), Error> {
    use std::fs;

    let repo = app.lock_index()?;

    let dst = repo.index_file(&name);

    let prev = fs::read_to_string(&dst)?;
    let new = prev
        .lines()
        .map(|line| {
            let mut git_crate = serde_json::from_str::<Metadata>(line)?;
            if &git_crate.name != name || git_crate.vers != *version {
                return Ok(line.to_string());
            }
            git_crate.yanked = yanked;
            Ok(serde_json::to_string(&git_crate)?)
        })
        .collect::<Result<Vec<_>, Error>>();
    let new = new?.join("\n") + "\n";
    fs::write(&dst, new.as_bytes())?;

    repo.commit_and_push(
        &format!(
            "{} crate `{}#{}`",
            if yanked { "Yanking" } else { "Unyanking" },
            name,
            version
        ),
        &repo.relative_index_file(name),
    )?;

    Ok(())
}
