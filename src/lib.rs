use chrono::prelude::*;

struct TickerInfo {
    ticker: String,
    price: f32,
    last_updated: DateTime<Local>,
}
    
impl TickerInfo {
    fn new(
        ticker: String
    ) -> TickerInfo {
        TickerInfo {
            ticker,
            price: 0.0,
            last_updated: Local::now()
        }
    }
