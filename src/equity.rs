pub struct Equity {
    name: String,
    ticker: String,
    price: f32,
    marketcap: i64,
    time_updated: i32,
    fivetwo_high: f32,
    fivetwo_low: f32,
    daily_volume: i32,
    owned_percent: f32,
    owned_shares: u32,
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
