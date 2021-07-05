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
    
    fn update_price(&mut self,
        new_value: f32
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.price = new_value;
        self.last_updated: Local::now();
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    fn make_struct(ticker: String) -> TickerInfo {
        TickerInfo::new(ticker)
    }
    
    #[test]
    fn test_struct_creation() {
        let mut temp = make_struct("GME");
        assert_eq!(temp.ticker, "GME");
        assert_eq!(temp.price, 0.0);
    }
    
    #[test]
    fn test_price_update() {
        let mut temp = make_struct("GME");
        temp.update_price(32.0);
        assert_eq!(temp.price, 32.0);
    }
    
}
