use chrono::{DateTime, Datelike, Days, Months, TimeZone, Utc};
use clap::{Parser, ValueEnum};
use kraken_trades::core::error::Error;
use std::path::PathBuf;

#[derive(Debug, Clone, ValueEnum)]
pub enum RequestPeriod {
    ThisWeek,
    ThisMonth,
    ThisYear,
    LastDay,
    LastYear,
}

impl RequestPeriod {
    pub fn range(&self) -> Result<(DateTime<Utc>, DateTime<Utc>), Error> {
        let now = Utc::now();
        match &self {
            RequestPeriod::ThisWeek => {
                let days = now.weekday().days_since(chrono::Weekday::Mon);
                let start = now
                    .checked_sub_days(Days::new(days as u64))
                    .unwrap()
                    .date_naive()
                    .and_hms_opt(0, 0, 0)
                    .unwrap();
                let end = now.date_naive().and_hms_opt(23, 59, 59).unwrap();
                Ok((start.and_utc(), end.and_utc()))
            }
            RequestPeriod::ThisMonth => {
                let start = now
                    .checked_sub_months(Months::new(1))
                    .unwrap()
                    .date_naive()
                    .with_day(1)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap();
                let end = now.date_naive().and_hms_opt(23, 59, 59).unwrap();
                Ok((start.and_utc(), end.and_utc()))
            }
            RequestPeriod::ThisYear => {
                let start = Utc.with_ymd_and_hms(now.year(), 1, 1, 0, 0, 0).unwrap();
                let end = now.date_naive().and_hms_opt(23, 59, 59).unwrap();
                Ok((start, end.and_utc()))
            }
            RequestPeriod::LastDay => {
                let start = now
                    .checked_sub_days(Days::new(1))
                    .unwrap()
                    .date_naive()
                    .and_hms_opt(0, 0, 0)
                    .unwrap();
                let end = start.date().and_hms_opt(23, 59, 59).unwrap();
                Ok((start.and_utc(), end.and_utc()))
            }
            RequestPeriod::LastYear => {
                let start = Utc.with_ymd_and_hms(now.year() - 1, 1, 1, 0, 0, 0).unwrap();
                let end = Utc
                    .with_ymd_and_hms(now.year() - 1, 12, 31, 23, 59, 59)
                    .unwrap();
                Ok((start, end))
            }
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to the Kraken credentials file
    #[arg(short, long)]
    pub kraken_credentials: PathBuf,

    /// Period request
    #[arg(short, long, value_enum)]
    pub period: RequestPeriod,

    // Path of the csv output file
    #[arg(short, long)]
    pub csv_output: PathBuf,
}
