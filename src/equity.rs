use std::default;

#[derive(Default, Debug, Clone)]
pub struct Quote {
    pub ticker: String,
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub price: f32,
    pub volume: u64,
    pub ltd: f32,
    pub previous_close: f32,
    pub change: f32,
    pub change_percent: f32,
}

#[derive(Default, Debug, Clone)]
pub struct Equity {
    pub name: Option<String>,
    pub ticker: String,
    pub marketcap: Option<i32>,
    pub time_updated: Option<i32>,
    pub fivetwo_high: Option<f32>,
    pub fivetwo_low: Option<f32>,
    pub daily_volume: Option<i32>,
    pub owned_percent: Option<f32>,
    pub owned_shares: Option<u32>,
    pub global_quote: Option<Quote>,
}

impl Quote {
    fn new(
        ticker: String,
        open: f32,
        high: f32,
        low: f32,
        price: f32,
        volume: u64,
        ltd: f32,
        previous_close: f32,
        change: f32,
        change_percent: f32,
    ) -> Self {
        Quote {
            ticker,
            open,
            high,
            low,
            price,
            volume,
            ltd,
            previous_close,
            change,
            change_percent,
        }
    }
}

impl Equity {
    pub fn new(ticker: String) -> Self {
        Equity {
            name: None,
            ticker,
            marketcap: None,
            time_updated: None,
            fivetwo_high: None,
            fivetwo_low: None,
            daily_volume: None,
            owned_percent: None,
            owned_shares: None,
            global_quote: None,
        }
    }

    pub fn short_info() {}

    pub fn long_info() {}
}
