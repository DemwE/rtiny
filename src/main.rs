mod args;
mod api;

use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse arguments
    let url = args::Args::parse().url;

    // Run the asynchronous task and await its result
    let response = api::request_api(&url).await?;

    println!("Original url: {}", response.long);
    println!("Shorten url: https://1pt.co/{}", response.short);

    Ok(())
}