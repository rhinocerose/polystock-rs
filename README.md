![example workflow](https://github.com/rhinocerose/polystock-rs/actions/workflows/rust.yml/badge.svg)

# `polystock-rs`

Testing adapted from [this tutorial](https://rust-cli.github.io/book/tutorial/testing.html).

Reading environment variables:
```rust
use std::env;
use dotenv;

fn main() {
    dotenv::dotenv().expect("Failed to read .env file");
    println!("Finnhub token: {}",
         env::var("FINNHUB_TOKEN").expect("FINNHUB_TOKEN not found"));
}
```
