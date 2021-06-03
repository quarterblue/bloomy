#[derive(Debug, Clone)]
pub struct Equity {
    pub name: String,
    pub ticker: String,
    pub price: f32,
    pub marketcap: i64,
    pub time_updated: i32,
    pub fivetwo_high: f32,
    pub fivetwo_low: f32,
    pub daily_volume: i32,
    pub owned_percent: f32,
    pub owned_shares: u32,
}

impl Equity {
    pub fn new(
        name: String,
        ticker: String,
        price: f32,
        marketcap: i64,
        time_updated: i32,
        fivetwo_high: f32,
        fivetwo_low: f32,
        daily_volume: i32,
    ) -> Self {
        Equity {
            name,
            ticker,
            price,
            marketcap,
            time_updated,
            fivetwo_high,
            fivetwo_low,
            daily_volume,
            owned_percent: 0.0,
            owned_shares: 0,
        }
    }

    pub fn short_info() {}

    pub fn long_info() {}
}
