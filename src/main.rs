use std::env;
use dotenv::dotenv;
use tokio_compat_02::FutureExt;


use finnhub_rs;
use futures::{ future, StreamExt };
use yahoo_finance::Streamer;
use structopt::StructOpt;

mod ticker_info;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "polystock-rs",
    about = "Display ticker information in polybar"
)]
struct Opt {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short,
                long)]
    debug: bool,

    /// Set can interface
    #[structopt(short = "t",
                long = "tickers",
                help = "Comma separated list of quoted tickers",
                default_value = "GME",
                required = true,
                min_values = 1,
                )]
    tickers: Vec<String>,
}

const FINNHUB_TOKEN: &str = "FINNHUB_TOKEN";

#[tokio::main]
async fn main() {

    let opt = Opt::from_args();
    let ticker_list = opt.tickers
                         .iter().map(|x| x.as_str().clone())
                         .collect::<Vec<_>>();
    let spare = ticker_list.clone();
    let streamer = Streamer::new(ticker_list);

    dotenv().ok();
    let finnhub_token = env::var(FINNHUB_TOKEN).expect("Key not present in .env file");
    let finnhub_client = finnhub_rs::client::Client::new(finnhub_token);
    // let res = finnhub_client.stock_symbol("US".to_string())
    //                         .await
    //                         .expect("Invalid response");
    let res = finnhub_client.symbol_lookup(spare[0].to_string())
                            .await
                            .expect("Invalid response");
    for i in res.result {
        println!("{:?}", i.description);

    }
    // println!("{:#?}", res);

    let mut ticker = ticker_info::TickerInfo::new("^N225");

    streamer.stream().compat().await
        .for_each(|quote| {
            ticker.process_quote(quote)
                  .expect("Unable to process quote");
            // println!("{:?}", ticker.return_ticker_values().unwrap());
            future::ready(())
        })
    .await;

}

#[cfg(test)]
mod tests {
    use super::*;
}
