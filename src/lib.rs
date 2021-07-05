use assert_approx_eq::assert_approx_eq;

struct TickerInfo {
    ticker: String,
    price: f64,
    percent_change: f32,
    timestamp: i64,
}
    
impl TickerInfo {
    fn new(
        ticker: &str
    ) -> TickerInfo {
        TickerInfo {
            ticker: ticker.to_string(),
            price: 1.0,
            percent_change: 0.0,
            timestamp: 0
        }
    }
    
    fn update_price(&mut self,
        new_value: f64,
        timestamp: i64
    ) -> Result<(), Box<dyn std::error::Error>> {
        let old_value = self.price;
        self.price = new_value;
        self.percent_change = self.calculate_percent_change(new_value, old_value)
                                  .expect("Could not calculate percent change");
        self.timestamp= timestamp;
        Ok(())
    }
    
    fn calculate_percent_change(&mut self,
        new_value: f64,
        old_value: f64
    ) -> Result<f32, Box<dyn std::error::Error>> {
        let temp: f32 = (((old_value - new_value) / old_value) * 100.0) as f32;
        Ok(temp)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    fn make_struct(ticker: &str) -> TickerInfo {
        TickerInfo::new(ticker)
    }
    
    #[test]
    fn test_struct_creation() {
        let temp = make_struct("GME");
        assert_eq!(temp.ticker, "GME");
        assert_eq!(temp.price, 1.0);
        assert_eq!(temp.percent_change, 0.0);
        assert_eq!(temp.timestamp, 0);
    }
    
    #[test]
    fn test_price_update() {
        let mut temp = make_struct("GME");
        temp.update_price(32.0, 55).expect("Unable to update price");
        assert_eq!(temp.price, 32.0);
        assert_eq!(temp.timestamp, 55);
        assert_approx_eq!(temp.percent_change, -31.0);
    }
    
}
