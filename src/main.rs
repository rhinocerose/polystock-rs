use futures::{ future, StreamExt };
use yahoo_finance::Streamer;
use polystock_rs;

#[tokio::main]
async fn main() {
   let streamer = Streamer::new(vec!["AAPL", "QQQ", "^DJI", "^IXIC"]);

   let ticker = TickerInfo::new("GME");
   
   streamer.stream().await
      .for_each(|quote| {
         println!("At {} in session {:?}, {} is trading for ${}", quote.timestamp, quote.session, quote.symbol, quote.price);
         ticker.update_price(quote.price, quote.timestamp);
         future::ready(())
      })
      .await;
   
}

#[cfg(test)]
mod tests {
    use super::*;
}
