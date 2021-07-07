use yahoo_finance::Quote;
use yahoo_finance;
// use thiserror::Error;
use std::error::Error;

static POSITIVE_PERCENTAGE_COLOR: &str = "#0f5c15";
static NEGATIVE_PERCENTAGE_COLOR: &str = "#fc0511";

#[derive(Clone)]
pub struct TickerInfo {
    ticker: String,
    price: f64,
    percent_change: f32,
    timestamp: i64,
}

impl TickerInfo {
    pub fn new(
        ticker: &str
    ) -> TickerInfo {
        TickerInfo {
            ticker: ticker.to_string(),
            price: 1.0,
            percent_change: 0.0,
            timestamp: 0
        }
    }

    pub fn process_quote(&mut self,
        quote: Quote
    ) -> Result<(), Box<dyn Error>> {
        println!("At {} in session {:?}, {} is trading for ${}",
                 quote.timestamp, quote.session, quote.symbol, quote.price);
        Ok(self.update_price(quote.price, quote.timestamp).expect("Unable to update ticker"))
    }

    pub fn update_price(&mut self,
        new_value: f64,
        timestamp: i64
    ) -> Result<(), Box<dyn Error>> {
        let old_value = self.price;
        self.price = new_value;
        self.percent_change = self.calculate_percent_change(new_value, old_value)
                                  .expect("Could not calculate percent change");
        self.timestamp= timestamp;
        self.bar_print();
        Ok(())
    }

    pub fn return_ticker_values(&mut self,
    ) -> Result<(String, f64, f32, i64), Box<dyn Error>> {
        let temp = (self.ticker.clone(), self.price, self.percent_change, self.timestamp);
        Ok(temp)
    }

    fn calculate_percent_change(&mut self,
        new_value: f64,
        old_value: f64
    ) -> Result<f32, Box<dyn Error>> {
        let temp: f32 = (((old_value - new_value) / old_value) * 100.0) as f32;
        Ok(temp)
    }

    fn percentage_polarity(&self) -> String {
        let beginning: &str = "%{F";
        let change: &str = &self.percent_change.to_string();
        let ending: &str = "%%{F-}";
        let par: &str = "}";
        if self.percent_change < 0.0 {
            let percentage = NEGATIVE_PERCENTAGE_COLOR;
            let arr = [beginning, percentage, par, change, ending].join("");
            arr
        } else {
            let percentage = POSITIVE_PERCENTAGE_COLOR;
            let arr = [beginning, percentage, par, change, ending].join("");
            arr
        }
    }

    fn bar_print(&mut self) {
        let values = self.return_ticker_values().unwrap();
        // let mut print: String = self.return_ticker_values().unwrap().0;
        println!("{} {} {}", values.0, values.1, self.percentage_polarity());
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    fn make_test_struct(ticker: &str) -> TickerInfo {
        TickerInfo::new(ticker)
    }

    fn make_test_quote() -> Quote {
        let temp = Quote {
            symbol: "GME".to_string(),
            timestamp: 10001000,
            session: yahoo_finance::TradingSession::Regular,
            price: 211.0,
            volume: 200
        };
        temp
    }

    #[test]
    fn test_struct_creation() {
        let test_struct = make_test_struct("GME");
        assert_eq!(test_struct.ticker, "GME");
        assert_eq!(test_struct.price, 1.0);
        assert_eq!(test_struct.percent_change, 0.0);
        assert_eq!(test_struct.timestamp, 0);
    }

    #[test]
    fn test_price_update() {
        // let mut test_struct = make_test_struct("GME");
        let mut test_struct = TickerInfo{
            ticker: "GME".to_string(),
            price: 1.0,
            percent_change: 0.0,
            timestamp: 0
        };
        test_struct.update_price(32.0, 55).unwrap();
        assert_approx_eq!(test_struct.price, 32.0);
        assert_eq!(test_struct.timestamp, 55);
        assert_approx_eq!(test_struct.percent_change, -3100.0);
    }

    #[test]
    fn test_return_ticker_values() {
        let mut test_struct = TickerInfo{
            ticker: "GME".to_string(),
            price: 1.0,
            percent_change: 0.0,
            timestamp: 0
        };
        assert_eq!(test_struct.return_ticker_values().unwrap(), ("GME".to_string(), 1.0, 0.0, 0));
        let quote = make_test_quote();
        test_struct.process_quote(quote).unwrap();
        assert_eq!(test_struct.return_ticker_values().unwrap(), ("GME".to_string(), 211.0, -21000.0, 10001000));
    }

    #[test]
    fn test_process_quote() {
        let mut test_struct = TickerInfo{
            ticker: "GME".to_string(),
            price: 1.0,
            percent_change: 0.0,
            timestamp: 0
        };
        let quote = make_test_quote();
        test_struct.process_quote(quote).unwrap();
        assert_approx_eq!(test_struct.price, 211.0);
        assert_eq!(test_struct.timestamp, 10001000);
        assert_approx_eq!(test_struct.percent_change, -21000.0);
        assert_eq!(test_struct.return_ticker_values().unwrap(), ("GME".to_string(), 211.0, -21000.0, 10001000));
    }
}
