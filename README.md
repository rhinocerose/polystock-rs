![example workflow](https://github.com/rhinocerose/polystock-rs/actions/workflows/rust.yml/badge.svg)

# `polystock-rs`

Testing adapted from [this tutorial](https://rust-cli.github.io/book/tutorial/testing.html).

Reading environment variables:
```rust
use std::env;
use dotenv::dotenv;
use finnhub_rs::client::Client;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let finnhub_key = "FINNHUB_TOKEN";
    let token = env::var(finnhub_key).expect("Key not present in .env file");
    let client = Client::new(token);

    let res = client.stock_symbol("US".to_string()).await.expect("Invalid response");
    println!("{:#?}", res);
}
```
