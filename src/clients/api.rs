use crate::{
    core::{error::Error, trade::Trade},
    Timestamp,
};

pub trait Api {
    fn get_history_trades(&mut self, start: Timestamp, end: Timestamp)
        -> Result<Vec<Trade>, Error>;
}
