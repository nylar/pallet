use std::path::PathBuf;

use crate::error::Error;

use structopt::StructOpt;

trait Command {
    fn run(&self) -> Result<(), Error>;
}

#[derive(StructOpt)]
pub enum Commands {
    /// Serves the HTTP API
    #[structopt(name = "server")]
    Server(Server),
}

impl Commands {
    pub fn run(&self) -> Result<(), Error> {
        match *self {
            Commands::Server(ref cmd) => cmd.run(),
        }
    }
}

#[derive(StructOpt)]
pub struct Server {
    /// Port to serve the HTTP API on
    #[structopt(long = "port", env = "PORT")]
    pub port: u16,
    /// URL of database.
    #[structopt(long = "db-url", env = "DB_URL")]
    pub db_url: String,
    #[cfg(feature = "local")]
    #[structopt(flatten)]
    pub local_opts: LocalOpts,
    #[cfg(feature = "s3")]
    #[structopt(flatten)]
    pub s3_opts: S3Opts,
    /// Index location, e.g. git@github.com:nylar/private-registry.git
    #[structopt(long = "index-location", env = "INDEX_LOCATION")]
    pub index_location: String,
    /// Checkout path
    #[structopt(long = "checkout-path", env = "CHECKOUT_PATH")]
    pub checkout_path: Option<PathBuf>,
    /// Max upload size in bytes.
    #[structopt(
        long = "max-upload-size",
        env = "MAX_UPLOAD_SIZE",
        default_value = "10485760"
    )]
    pub max_upload_size: u64,
}

impl Command for Server {
    fn run(&self) -> Result<(), Error> {
        use std::net::SocketAddr;
        use std::sync::Arc;

        let app = crate::Application::new(&self)?;

        let addr = format!("0.0.0.0:{}", self.port)
            .parse::<SocketAddr>()
            .unwrap();

        crate::api::server(addr, Arc::new(app));

        Ok(())
    }
}

#[cfg(feature = "local")]
#[derive(StructOpt)]
pub struct LocalOpts {
    /// Path to where the crates are stored
    #[structopt(long = "local-base-path", env = "LOCAL_BASE_PATH")]
    pub local_base_path: PathBuf,
}

#[cfg(feature = "s3")]
#[derive(StructOpt)]
pub struct S3Opts {
    /// S3 region
    #[structopt(long = "s3-region", env = "S3_REGION")]
    pub s3_region: rusoto_core::Region,
    /// S3 bucket
    #[structopt(long = "s3-bucket", env = "S3_BUCKET")]
    pub s3_bucket: String,
    /// S3 access key
    #[structopt(long = "s3-access-key", env = "S3_ACCESS_KEY")]
    pub s3_access_key: String,
    /// S3 secret key
    #[structopt(long = "s3-secret-key", env = "S3_SECRET_KEY")]
    pub s3_secret_key: String,
}
