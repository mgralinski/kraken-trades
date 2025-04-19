use core::{error::Error, trade::Trade};
use std::{fs::File, path::PathBuf};

use chrono::{DateTime, Utc};

pub mod clients;
pub mod core;

pub type Timestamp = DateTime<Utc>;

pub fn write_to_csv(trades: &[Trade], path: PathBuf) -> Result<(), Error> {
    let file = File::create(path)?;
    let mut writer = csv::Writer::from_writer(file);
    for t in trades {
        writer.serialize(t)?;
    }
    writer.flush()?;
    Ok(())
}
