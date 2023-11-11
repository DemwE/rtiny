mod args;
mod api;

use clap::Parser;
use colorful::{Color, Colorful};
use log::{LevelFilter, info, debug};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    if args::Args::parse().debug {
        env_logger::builder().filter(None, LevelFilter::Debug).init();
        info!("Debug mode is enabled");
    }

    // Parse arguments
    let args = args::Args::parse();
    //Debug arguments
    debug!("{:#?}", args);

    // Run the asynchronous task and await its result
    let response = api::request_api(args.url.as_ref(),args.custom.as_ref()).await?;
    //Print the all result in debug mode
    debug!("{:#?}", response);

    println!("Original url: {}", response.long);

    // Check if custom url is used and is accepted by the server
    if args.custom.as_ref().is_some() && response.short != args.custom.unwrap() {
        println!("{}", "Custom url is used!".color(Color::Yellow));
    }

    println!("Shorten url: https://1pt.co/{}", response.short);

    Ok(())
}