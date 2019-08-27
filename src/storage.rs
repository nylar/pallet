use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::error::Error;

// TODO: S3
#[derive(Clone)]
pub enum Storage {
    Local(Local),
}

impl Storage {
    fn crate_path(name: &str, version: &str) -> String {
        format!("crates/{}/{}-{}.crate", name, name, version)
    }

    pub fn put(&self, name: &str, version: &str, content: &[u8]) -> Result<(), Error> {
        let crate_path = Storage::crate_path(name, version);

        match *self {
            Storage::Local(ref local) => local.put(&crate_path, content),
        }
    }

    pub fn get(&self, name: &str, version: &str) -> Result<String, Error> {
        let crate_path = Storage::crate_path(name, version);

        match *self {
            Storage::Local(ref local) => local.get(&crate_path),
        }
    }

    pub fn local_base_path(&self) -> Option<PathBuf> {
        match *self {
            Storage::Local(ref local) => Some(local.base_path.clone()),
        }
    }
}

#[derive(Clone)]
pub struct Local {
    base_path: PathBuf,
}

impl Local {
    pub fn new(base_path: &Path) -> Self {
        Local {
            base_path: base_path.to_path_buf(),
        }
    }

    fn put(&self, crate_path: &str, content: &[u8]) -> Result<(), Error> {
        let filename = self.base_path.join(Path::new(crate_path));

        let dir = filename.parent().unwrap();
        fs::create_dir_all(dir)?;

        let mut file = File::create(&filename)?;
        file.write_all(content)?;

        Ok(())
    }

    fn get(&self, crate_path: &str) -> Result<String, Error> {
        Ok(format!("/crates/{}", crate_path))
    }
}
