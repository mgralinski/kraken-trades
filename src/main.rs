use clap::Parser;
use cli::Args;
use env_logger::Env;
use kraken_trades::{
    clients::{api::Api, kraken_futures::KrakenFutures},
    core::{credentials::Credentials, error::Error},
    write_to_csv,
};
use log::{error, info};

mod cli;

fn main() -> Result<(), Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let args = Args::parse();
    let creds = Credentials::load(&args.kraken_credentials);
    if creds.is_err() {
        error!(
            "Failed to load credentials, error: {:?}",
            creds.unwrap_err()
        );
        return Err(Error::NoCredentials);
    }

    let (start_date, end_date) = args.period.range()?;

    let mut client: Box<dyn Api> = Box::new(KrakenFutures::new(creds.unwrap()));

    match client.get_history_trades(start_date, end_date) {
        Ok(trades) => {
            info!("We received trades: {}", trades.len());
            write_to_csv(&trades, args.csv_output.into())?;
        }
        Err(err) => error!("Failed to get trades, error: {:?}", err),
    }

    Ok(())
}
