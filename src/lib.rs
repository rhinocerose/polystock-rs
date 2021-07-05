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
}


#[cfg(test)]
mod tests {
    use super::*;
    
    fn make_struct(ticker: String) -> TickerInfo<'static> {
        TickerInfo::new(ticker)
    }
    
    #[test]
    fn test_struct_creation() {
        let mut temp = make_struct("GME");
        assert_eq!(temp.ticker, "GME");
        assert_eq!(temp.price, 0.0);
    }
}
