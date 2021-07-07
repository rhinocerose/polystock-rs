![example workflow](https://github.com/rhinocerose/polystock-rs/actions/workflows/rust.yml/badge.svg)

# `polystock-rs`

Testing adapted from [this tutorial](https://rust-cli.github.io/book/tutorial/testing.html).

## Passing Variables

Required parameters:
- `-t` or `--tickers`: An array of space separated, quote-wrapped tickers

## CLI Manifest Add

```
cargo install cargo-edit
cargo add {CRATE}
```

## Reading environment variables:
```rust
use std::env;
use dotenv::dotenv;
use finnhub_rs::client::Client;

const FINNHUB_KEY: &str = "FINNHUB_TOKEN";

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var(FINNHUB_KEY).expect("Key not present in .env file");
    let client = Client::new(token);

    let res = client.stock_symbol("US".to_string()).await.expect("Invalid response");
    println!("{:#?}", res);
}
```
