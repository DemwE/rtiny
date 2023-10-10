use clap::{command, Parser};

#[derive(Debug, Default, Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(author = env!("CARGO_PKG_AUTHORS"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = env!("CARGO_PKG_DESCRIPTION"))]
#[command(
help_template = "{name} {version} {author-section} {about-with-newline} \n {all-args}"
)]
pub struct Args {
    /// Url to short
    #[clap(required = true)]
    #[clap(short, long)]
    pub url: Option<String>,
}