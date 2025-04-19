use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub fill_id: String,
    pub symbol: String,
    pub side: String,
    pub order_id: String,
    pub size: Decimal,
    pub price: Decimal,
    #[serde(rename = "fillTime")]
    pub fill_time: String,
    #[serde(rename = "fillType")]
    pub fill_type: String,
}
