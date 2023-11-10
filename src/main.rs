mod args;
mod api;

use clap::Parser;
use colorful::{Color, Colorful};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse arguments
    let url = args::Args::parse().url;
    let custom = args::Args::parse().custom;

    // Run the asynchronous task and await its result
    let response = api::request_api(&url,&custom).await?;

    println!("Original url: {}", response.long);
    if url!= Option::from(response.short.clone()) {
        println!("{}", "Custom url is used!".color(Color::Yellow));
    }
    println!("Shorten url: https://1pt.co/{}", response.short);

    Ok(())
}