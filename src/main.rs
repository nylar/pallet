use pallet::Commands;
use structopt::StructOpt;

fn main() {
    pretty_env_logger::init();

    let commands = Commands::from_args();

    commands.run().unwrap();

    // let db_url = std::env::var("DATABASE_URL").unwrap();

    // let app = pallet::Application::new(&db_url).unwrap();

    // let commands = Commands::from_args();

    // match commands {
    //     Commands::CreateOwner { login, name } => {
    //         // TODO: Move me to a function
    //         use pallet::models::owner::NewOwner;

    //         let conn = app.pool.get().unwrap();

    //         let new_owner = NewOwner {
    //             login: &login,
    //             name: name.as_ref().map(|x| &**x),
    //         };

    //         let owner = new_owner.save(&conn).unwrap();

    //         println!("Created owner: {}", owner);
    //     }
    //     Commands::CreateToken { login, name } => {
    //         // TODO: Move me to a function
    //         use pallet::models::{owner::Owner, token::NewToken};

    //         let conn = app.pool.get().unwrap();

    //         let owner = Owner::by_login(&conn, &login).unwrap();

    //         let token = pallet::utils::generate_token();

    //         let new_token = NewToken {
    //             owner_id: owner.id,
    //             name: &name,
    //             hashed_token: &pallet::utils::hash_token(&token),
    //             created_at: chrono::Utc::now().naive_utc(),
    //         };

    //         let api_token = new_token.save(&conn).unwrap();

    //         println!("Created API token: {} ({})", token, api_token.name);
    //     }
    //     Commands::Server { port } => {
    //         // TODO: Move me to a function
    //         use std::net::SocketAddr;

    //         pallet::server(
    //             format!("0.0.0.0:{}", port).parse::<SocketAddr>().unwrap(),
    //             app,
    //         );
    //     }
    // }
}
