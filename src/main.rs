use futures::{ future, StreamExt };
use yahoo_finance::Streamer;

#[tokio::main]
async fn main() {
   let streamer = Streamer::new(vec!["AAPL", "QQQ", "^DJI", "^IXIC"]);

   streamer.stream().await
      .for_each(|quote| {
         println!("At {} in session {}, {} is trading for ${}", quote.timestamp, quote.session, quote.symbol, quote.price);
         future::ready(())
      })
      .await;
}

#[cfg(test)]
mod tests {
    use super::*;
}
