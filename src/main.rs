mod args;
mod api;

use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse arguments
    let url = args::Args::parse().url;

    // Run the asynchronous task and await its result
    let response = api::makeitshort(&url).await?;

    println!("Original url: {}", response.long);
    println!("Shorten url: https://gotiny.cc/{}", response.code);

    Ok(())
}