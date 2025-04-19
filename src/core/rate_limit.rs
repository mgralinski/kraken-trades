use std::time::Instant;

/// Cost of the request, base on the
/// https://docs.kraken.com/api/docs/guides/futures-rate-limits
pub enum Cost {
    FillsWithLastFillTime,
}

impl Cost {
    pub fn value(&self) -> usize {
        match &self {
            Cost::FillsWithLastFillTime => 25,
        }
    }
}

pub struct RateLimit {
    start_time: Instant,
    total_cost: usize,
    max_limit: usize,
    limit_interval_sec: u64,
}

impl RateLimit {
    pub fn new(max_limit: usize, limit_interval_sec: u64) -> Self {
        RateLimit {
            start_time: Instant::now(),
            total_cost: 0,
            max_limit,
            limit_interval_sec,
        }
    }

    pub fn try_increment(&mut self, request: Cost) -> bool {
        if self.start_time.elapsed().as_secs() > self.limit_interval_sec {
            self.start_time = Instant::now();
            self.total_cost = 0;
        }

        if self.total_cost + request.value() < self.max_limit {
            self.total_cost += request.value();
            return true;
        }
        false
    }
}
