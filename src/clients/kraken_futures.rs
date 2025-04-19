use log::{error, info};
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        credentials::Credentials,
        error::Error,
        rate_limit::{Cost, RateLimit},
        requests::{ReqError, RestClient},
        trade::Trade,
    },
    Timestamp,
};

use super::api::Api;

/// Limit of the request for the kraken futures, see [doc](https://docs.kraken.com/api/docs/guides/futures-rate-limits/)
const MAX_REQ_LIMIT: usize = 500;

/// The interval for resetting the counter, see [doc](https://docs.kraken.com/api/docs/guides/futures-rate-limits/)
const INTERVAL_LIMIT_SEC: u64 = 10;

#[derive(Debug, Serialize, Deserialize)]
struct TradeResponse {
    result: String,
    fills: Vec<Trade>,
    #[serde(rename = "serverTime")]
    server_time: String,
}

pub struct KrakenFutures {
    client: RestClient,
    rate_limiter: RateLimit,
}

impl KrakenFutures {
    pub fn new(creds: Credentials) -> Self {
        KrakenFutures {
            client: RestClient::new("https://futures.kraken.com".to_string(), creds),
            rate_limiter: RateLimit::new(MAX_REQ_LIMIT, INTERVAL_LIMIT_SEC),
        }
    }

    pub fn request_fills(
        &mut self,
        last_fill_time: Option<Timestamp>,
    ) -> Result<Vec<Trade>, Error> {
        if !self.rate_limiter.try_increment(Cost::FillsWithLastFillTime) {
            return Err(Error::Request(ReqError::OverTheReqLimit));
        }

        let res = {
            if last_fill_time.is_none() {
                self.client.make_request(
                    "/derivatives/api/v3/fills".to_string(),
                    "".to_string(),
                    "".to_string(),
                )
            } else {
                self.client.make_request(
                    "/derivatives/api/v3/fills".to_string(),
                    format!(
                        "lastFillTime={}",
                        last_fill_time.unwrap().format("%Y-%m-%dT%H:%M:%S%.3fZ")
                    ),
                    "".to_string(),
                )
            }
        };

        if let Ok(resp) = res {
            if resp.status().is_success() {
                let response = serde_json::from_str::<TradeResponse>(&resp.text().unwrap())?;
                return Ok(response.fills);
            } else {
                let status = resp.status();
                let body = resp.text().unwrap();
                error!("Request failed with status: {}\nBody: {}", status, body);
            }
        }
        Err(Error::Request(ReqError::SystemTimeFailure))
    }
}

impl Api for KrakenFutures {
    fn get_history_trades(
        &mut self,
        start: Timestamp,
        end: Timestamp,
    ) -> Result<Vec<Trade>, Error> {
        let mut trades = vec![];
        let mut last_date = end;

        while last_date > start {
            match self.request_fills(Some(last_date)) {
                Ok(req_trades) => {
                    let valid_trades = verify_trades(&start, &end, &req_trades);
                    if valid_trades.is_empty() {
                        // For this range there are no trades. We can end the loop.
                        break;
                    }

                    trades.extend(valid_trades.into_iter().cloned());
                    last_date = trades.last().unwrap().fill_time.parse()?;

                    info!("Last date of trade: {}", last_date);
                }
                Err(Error::Request(ReqError::OverTheReqLimit)) => {
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
                Err(err) => return Err(err),
            }
        }
        Ok(trades)
    }
}

fn verify_trades<'a>(start: &Timestamp, end: &Timestamp, trades: &'a Vec<Trade>) -> Vec<&'a Trade> {
    trades
        .iter()
        .filter(|&t| {
            if let Ok(timestamp) = t.fill_time.parse::<Timestamp>() {
                return timestamp >= *start && timestamp <= *end;
            }
            false
        })
        .collect::<Vec<&Trade>>()
}
