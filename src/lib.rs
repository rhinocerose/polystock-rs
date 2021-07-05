struct TickerInfo {
    ticker: String,
    price: f32,
    last_updated: i64,
}
    
impl TickerInfo {
    fn new(
        ticker: &str
    ) -> TickerInfo {
        TickerInfo {
            ticker: tisker.to_string(),
            price: 0.0,
            percent_change: f32,
            last_updated: 0
        }
    }
    
    fn update_price(&mut self,
        new_value: f32,
        timestamp: i64
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.price = new_value;
        self.last_updated= timestamp;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    fn make_struct(ticker: &str) -> TickerInfo {
        TickerInfo::new(ticker.to_string())
    }
    
    #[test]
    fn test_struct_creation() {
        let temp = make_struct("GME");
        assert_eq!(temp.ticker, "GME");
        assert_eq!(temp.price, 0.0);
        assert_eq!(temp.timestamp, 0);
    }
    
    #[test]
    fn test_price_update() {
        let mut temp = make_struct("GME");
        temp.update_price(32.0, 55).expect("Unable to update price");
        assert_eq!(temp.price, 32.0);
        assert_eq!(temp.timestamp, 55);
    }
    
}
