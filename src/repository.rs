use std::path::{Path, PathBuf};

use crate::error::Error;
use crate::git_auth::with_authentication;

pub struct Repository {
    checkout_path: PathBuf,
    repository: git2::Repository,
    url: String,
}

impl Repository {
    pub fn open(url: &str, checkout_path: &Path) -> Result<Self, Error> {
        // let checkout_path = TempDir::new()?;

        // info!("Checkout path: {}", checkout_path.path().display());

        let cfg = git2::Config::new()?;

        let repository = with_authentication(url, &cfg, |f| {
            let mut cb = git2::RemoteCallbacks::new();
            cb.credentials(f);
            let mut opts = git2::FetchOptions::new();
            opts.remote_callbacks(cb);
            let mut rb = git2::build::RepoBuilder::new();
            rb.fetch_options(opts);
            rb.clone(url, checkout_path)
        })?;

        Ok(Self {
            checkout_path: checkout_path.to_path_buf(),
            repository,
            url: url.to_owned(),
        })
    }

    pub fn index_file(&self, name: &str) -> PathBuf {
        self.checkout_path.join(self.relative_index_file(name))
    }

    pub fn relative_index_file(&self, name: &str) -> PathBuf {
        let name = name.to_lowercase();
        match name.len() {
            1 => Path::new("1").join(&name),
            2 => Path::new("2").join(&name),
            3 => Path::new("3").join(&name[..1]).join(&name),
            _ => Path::new(&name[0..2]).join(&name[2..4]).join(&name),
        }
    }

    pub fn commit_and_push(&self, msg: &str, modified_file: &Path) -> Result<(), Error> {
        debug!("Adding file");
        // git add $file
        let mut index = self.repository.index()?;
        index.add_path(modified_file)?;
        index.write()?;
        let tree_id = index.write_tree()?;
        let tree = self.repository.find_tree(tree_id)?;

        debug!("Committing");
        // git commit -m "..."
        let head = self.repository.head()?;
        let parent = self.repository.find_commit(head.target().unwrap())?;
        let sig = self.repository.signature()?;
        self.repository
            .commit(Some("HEAD"), &sig, &sig, &msg, &tree, &[&parent])?;

        debug!("Pushing");
        with_authentication(&self.url, &self.repository.config()?, |f| {
            let mut origin = self.repository.find_remote("origin")?;
            let mut cb = git2::RemoteCallbacks::new();
            cb.credentials(f);

            let mut opts = git2::PushOptions::new();
            opts.remote_callbacks(cb);
            origin.push(&["refs/heads/master"], Some(&mut opts))?;

            Ok(())
        })
        .map_err(Error::Git)
    }

    pub fn reset_head(&self) -> Result<(), Error> {
        debug!("Reseting head");
        with_authentication(&self.url, &self.repository.config()?, |f| {
            let mut cb = git2::RemoteCallbacks::new();
            cb.credentials(f);

            let mut origin = self.repository.find_remote("origin")?;

            let mut opts = git2::FetchOptions::new();
            opts.remote_callbacks(cb);

            origin.fetch(&["refs/heads/*:refs/heads/*"], Some(&mut opts), None)?;
            let head = self.repository.head()?.target().unwrap();
            let obj = self.repository.find_object(head, None)?;
            self.repository.reset(&obj, git2::ResetType::Hard, None)?;
            Ok(())
        })
        .map_err(Error::Git)
    }
}
