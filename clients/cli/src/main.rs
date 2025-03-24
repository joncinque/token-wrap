mod cli;
mod common;
mod config;
mod create_mint;
mod output;

use {
    crate::{cli::Cli, config::Config},
    clap::{CommandFactory, Parser},
};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type CommandResult = Result<String, Error>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    let matches = Cli::command().get_matches();
    let mut wallet_manager = None;

    let config = Config::new(cli.clone(), matches.clone(), &mut wallet_manager)?;

    solana_logger::setup_with_default("solana=info");

    let result = cli
        .command
        .execute(&config, &matches, &mut wallet_manager)
        .await?;
    println!("{}", result);

    Ok(())
}
