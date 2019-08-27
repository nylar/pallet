use std::path::PathBuf;

use crate::error::Error;

use structopt::StructOpt;

trait Command {
    fn run(&self) -> Result<(), Error>;
}

#[derive(StructOpt)]
pub enum Commands {
    /// Creates a new owner
    #[structopt(name = "create_owner")]
    CreateOwner(CreateOwner),
    #[structopt(name = "create_token")]
    CreateToken(CreateToken),
    /// Serves the HTTP API
    #[structopt(name = "server")]
    Server(Server),
}

impl Commands {
    pub fn run(&self) -> Result<(), Error> {
        match *self {
            Commands::CreateOwner(ref cmd) => cmd.run(),
            Commands::CreateToken(ref cmd) => cmd.run(),
            Commands::Server(ref cmd) => cmd.run(),
        }
    }
}

#[derive(StructOpt)]
pub struct CreateOwner {
    /// A unique identifier (e.g. email) for an owner
    #[structopt(short = "l")]
    pub login: String,
    /// An optional name to associate with the owner
    #[structopt(short = "n")]
    pub name: Option<String>,
    /// URL of database.
    #[structopt(short = "d", env = "DB_URL")]
    pub db_url: String,
}

impl Command for CreateOwner {
    fn run(&self) -> Result<(), Error> {
        use crate::models::owner::NewOwner;

        let pool = crate::make_pool(&self.db_url)?;

        let conn = pool.get()?;

        let new_owner = NewOwner {
            login: &self.login,
            name: self.name.as_ref().map(|x| &**x),
        };

        let owner = new_owner.save(&conn)?;

        println!("Created owner: {}", owner);

        Ok(())
    }
}

#[derive(StructOpt)]
pub struct CreateToken {
    /// The login of a registered owner
    #[structopt(short = "l")]
    pub login: String,
    /// A name to associate with the token
    #[structopt(short = "n")]
    pub name: String,
    /// URL of database.
    #[structopt(short = "d", env = "DB_URL")]
    pub db_url: String,
}

impl Command for CreateToken {
    fn run(&self) -> Result<(), Error> {
        use crate::models::{owner::Owner, token::NewToken};

        let pool = crate::make_pool(&self.db_url)?;

        let conn = pool.get()?;

        let owner = Owner::by_login(&conn, &self.login)?;

        let token = crate::utils::generate_token();

        let new_token = NewToken {
            owner_id: owner.id,
            name: &self.name,
            api_token: &token,
            created_at: chrono::Utc::now().naive_utc(),
        };

        let api_token = new_token.save(&conn)?;

        println!("Created API token: {} ({})", token, api_token.name);

        Ok(())
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
    /// Local storage settings.
    #[structopt(long = "local-base-path", env = "LOCAL_BASE_PATH")]
    pub local_base_path: PathBuf,
    /// Index location.
    #[structopt(long = "index-location", env = "INDEX_LOCATION")]
    pub index_location: String,
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
