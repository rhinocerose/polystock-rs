use futures::{ future, StreamExt };
use yahoo_finance::Streamer;
// mod ticker_info;
mod ticker_info;

#[tokio::main]
async fn main() {
   let streamer = Streamer::new(vec!["AAPL", "QQQ", "^DJI", "^IXIC"]);

   let mut ticker = ticker_info::TickerInfo::new("GME");

   streamer.stream().await
      .for_each(|quote| {
         println!("At {} in session {:?}, {} is trading for ${}", quote.timestamp, quote.session, quote.symbol, quote.price);
         ticker.update_price(quote.price, quote.timestamp).expect("Unable to update ticker");
         future::ready(())
      })
      .await;

}
