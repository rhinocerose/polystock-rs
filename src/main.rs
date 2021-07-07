use futures::{ future, StreamExt };
use yahoo_finance::Streamer;
use structopt::StructOpt;
// mod ticker_info;
mod ticker_info;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "polystock-rs",
    about = "Display ticker information in polybar"
)]
struct Opt {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short, long)]
    debug: bool,

    /// Set can interface
    #[structopt(short = "t", long = "tickers", help = "Comma separated list of quoted tickers")]
    tickers: Vec<String>,
}

#[tokio::main]
async fn main() {
    // let streamer = Streamer::new(vec!["AAPL", "QQQ", "^DJI", "^IXIC"]);

    let opt = Opt::from_args();
    let streamer = Streamer::new(opt.tickers.as_str());
    let mut ticker = ticker_info::TickerInfo::new("^N225");

    streamer.stream().await
        .for_each(|quote| {
            ticker.process_quote(quote).expect("Unable to process quote");
            println!("{:?}", ticker.return_ticker_values().unwrap());
            future::ready(())
        })
    .await;

}

#[cfg(test)]
mod tests {
    use super::*;
}
