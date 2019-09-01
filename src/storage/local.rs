use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::commands::Server;
use crate::error::Error;

#[derive(Clone)]
pub struct Local {
    base_path: PathBuf,
}

impl Local {
    pub fn new(server: &Server) -> Self {
        Local {
            base_path: server.local_opts.local_base_path.to_path_buf(),
        }
    }

    pub fn base_path(&self) -> PathBuf {
        self.base_path.to_path_buf()
    }

    pub fn put(&self, name: &str, version: &str, content: &[u8]) -> Result<(), Error> {
        let crate_path = super::crate_path(name, version);

        let filename = self.base_path.join(Path::new(&crate_path));

        let dir = filename.parent().unwrap();
        fs::create_dir_all(dir)?;

        let mut file = File::create(&filename)?;
        file.write_all(content)?;

        Ok(())
    }

    pub fn get(&self, name: &str, version: &str) -> Result<String, Error> {
        let crate_path = super::crate_path(name, version);

        Ok(crate_path)
    }
}
