use pallet::Commands;
use structopt::StructOpt;

fn main() {
    pretty_env_logger::init();

    let commands = Commands::from_args();

    commands.run().unwrap();
}
